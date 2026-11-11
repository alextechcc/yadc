//! `#[derive(Dynamic)]`: a struct of yours holding any of several static
//! types - each mapped field by field - with operators and methods lifted
//! over the fields.
//!
//! A mapping `Static => { field: source, .. }` fills every field of the
//! struct from a const parameter or a constant expression (a *fact*:
//! recorded, checked on the way back) or from a field path of the static
//! value (*payload*: moved in and out). Per mapping it writes std
//! `From<Static>` and the way back, `TryFrom<Self> for Static`.
//! `#[dynamic(impl(..))]` lists the lifted items, each with its output
//! stated: every field by its own operator, a `#[dynamic(same)]` field by
//! equality.

use std::collections::BTreeMap;

use proc_macro2::{Span, TokenStream as TokenStream2, TokenTree};
use quote::{ToTokens, format_ident, quote, quote_spanned};
use syn::{
    Data, DeriveInput, Expr, Fields, GenericArgument, GenericParam, Ident, Member, Path,
    PathArguments, Token, Type, TypeParam,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    spanned::Spanned,
};

use crate::krate;

// ===== Parsing: mappings ======================================================

/// Where a field of the dynamic type comes from, in one mapping.
#[derive(Clone)]
enum Source {
    /// A const parameter of the static type: a fact.
    Param(Ident),
    /// A constant expression: a fact. `generic` when it names a declared
    /// parameter (`C::CODE`), so its value is known per member.
    Const { expr: Box<Expr>, generic: bool },
    /// A field path of the static value, `.0`, `.since.0`, or `self` (empty):
    /// payload.
    Path(Vec<Member>),
}

impl Source {
    fn is_fact(&self) -> bool {
        !matches!(self, Source::Path(_))
    }
}

/// `since: Time`, or `since: Span { start: Time }`: the type of another
/// field of the static value - one a deep path passes through (Rust can't
/// build a struct without naming it), or one filled with `Default` (a
/// `PhantomData`).
struct Decl {
    member: Member,
    ty: Path,
    children: Vec<Decl>,
}

impl Parse for Decl {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let member: Member = input.parse()?;
        input.parse::<Token![:]>()?;
        let ty: syn::TypePath = input.parse()?;
        let children = decls(input)?;
        Ok(Decl {
            member,
            ty: expr_path(ty.path),
            children,
        })
    }
}

/// An optional `{ field: Type, .. }`.
fn decls(input: ParseStream<'_>) -> syn::Result<Vec<Decl>> {
    if !input.peek(syn::token::Brace) {
        return Ok(Vec::new());
    }
    let inner;
    syn::braced!(inner in input);
    let list: Punctuated<Decl, Token![,]> = Punctuated::parse_terminated(&inner)?;
    Ok(list.into_iter().collect())
}

/// `Foo<A>` as an expression path, `Foo::<A>` (also valid as a type).
fn expr_path(mut p: Path) -> Path {
    for seg in &mut p.segments {
        if let PathArguments::AngleBracketed(a) = &mut seg.arguments {
            a.colon2_token = Some(Default::default());
        }
    }
    p
}

/// A parameter a mapping declares: `const D: Dim` or `C: Currency`.
enum Param {
    Const(Ident, Type),
    Type(TypeParam),
}

impl Param {
    fn name(&self) -> &Ident {
        match self {
            Param::Const(n, _) => n,
            Param::Type(t) => &t.ident,
        }
    }
    fn decl(&self) -> TokenStream2 {
        match self {
            Param::Const(n, t) => quote! { const #n: #t },
            Param::Type(t) => quote! { #t },
        }
    }
}

/// One `Static => { field: source, .. }`.
struct Mapping {
    path: Path,
    /// The declared parameters, in order.
    params: Vec<Param>,
    /// The static type's arguments, parameters by name.
    args: Vec<TokenStream2>,
    decls: Vec<Decl>,
    fills: Vec<(Member, Source)>,
    /// `shared`: it may hold the same facts as another mapping - the way
    /// back gives either.
    shared: bool,
    span: Span,
}

impl Mapping {
    /// `Qty<D>`, for type position.
    fn ty(&self) -> TokenStream2 {
        let (p, a) = (&self.path, &self.args);
        if a.is_empty() {
            quote! { #p }
        } else {
            quote! { #p<#(#a),*> }
        }
    }
    /// `Qty::<D>`, for a struct expression.
    fn expr(&self) -> TokenStream2 {
        let (p, a) = (&self.path, &self.args);
        if a.is_empty() {
            quote! { #p }
        } else {
            quote! { #p::<#(#a),*> }
        }
    }
    /// `const D: Dim`, `C: Currency`, for an impl's parameters.
    fn param_decls(&self) -> Vec<TokenStream2> {
        self.params.iter().map(Param::decl).collect()
    }
    /// How messages name it: `Qty<D>`.
    fn name(&self) -> String {
        tidy(&self.ty().to_string())
    }
    fn source(&self, m: &Member) -> &Source {
        &self.fills.iter().find(|(f, _)| f == m).unwrap().1
    }
}

/// Token spacing removed where Rust wouldn't write it: `Qty < D >` -> `Qty<D>`.
fn tidy(s: &str) -> String {
    s.replace(" < ", "<")
        .replace(" <", "<")
        .replace("< ", "<")
        .replace(" >", ">")
        .replace(" :: ", "::")
        .replace(" ,", ",")
}

/// Whether `tokens` contain one of `names` as an identifier.
fn mentions(tokens: TokenStream2, names: &[&Ident]) -> bool {
    tokens.into_iter().any(|t| match t {
        TokenTree::Ident(i) => names.contains(&&i),
        TokenTree::Group(g) => mentions(g.stream(), names),
        _ => false,
    })
}

impl Parse for Mapping {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let span = input.span();
        // `shared Static => ..`: the static type may hold another's facts.
        let shared = {
            let f = input.fork();
            match f.parse::<Ident>() {
                Ok(id)
                    if id == "shared"
                        && !(f.peek(Token![=>])
                            || f.peek(Token![<])
                            || f.peek(Token![::])
                            || f.peek(syn::token::Brace)) =>
                {
                    input.parse::<Ident>()?;
                    true
                }
                _ => false,
            }
        };
        let path = Path::parse_mod_style(input)?;
        let (mut params, mut args) = (Vec::new(), Vec::new());
        if input.peek(Token![<]) {
            input.parse::<Token![<]>()?;
            while !input.peek(Token![>]) {
                if input.peek(Token![const]) {
                    // `const D: Dim` declares a const parameter.
                    input.parse::<Token![const]>()?;
                    let name: Ident = input.parse()?;
                    input.parse::<Token![:]>()?;
                    let ty: Type = input.parse()?;
                    args.push(quote! { #name });
                    params.push(Param::Const(name, ty));
                } else if input.peek(Ident) && input.peek2(Token![:]) && !input.peek2(Token![::]) {
                    // `C: Currency` declares a type parameter.
                    let tp: TypeParam = input.parse()?;
                    let name = &tp.ident;
                    args.push(quote! { #name });
                    params.push(Param::Type(tp));
                } else {
                    // Anything else is passed as written.
                    let a: GenericArgument = input.parse()?;
                    args.push(quote! { #a });
                }
                if !input.peek(Token![,]) {
                    break;
                }
                input.parse::<Token![,]>()?;
            }
            input.parse::<Token![>]>()?;
        }
        let decls = decls(input)?;
        input.parse::<Token![=>]>().map_err(|e| {
            syn::Error::new(
                e.span(),
                "expected `=>`: a mapping reads `Static => { field: source, .. }`, e.g. \
                 `Qty<const D: Dim> => { dim: D, value: .0 }`",
            )
        })?;
        let inner;
        syn::braced!(inner in input);
        let mut fills = Vec::new();
        while !inner.is_empty() {
            let m: Member = inner.parse()?;
            inner.parse::<Token![:]>()?;
            let src = parse_source(&inner, &params)?;
            fills.push((m, src));
            if inner.is_empty() {
                break;
            }
            inner.parse::<Token![,]>()?;
        }
        Ok(Mapping {
            path,
            params,
            args,
            decls,
            fills,
            shared,
            span,
        })
    }
}

/// `.a.0` or `self` (a field path), a declared const parameter, or a
/// constant expression.
fn parse_source(input: ParseStream<'_>, params: &[Param]) -> syn::Result<Source> {
    if input.peek(Token![self])
        && (input.peek2(Token![,]) || {
            let f = input.fork();
            f.parse::<Token![self]>()?;
            f.is_empty()
        })
    {
        input.parse::<Token![self]>()?;
        return Ok(Source::Path(Vec::new()));
    }
    if input.peek(Token![.]) {
        let mut segs = Vec::new();
        while input.peek(Token![.]) {
            input.parse::<Token![.]>()?;
            if input.peek(syn::LitFloat) {
                // `.0.1` lexes as `.` and the float `0.1`.
                let f: syn::LitFloat = input.parse()?;
                for part in f.base10_digits().split('.') {
                    let i: u32 = part
                        .parse()
                        .map_err(|_| syn::Error::new(f.span(), "expected a field index"))?;
                    segs.push(Member::Unnamed(syn::Index {
                        index: i,
                        span: f.span(),
                    }));
                }
            } else {
                segs.push(input.parse()?);
            }
        }
        return Ok(Source::Path(segs));
    }
    // A lone identifier naming a declared parameter is that parameter.
    let fork = input.fork();
    if let Ok(id) = fork.parse::<Ident>()
        && (fork.is_empty() || fork.peek(Token![,]))
    {
        match params.iter().find(|p| *p.name() == id) {
            Some(Param::Const(..)) => {
                input.parse::<Ident>()?;
                return Ok(Source::Param(id));
            }
            Some(Param::Type(_)) => {
                return Err(syn::Error::new_spanned(
                    id,
                    "a type parameter is not a value: a fact comes from one of its constants, \
                     `C::CODE`",
                ));
            }
            None => {}
        }
    }
    let expr: Expr = input.parse()?;
    let names: Vec<&Ident> = params.iter().map(Param::name).collect();
    let generic = mentions(expr.to_token_stream(), &names);
    Ok(Source::Const {
        expr: Box::new(expr),
        generic,
    })
}

// ===== Parsing: lifted items ==================================================

/// What a lifted item returns.
#[derive(Clone, Copy, PartialEq)]
enum Out {
    /// `-> Self`: each field's result is its new value.
    Plain,
    /// `-> Option<Self>`: each field's result through `Into<Option<_>>`.
    Partial,
}

/// `Self` or `Option<Self>`, as written.
fn out_of(t: &Type) -> Option<Out> {
    let Type::Path(tp) = t else { return None };
    if tp.qself.is_some() {
        return None;
    }
    if tp.path.is_ident("Self") {
        return Some(Out::Plain);
    }
    let last = tp.path.segments.last()?;
    if last.ident != "Option" {
        return None;
    }
    let PathArguments::AngleBracketed(a) = &last.arguments else {
        return None;
    };
    match a.args.iter().collect::<Vec<_>>().as_slice() {
        [GenericArgument::Type(Type::Path(inner))] if inner.path.is_ident("Self") => {
            Some(Out::Partial)
        }
        _ => None,
    }
}

/// A method's receiver.
#[derive(Clone, Copy, PartialEq)]
enum Recv {
    Value,
    Ref,
    Mut,
}

/// A lifted method or a getter.
struct Method {
    /// The trait, as written (with associated-type bindings).
    tr: Path,
    method: Ident,
    recv: Recv,
    args: Vec<(Ident, Type)>,
    ret: Type,
    body: Body,
}

enum Body {
    /// Lifted over every field.
    Lift { out: Out },
    /// `= self.field`.
    Getter(Member),
}

/// One entry of `impl(..)`.
enum Item {
    TryFrom,
    PartialOrd,
    /// `Mul -> Self`, `Mul<K> -> Option<Self>`, `Neg -> Self`.
    Op {
        op: Ident,
        rhs: Option<Box<Type>>,
        out: Out,
    },
    Method(Box<Method>),
}

/// The binary operators a lift takes, with their token and method.
const BINARY: [(&str, &str, &str); 5] = [
    ("Mul", "*", "mul"),
    ("Div", "/", "div"),
    ("Add", "+", "add"),
    ("Sub", "-", "sub"),
    ("Rem", "%", "rem"),
];

const ITEMS_HELP: &str = "`impl(..)` takes `TryFrom`, `PartialOrd`, an operator with its output \
     (`Mul -> Self`, `Add -> Option<Self>`, `Mul<K> -> Self`, `Neg -> Self`; `Mul`, `Div`, `Add`, \
     `Sub`, `Rem`, `Neg`), or a trait method (`path::Trait::method(self, n: i8) -> Option<Self>`, \
     a getter `path::Trait::get(&self) -> T = self.field`)";

/// `-> Type`, required.
fn arrow(input: ParseStream<'_>, what: &str) -> syn::Result<Type> {
    if !input.peek(Token![->]) {
        return Err(input.error(format!(
            "state {what}'s output: `-> Self` (each field's own result) or `-> Option<Self>` \
             (`None` if any field's is)"
        )));
    }
    input.parse::<Token![->]>()?;
    input.parse()
}

impl Parse for Item {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let path: Path = input.parse()?;
        // `TryFrom`, `PartialOrd`: alone.
        if input.is_empty() || input.peek(Token![,]) {
            if path.is_ident("TryFrom") {
                return Ok(Item::TryFrom);
            }
            if path.is_ident("PartialOrd") {
                return Ok(Item::PartialOrd);
            }
        }
        if input.peek(syn::token::Paren) {
            // A trait method: `path::Trait::method(recv, args..) -> T [= ..]`.
            let mut tr = path.clone();
            let Some(seg) = tr.segments.pop() else {
                return Err(syn::Error::new_spanned(path, ITEMS_HELP));
            };
            let seg = seg.into_value();
            if tr.segments.is_empty() || !seg.arguments.is_none() {
                return Err(syn::Error::new_spanned(
                    &path,
                    "a method is named by its trait: `path::Trait::method(self, ..)`",
                ));
            }
            // Drop the trailing `::` the pop left.
            let tr = Path {
                leading_colon: tr.leading_colon,
                segments: tr.segments.into_iter().collect(),
            };
            let inner;
            syn::parenthesized!(inner in input);
            let recv = if inner.peek(Token![&]) {
                inner.parse::<Token![&]>()?;
                if inner.peek(Token![mut]) {
                    inner.parse::<Token![mut]>()?;
                    inner.parse::<Token![self]>()?;
                    Recv::Mut
                } else {
                    inner.parse::<Token![self]>()?;
                    Recv::Ref
                }
            } else if inner.peek(Token![self]) {
                inner.parse::<Token![self]>()?;
                Recv::Value
            } else {
                return Err(inner.error(
                    "a lifted method takes `self`, `&self` or `&mut self` first: it is called on \
                     every field",
                ));
            };
            let mut args = Vec::new();
            while inner.peek(Token![,]) {
                inner.parse::<Token![,]>()?;
                if inner.is_empty() {
                    break;
                }
                let n: Ident = inner.parse()?;
                inner.parse::<Token![:]>()?;
                let t: Type = inner.parse()?;
                args.push((n, t));
            }
            if !inner.is_empty() {
                return Err(inner.error("expected `, name: Type` or `)`"));
            }
            let ret = arrow(input, &format!("`{}`", seg.ident))?;
            let body = if input.peek(Token![=]) {
                input.parse::<Token![=]>()?;
                if !input.peek(Token![self]) {
                    return Err(input.error("after `=`, a getter names a field: `= self.field`"));
                }
                input.parse::<Token![self]>()?;
                input.parse::<Token![.]>()?;
                let m: Member = input.parse()?;
                if !args.is_empty() {
                    return Err(syn::Error::new_spanned(
                        &seg.ident,
                        "a getter returns a field: it takes no arguments",
                    ));
                }
                Body::Getter(m)
            } else {
                let out = out_of(&ret).ok_or_else(|| lift_out_error(&ret))?;
                Body::Lift { out }
            };
            return Ok(Item::Method(Box::new(Method {
                tr,
                method: seg.ident,
                recv,
                args,
                ret,
                body,
            })));
        }
        // An operator: `Op`, `Op<K>`, with its output.
        let (op, rhs) = match path.segments.iter().collect::<Vec<_>>().as_slice() {
            [seg] if path.leading_colon.is_none() => {
                let name = seg.ident.to_string();
                let known = name == "Neg" || BINARY.iter().any(|(o, _, _)| *o == name);
                if !known {
                    return Err(syn::Error::new_spanned(&path, ITEMS_HELP));
                }
                let rhs = match &seg.arguments {
                    PathArguments::None => None,
                    PathArguments::AngleBracketed(a) if name == "Mul" || name == "Div" => {
                        match a.args.iter().collect::<Vec<_>>().as_slice() {
                            [GenericArgument::Type(t)] => Some(Box::new(t.clone())),
                            _ => {
                                return Err(syn::Error::new_spanned(
                                    a,
                                    "a scaling names one type: `Mul<f64>`",
                                ));
                            }
                        }
                    }
                    other => {
                        return Err(syn::Error::new_spanned(
                            other,
                            "only `Mul<K>` and `Div<K>` take a right-hand type (scaling)",
                        ));
                    }
                };
                (seg.ident.clone(), rhs)
            }
            _ => return Err(syn::Error::new_spanned(&path, ITEMS_HELP)),
        };
        let ret = arrow(
            input,
            &format!("`{}`", tidy(&path.to_token_stream().to_string())),
        )?;
        let out = out_of(&ret).ok_or_else(|| lift_out_error(&ret))?;
        Ok(Item::Op { op, rhs, out })
    }
}

fn lift_out_error(t: &Type) -> syn::Error {
    syn::Error::new_spanned(
        t,
        "a lifted item builds `Self` from every field's result: its output is `Self` (each field's \
         result used directly) or `Option<Self>` (each through `Into<Option<_>>`, `None` if any \
         is); a getter returns a field, `= self.field`",
    )
}

/// One entry of a container `#[dynamic(..)]`.
enum Container {
    Mapping(Box<Mapping>),
    Impl(Vec<Item>),
}

impl Parse for Container {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        if input.peek(Token![impl]) {
            input.parse::<Token![impl]>()?;
            let inner;
            syn::parenthesized!(inner in input);
            let items: Punctuated<Item, Token![,]> = Punctuated::parse_terminated(&inner)?;
            Ok(Container::Impl(items.into_iter().collect()))
        } else {
            Ok(Container::Mapping(Box::new(input.parse()?)))
        }
    }
}

// ===== Checks =================================================================

/// One field of the dynamic type.
struct Field {
    member: Member,
    ty: Type,
    fact: bool,
    /// `#[dynamic(same)]`: two operands must agree in it; one operand keeps it.
    same: bool,
}

fn member_str(m: &Member) -> String {
    match m {
        Member::Named(i) => i.to_string(),
        Member::Unnamed(i) => i.index.to_string(),
    }
}

fn path_str(p: &[Member]) -> String {
    if p.is_empty() {
        return "self".into();
    }
    p.iter().map(|m| format!(".{}", member_str(m))).collect()
}

/// Every field filled exactly once, nothing else; payload paths distinct and
/// rebuildable; every declared parameter in a fact.
fn check_mapping(m: &Mapping, fields: &[Field]) -> syn::Result<()> {
    for (i, (f, _)) in m.fills.iter().enumerate() {
        if !fields.iter().any(|x| x.member == *f) {
            return Err(syn::Error::new(
                f.span(),
                format!("the dynamic type has no field `{}`", member_str(f)),
            ));
        }
        if m.fills[..i].iter().any(|(o, _)| o == f) {
            return Err(syn::Error::new(
                f.span(),
                format!("`{}` is filled twice", member_str(f)),
            ));
        }
    }
    for f in fields {
        if !m.fills.iter().any(|(o, _)| *o == f.member) {
            return Err(syn::Error::new(
                m.span,
                format!(
                    "`{}` fills no `{}`: every field comes from a const parameter, a constant \
                     or a field path (`.0`, `self`)",
                    m.name(),
                    member_str(&f.member)
                ),
            ));
        }
    }
    for p in &m.params {
        let name = p.name();
        let used = m.fills.iter().any(|(_, s)| match s {
            Source::Param(q) => q == name,
            Source::Const { expr, .. } => mentions(expr.to_token_stream(), &[name]),
            Source::Path(_) => false,
        });
        if !used {
            return Err(syn::Error::new_spanned(
                name,
                format!(
                    "`{name}` is in no fact: two `{}` differing only in `{name}` would come back \
                     as each other - fill a field with it{}",
                    tidy(&m.path.to_token_stream().to_string()),
                    if matches!(p, Param::Type(_)) {
                        " (one of its constants: `C::CODE`)"
                    } else {
                        ""
                    }
                ),
            ));
        }
    }
    let paths: Vec<&Vec<Member>> = m
        .fills
        .iter()
        .filter_map(|(_, s)| match s {
            Source::Path(p) => Some(p),
            _ => None,
        })
        .collect();
    for (i, a) in paths.iter().enumerate() {
        for b in &paths[..i] {
            let n = a.len().min(b.len());
            if a[..n] == b[..n] {
                return Err(syn::Error::new(
                    m.span,
                    format!(
                        "`{}` and `{}` overlap: each part of `{}` moves once",
                        path_str(b),
                        path_str(a),
                        m.name()
                    ),
                ));
            }
        }
        for k in 1..a.len() {
            if decl_at(&m.decls, &a[..k]).is_none() {
                return Err(syn::Error::new(
                    a[k - 1].span(),
                    format!(
                        "rebuilding `{}` from `{}` needs the type of `{}`: declare it, \
                         `{} {{ {}: Type }} => {{ .. }}`",
                        m.name(),
                        path_str(a),
                        path_str(&a[..k]),
                        m.name(),
                        member_str(&a[0])
                    ),
                ));
            }
        }
    }
    Ok(())
}

fn decl_at<'a>(decls: &'a [Decl], path: &[Member]) -> Option<&'a Decl> {
    let (first, rest) = path.split_first()?;
    let d = decls.iter().find(|d| d.member == *first)?;
    if rest.is_empty() {
        Some(d)
    } else {
        decl_at(&d.children, rest)
    }
}

/// A term of a fact pattern: a const parameter of one of the two mappings (a
/// free value), a constant (by its tokens), or a constant naming a
/// parameter of one side (its value per member).
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Term {
    Param(usize, String),
    /// A constant, by its field and tokens (`0` in a `u8` tag and `0` in a
    /// `u16` code are different values).
    Const(String, String),
    PerMember(usize, String),
}

/// The checks that tell the mappings apart: `global` const items, and per
/// mapping the assertions its impls make for each member used.
struct Apart {
    global: TokenStream2,
    per_member: Vec<Vec<TokenStream2>>,
}

/// The static types must be told apart by their facts. For each pair of
/// mappings, unify their fact patterns field by field (union-find): the two
/// can hold the same facts exactly when, in every class of unified terms,
/// the valued terms (constants, and constants naming a parameter) are equal.
/// A class whose values can be compared at compile time gives a condition;
/// with none, the pair is refused here. Conditions on constants alone become
/// a `const` assertion; those naming one side's parameters an assertion in
/// that side's impls, checked per member.
fn differentiate(maps: &[Mapping], fields: &[Field], generic_tys: &[Ident]) -> syn::Result<Apart> {
    let facts: Vec<&Field> = fields.iter().filter(|f| f.fact).collect();
    let mut apart = Apart {
        global: TokenStream2::new(),
        per_member: maps.iter().map(|_| Vec::new()).collect(),
    };
    for j in 0..maps.len() {
        for i in 0..j {
            let (a, b) = (&maps[i], &maps[j]);
            if a.shared || b.shared {
                continue;
            }
            let side_maps = [a, b];
            let mut terms: Vec<Term> = Vec::new();
            let mut parent: Vec<usize> = Vec::new();
            // A valued term's expression and the field type it has.
            let mut exprs: BTreeMap<Term, (TokenStream2, Type)> = BTreeMap::new();
            fn find(p: &mut [usize], x: usize) -> usize {
                let mut r = x;
                while p[r] != r {
                    r = p[r];
                }
                p[x] = r;
                r
            }
            let id = |t: Term, terms: &mut Vec<Term>, parent: &mut Vec<usize>| -> usize {
                match terms.iter().position(|x| *x == t) {
                    Some(k) => k,
                    None => {
                        terms.push(t);
                        parent.push(parent.len());
                        parent.len() - 1
                    }
                }
            };
            for f in &facts {
                let mut ids = Vec::new();
                for (side, m) in side_maps.iter().enumerate() {
                    let t = match m.source(&f.member) {
                        Source::Param(p) => Term::Param(side, p.to_string()),
                        Source::Const { expr, generic } => {
                            let s = expr.to_token_stream().to_string();
                            let t = if *generic {
                                Term::PerMember(side, s)
                            } else {
                                Term::Const(member_str(&f.member), s)
                            };
                            exprs
                                .entry(t.clone())
                                .or_insert((expr.to_token_stream(), f.ty.clone()));
                            t
                        }
                        Source::Path(_) => unreachable!("roles are checked first"),
                    };
                    ids.push(id(t, &mut terms, &mut parent));
                }
                let (rx, ry) = (find(&mut parent, ids[0]), find(&mut parent, ids[1]));
                parent[rx] = ry;
            }
            // The valued terms of each class.
            let mut classes: BTreeMap<usize, Vec<Term>> = BTreeMap::new();
            for (k, term) in terms.iter().enumerate() {
                if matches!(term, Term::Param(..)) {
                    continue;
                }
                let r = find(&mut parent, k);
                let c = classes.entry(r).or_default();
                if !c.contains(term) {
                    c.push(term.clone());
                }
            }
            let (na, nb) = (a.name(), b.name());
            // Each class with two or more values gives a condition, unless it
            // compares the two sides' per-member constants (no impl sees both).
            let mut conds: Vec<TokenStream2> = Vec::new();
            let mut sides: Vec<usize> = Vec::new();
            for c in classes.values().filter(|c| c.len() > 1) {
                let per: Vec<usize> = c
                    .iter()
                    .filter_map(|t| match t {
                        Term::PerMember(s, _) => Some(*s),
                        _ => None,
                    })
                    .collect();
                if per.contains(&0) && per.contains(&1) {
                    continue;
                }
                let mut eqs = Vec::new();
                for w in c.windows(2) {
                    let (ea, ty) = &exprs[&w[0]];
                    let (eb, _) = &exprs[&w[1]];
                    eqs.push(quote! {{
                        let (__a, __b): (#ty, #ty) = (const { #ea }, const { #eb });
                        __a == __b
                    }});
                }
                if let Some(s) = per.first() {
                    sides.push(*s);
                } else {
                    // A global `const` item cannot name a parameter of the
                    // dynamic type.
                    let (_, ty) = &exprs[&c[0]];
                    if mentions(
                        ty.to_token_stream(),
                        &generic_tys.iter().collect::<Vec<_>>(),
                    ) {
                        continue;
                    }
                }
                conds.push(quote! { (#(#eqs)&&*) });
            }
            if conds.is_empty() {
                return Err(syn::Error::new(
                    b.span,
                    format!(
                        "`{na}` and `{nb}` can have the same facts, so the way back could not tell \
                         them apart: give them a fact whose constants differ (a field filled \
                         with a different constant in each, e.g. a tag) - or, if one is the same \
                         value as the other (a bare number and a dimensionless quantity), mark it \
                         `shared`"
                    ),
                ));
            }
            sides.sort_unstable();
            sides.dedup();
            let msg = format!(
                "`{na}` and `{nb}` have the same facts, so the way back could not tell them apart: \
                 their constants must differ"
            );
            match sides.as_slice() {
                [] => apart.global.extend(quote_spanned! { b.span=>
                    const _: () = ::core::assert!(!(#(#conds)&&*), #msg);
                }),
                [s] => {
                    let msg = format!(
                        "{msg} (checked for each member of `{}`)",
                        side_maps[*s].name()
                    );
                    apart.per_member[if *s == 0 { i } else { j }].push(quote! {
                        const { ::core::assert!(!(#(#conds)&&*), #msg) };
                    });
                }
                _ => {
                    return Err(syn::Error::new(
                        b.span,
                        format!(
                            "`{na}` and `{nb}` are told apart only by constants of both families' \
                             parameters, which no one impl sees: give them a fact whose constants \
                             differ (e.g. a tag)"
                        ),
                    ));
                }
            }
        }
    }
    Ok(apart)
}

// ===== Expansion ==============================================================

/// One predicate per distinct type.
fn each<'a>(
    tys: impl Iterator<Item = &'a Type>,
    pred: impl Fn(&Type) -> TokenStream2,
) -> Vec<TokenStream2> {
    let mut seen: Vec<String> = Vec::new();
    let mut out = Vec::new();
    for t in tys {
        let key = quote! { #t }.to_string();
        if !seen.contains(&key) {
            seen.push(key);
            out.push(pred(t));
        }
    }
    out
}

/// The static value rebuilt from the dynamic one's payload: a struct
/// expression per level, the declared types in between, and `Default` for a
/// declared field no path reaches.
fn rebuild(
    ty: &TokenStream2,
    prefix: &[Member],
    decls: &[Decl],
    payload: &[(&Member, &Vec<Member>)],
) -> TokenStream2 {
    if let Some((field, _)) = payload.iter().find(|(_, p)| p.is_empty()) {
        return quote! { __dynamic.#field };
    }
    let mut next: Vec<&Member> = Vec::new();
    for (_, p) in payload {
        if p.len() > prefix.len()
            && p[..prefix.len()] == *prefix
            && !next.contains(&&p[prefix.len()])
        {
            next.push(&p[prefix.len()]);
        }
    }
    let mut parts: Vec<TokenStream2> = next
        .iter()
        .map(|seg| {
            let mut path = prefix.to_vec();
            path.push((*seg).clone());
            let value = match payload.iter().find(|(_, p)| **p == path) {
                Some((field, _)) => quote! { __dynamic.#field },
                None => {
                    let d = decl_at(decls, &path).expect("checked");
                    let t = &d.ty;
                    rebuild(&quote! { #t }, &path, decls, payload)
                }
            };
            quote! { #seg: #value }
        })
        .collect();
    // Declared fields at this level that no path reaches.
    let level: &[Decl] = if prefix.is_empty() {
        decls
    } else {
        match decl_at(decls, prefix) {
            Some(d) => &d.children,
            None => &[],
        }
    };
    for d in level {
        if !next.contains(&&d.member) {
            let (m, t) = (&d.member, &d.ty);
            parts.push(quote! { #m: <#t as ::core::default::Default>::default() });
        }
    }
    quote! { #ty { #(#parts,)* } }
}

/// The types of declared fields filled with `Default`.
fn defaulted<'a>(
    decls: &'a [Decl],
    payload: &[&Vec<Member>],
    prefix: &mut Vec<Member>,
    out: &mut Vec<&'a Path>,
) {
    for d in decls {
        prefix.push(d.member.clone());
        let reached = payload
            .iter()
            .any(|p| p.len() >= prefix.len() && p[..prefix.len()] == prefix[..]);
        if reached {
            defaulted(&d.children, payload, prefix, out);
        } else {
            out.push(&d.ty);
        }
        prefix.pop();
    }
}

/// The trait path without associated-type bindings, for calling a method
/// (`my::HasDim<D = X>` -> `my::HasDim`), and the bindings.
fn split_bindings(tr: &Path) -> (Path, Vec<(Ident, Type)>) {
    let mut p = tr.clone();
    let mut binds = Vec::new();
    if let Some(last) = p.segments.last_mut()
        && let PathArguments::AngleBracketed(a) = &mut last.arguments
    {
        let mut kept = Punctuated::<GenericArgument, Token![,]>::new();
        for arg in a.args.iter() {
            match arg {
                GenericArgument::AssocType(at) => binds.push((at.ident.clone(), at.ty.clone())),
                other => kept.push(other.clone()),
            }
        }
        if kept.is_empty() {
            last.arguments = PathArguments::None;
        } else {
            a.args = kept;
        }
    }
    (p, binds)
}

pub(crate) fn expand(input: &DeriveInput) -> syn::Result<TokenStream2> {
    krate(input)?;
    let name = &input.ident;
    let Data::Struct(s) = &input.data else {
        return Err(syn::Error::new_spanned(
            name,
            "`Dynamic` is for a struct: its fields hold what a static type held",
        ));
    };
    if matches!(s.fields, Fields::Unit) || s.fields.is_empty() {
        return Err(syn::Error::new_spanned(name, "`Dynamic` needs fields"));
    }

    // --- Container ---------------------------------------------------------------
    let mut maps: Vec<Mapping> = Vec::new();
    let mut items = Vec::new();
    for a in input.attrs.iter().filter(|a| a.path().is_ident("dynamic")) {
        let list: Punctuated<Container, Token![,]> =
            a.parse_args_with(Punctuated::parse_terminated)?;
        for c in list {
            match c {
                Container::Mapping(m) => maps.push(*m),
                Container::Impl(i) => items.extend(i),
            }
        }
    }
    if maps.is_empty() {
        return Err(syn::Error::new_spanned(
            name,
            "`Dynamic` needs a mapping: `#[dynamic(Qty<const D: Dim> => { dim: D, value: .0 })]` \
             - each field from a const parameter or a constant (a fact) or a field path (payload)",
        ));
    }

    // --- Fields and their roles ------------------------------------------------------
    let mut fields: Vec<Field> = Vec::new();
    for (i, f) in s.fields.iter().enumerate() {
        let member = match &f.ident {
            Some(id) => Member::Named(id.clone()),
            None => Member::Unnamed(syn::Index::from(i)),
        };
        let mut same = false;
        for a in f.attrs.iter().filter(|a| a.path().is_ident("dynamic")) {
            a.parse_nested_meta(|meta| {
                if meta.path.is_ident("same") {
                    same = true;
                    Ok(())
                } else {
                    Err(meta.error(
                        "on a field, `#[dynamic(..)]` takes `same`: two operands must be equal \
                         in it (else there is no result), and one operand keeps it; what a \
                         field holds is said by each mapping, on the struct",
                    ))
                }
            })?;
        }
        fields.push(Field {
            member,
            ty: f.ty.clone(),
            fact: false,
            same,
        });
    }
    for m in &maps {
        check_mapping(m, &fields)?;
    }
    for f in &mut fields {
        let first = maps[0].source(&f.member).is_fact();
        for m in &maps[1..] {
            if m.source(&f.member).is_fact() != first {
                let role = |fact: bool| if fact { "a fact" } else { "payload" };
                return Err(syn::Error::new(
                    m.span,
                    format!(
                        "`{}` is {} in `{}` but {} in `{}`: a field plays one role in every \
                         mapping (a fact: a const parameter or a constant; payload: a field path)",
                        member_str(&f.member),
                        role(first),
                        maps[0].name(),
                        role(!first),
                        m.name()
                    ),
                ));
            }
        }
        f.fact = first;
    }
    let generic_tys: Vec<Ident> = input
        .generics
        .type_params()
        .map(|p| p.ident.clone())
        .collect();
    let apart = differentiate(&maps, &fields, &generic_tys)?;
    let mut out = apart.global.clone();

    // --- Generics -----------------------------------------------------------------
    let g = &input.generics;
    let lts: Vec<&GenericParam> = g
        .params
        .iter()
        .filter(|p| matches!(p, GenericParam::Lifetime(_)))
        .collect();
    let rest: Vec<&GenericParam> = g
        .params
        .iter()
        .filter(|p| !matches!(p, GenericParam::Lifetime(_)))
        .collect();
    let head = |extra: &[TokenStream2]| quote! { impl<#(#lts,)* #(#rest,)* #(#extra),*> };
    let (_, ty_args, _) = g.split_for_impl();
    let preds: Vec<TokenStream2> = g
        .where_clause
        .iter()
        .flat_map(|w| w.predicates.iter())
        .map(|p| quote! { #p })
        .collect();
    let me = quote! { #name #ty_args };

    let destruct = quote! { [const] ::core::marker::Destruct };
    let opt = quote! { ::core::option::Option };
    let res = quote! { ::core::result::Result };
    // Whatever is dropped - a refused operator's operands, what is left of a
    // value whose parts moved - must be `[const] Destruct` for a `const` impl.
    let all_destruct = each(fields.iter().map(|f| &f.ty), |t| quote! { #t: #destruct });
    let facts: Vec<&Field> = fields.iter().filter(|f| f.fact).collect();
    let payload: Vec<&Field> = fields.iter().filter(|f| !f.fact).collect();
    let fact_eq = each(facts.iter().map(|f| &f.ty), |t| {
        quote! { #t: [const] ::core::cmp::PartialEq }
    });
    let fm: Vec<&Member> = facts.iter().map(|f| &f.member).collect();
    let pm: Vec<&Member> = payload.iter().map(|f| &f.member).collect();
    let pt: Vec<&Type> = payload.iter().map(|f| &f.ty).collect();
    // The new value of each field, in a `let`.
    let var = |m: &Member| format_ident!("__v_{}", member_str(m));
    let all_vars: Vec<Ident> = fields.iter().map(|f| var(&f.member)).collect();
    let all_members: Vec<&Member> = fields.iter().map(|f| &f.member).collect();
    let build = quote! { Self { #(#all_members: #all_vars,)* } };

    // --- Dynamic, From and TryFrom, per mapping -------------------------------------
    for (mi, m) in maps.iter().enumerate() {
        let sty = m.ty();
        let h = head(&m.param_decls());
        let checks_apart = &apart.per_member[mi];
        let fill = m.fills.iter().map(|(f, s)| match s {
            Source::Param(p) => quote! { #f: #p },
            Source::Const { expr, .. } => quote! { #f: const { #expr } },
            Source::Path(p) => quote! { #f: __static #(.#p)* },
        });
        let checks: Vec<TokenStream2> = m
            .fills
            .iter()
            .filter_map(|(f, s)| match s {
                Source::Param(p) => Some(quote! { __dynamic.#f == #p }),
                Source::Const { expr, .. } => Some(quote! { __dynamic.#f == const { #expr } }),
                Source::Path(_) => None,
            })
            .collect();
        let pay: Vec<(&Member, &Vec<Member>)> = m
            .fills
            .iter()
            .filter_map(|(f, s)| match s {
                Source::Path(p) => Some((f, p)),
                _ => None,
            })
            .collect();
        let back = rebuild(&m.expr(), &[], &m.decls, &pay);
        let mut dflt = Vec::new();
        let pay_paths: Vec<&Vec<Member>> = pay.iter().map(|(_, p)| *p).collect();
        defaulted(&m.decls, &pay_paths, &mut Vec::new(), &mut dflt);
        let dflt_bounds: Vec<TokenStream2> = dflt
            .iter()
            .map(|t| quote! { #t: [const] ::core::default::Default })
            .collect();
        let all = if checks.is_empty() {
            quote! { true }
        } else {
            quote! { #(#checks)&&* }
        };
        out.extend(quote_spanned! { m.span=>
            // A tuple struct is built with numbered fields: a type alias has no
            // constructor function, so `S { 0: .. }` is the one form for both.
            #[allow(clippy::init_numbered_fields)]
            const #h ::core::convert::From<#sty> for #me
            where #(#preds,)* #sty: #destruct, #(#all_destruct,)*
            {
                fn from(__static: #sty) -> Self {
                    #(#checks_apart)*
                    Self { #(#fill,)* }
                }
            }
            #[allow(clippy::init_numbered_fields)]
            const #h ::core::convert::TryFrom<#me> for #sty
            where #(#preds,)* #(#all_destruct,)* #(#fact_eq,)* #(#dflt_bounds,)*
            {
                type Error = #me;
                fn try_from(__dynamic: #me) -> #res<Self, #me> {
                    #(#checks_apart)*
                    if #all {
                        #res::Ok(#back)
                    } else {
                        #res::Err(__dynamic)
                    }
                }
            }
        });
    }

    // --- Lifted items ----------------------------------------------------------------
    // One field's result: used directly, or through `Into<Option<_>>` with
    // `None` returned early.
    let take = |out: Out, ty: &TokenStream2, e: TokenStream2| -> TokenStream2 {
        match out {
            Out::Plain => e,
            Out::Partial => quote! {
                match <_ as ::core::convert::Into<#opt<#ty>>>::into(#e) {
                    #opt::Some(__x) => __x,
                    #opt::None => return #opt::None,
                }
            },
        }
    };
    let wrap = |out: Out, v: TokenStream2| match out {
        Out::Plain => v,
        Out::Partial => quote! { #opt::Some(#v) },
    };
    let out_ty = |out: Out| match out {
        Out::Plain => quote! { #me },
        Out::Partial => quote! { #opt<#me> },
    };
    // The bound an operator's result needs on a field type `t`.
    // `tr` is the operator's path, `rhs` its right operand when not `Self`.
    let result_bound = |out: Out, t: &Type, tr: &TokenStream2, rhs: Option<&Type>| {
        let args = rhs.map(|r| quote! { #r, });
        let with = rhs.map(|r| quote! { <#r> });
        match out {
            Out::Plain => vec![quote! { #t: [const] #tr<#args Output = #t> }],
            Out::Partial => vec![
                quote! { #t: [const] #tr #with },
                quote! { <#t as #tr #with>::Output: [const] ::core::convert::Into<#opt<#t>> },
            ],
        }
    };
    let drops = &all_destruct;

    // Methods and getters of one trait go in one impl.
    let mut by_trait: Vec<(String, Vec<&Method>)> = Vec::new();
    for item in &items {
        match item {
            Item::TryFrom => {}
            Item::PartialOrd => {
                let h = head(&[]);
                let ord = quote! { ::core::cmp::Ordering };
                let same = if fm.is_empty() {
                    quote! { true }
                } else {
                    quote! { #(self.#fm == other.#fm)&&* }
                };
                let cmp = match pm.split_last() {
                    None => quote! { #opt::Some(#ord::Equal) },
                    Some((last, first)) => quote! {
                        #(
                            match ::core::cmp::PartialOrd::partial_cmp(&self.#first, &other.#first) {
                                #opt::Some(#ord::Equal) => {}
                                c => return c,
                            }
                        )*
                        ::core::cmp::PartialOrd::partial_cmp(&self.#last, &other.#last)
                    },
                };
                let pay_ord = each(
                    pt.iter().copied(),
                    |t| quote! { #t: [const] ::core::cmp::PartialOrd },
                );
                // `const` only when `Self`'s `PartialEq` is: the bound says so.
                out.extend(quote! {
                    const #h ::core::cmp::PartialOrd for #me
                    where #(#preds,)* Self: [const] ::core::cmp::PartialEq, #(#fact_eq,)* #(#pay_ord,)*
                    {
                        fn partial_cmp(&self, other: &Self) -> #opt<#ord> {
                            if #same { #cmp } else { #opt::None }
                        }
                    }
                });
            }
            Item::Op { op, rhs, out: o } => {
                // Every field by its own operator; a `same` field must agree
                // between two operands, and is kept by a one-operand operator.
                let o = *o;
                let h = head(&[]);
                let tr = quote! { ::core::ops::#op };
                let output = out_ty(o);
                let neg = op == "Neg";
                let binary = !neg && rhs.is_none();
                if binary
                    && o == Out::Plain
                    && let Some(f) = fields.iter().find(|f| f.same)
                {
                    return Err(syn::Error::new(
                        f.member.span(),
                        format!(
                            "`{}` is `same`: two values that differ in it have no `{op}`, so \
                             state `{op} -> Option<Self>`",
                            member_str(&f.member)
                        ),
                    ));
                }
                let (tok, meth) = match BINARY.iter().find(|(o, _, _)| op == o) {
                    Some((_, tok, meth)) => (tok.parse::<TokenStream2>().unwrap(), format_ident!("{meth}")),
                    None => (quote! {}, format_ident!("neg")),
                };
                let mut lets = Vec::new();
                let mut bounds: Vec<TokenStream2> = Vec::new();
                let mut uses_k = 0usize;
                for f in &fields {
                    let (m, t) = (&f.member, &f.ty);
                    let v = var(m);
                    let tt = quote! { #t };
                    let e = if f.same {
                        if binary {
                            bounds.push(quote! { #t: [const] ::core::cmp::PartialEq });
                            quote! { if self.#m == rhs.#m { self.#m } else { return #opt::None } }
                        } else {
                            quote! { self.#m }
                        }
                    } else if neg {
                        bounds.extend(result_bound(o, t, &tr, None));
                        take(o, &tt, quote! { -self.#m })
                    } else if let Some(kt) = rhs {
                        uses_k += 1;
                        bounds.extend(result_bound(o, t, &tr, Some(kt)));
                        take(o, &tt, quote! { self.#m #tok k })
                    } else {
                        bounds.extend(result_bound(o, t, &tr, None));
                        take(o, &tt, quote! { self.#m #tok rhs.#m })
                    };
                    lets.push(quote! { let #v: #t = #e; });
                }
                let mut seen = Vec::new();
                bounds.retain(|b| {
                    let s = b.to_string();
                    !seen.contains(&s) && {
                        seen.push(s);
                        true
                    }
                });
                let body = wrap(o, build.clone());
                let (with, sig, extra) = if neg {
                    (quote! {}, quote! { fn neg(self) }, quote! {})
                } else if let Some(kt) = rhs {
                    let copy = if uses_k > 1 {
                        quote! { #kt: ::core::marker::Copy, }
                    } else {
                        quote! {}
                    };
                    (
                        quote! { <#kt> },
                        quote! { fn #meth(self, k: #kt) },
                        quote! { #kt: #destruct, #copy },
                    )
                } else {
                    (quote! {}, quote! { fn #meth(self, rhs: Self) }, quote! {})
                };
                out.extend(quote! {
                    #[allow(clippy::init_numbered_fields)]
                    const #h #tr #with for #me
                    where #(#preds,)* #(#bounds,)* #(#drops,)* #extra
                    {
                        type Output = #output;
                        #[allow(unused_variables)]
                        #sig -> #output { #(#lets)* #body }
                    }
                });
            }
            Item::Method(m) => {
                let (bare, _) = split_bindings(&m.tr);
                let key = bare.to_token_stream().to_string();
                match by_trait.iter_mut().find(|(k, _)| *k == key) {
                    Some((_, v)) => v.push(m),
                    None => by_trait.push((key, vec![m])),
                }
            }
        }
    }

    for (_, methods) in &by_trait {
        let (bare, binds) = split_bindings(&methods[0].tr);
        let mut all_binds = binds;
        for m in &methods[1..] {
            for b in split_bindings(&m.tr).1 {
                if !all_binds.iter().any(|(n, _)| *n == b.0) {
                    all_binds.push(b);
                }
            }
        }
        let assoc = all_binds.iter().map(|(n, t)| quote! { type #n = #t; });
        let getters_only = methods.iter().all(|m| matches!(m.body, Body::Getter(_)));
        let mut bounds: Vec<TokenStream2> = Vec::new();
        let mut fns = Vec::new();
        for m in methods {
            let meth = &m.method;
            let recv = match m.recv {
                Recv::Value => quote! { self },
                Recv::Ref => quote! { &self },
                Recv::Mut => quote! { &mut self },
            };
            let (an, at): (Vec<&Ident>, Vec<&Type>) = m.args.iter().map(|(n, t)| (n, t)).unzip();
            let ret = &m.ret;
            match &m.body {
                Body::Getter(f) => {
                    if !fields.iter().any(|x| x.member == *f) {
                        return Err(syn::Error::new(
                            f.span(),
                            format!("the dynamic type has no field `{}`", member_str(f)),
                        ));
                    }
                    fns.push(quote! {
                        fn #meth(#recv) -> #ret { self.#f }
                    });
                }
                Body::Lift { out: o } => {
                    // The method on every field; a `same` field is kept.
                    let o = *o;
                    let mut lets = Vec::new();
                    let mut called = 0usize;
                    for f in &fields {
                        let (x, t) = (&f.member, &f.ty);
                        let v = var(x);
                        let e = if f.same {
                            match m.recv {
                                Recv::Value => quote! { self.#x },
                                Recv::Ref | Recv::Mut => {
                                    bounds.push(quote! { #t: ::core::clone::Clone });
                                    quote! { ::core::clone::Clone::clone(&self.#x) }
                                }
                            }
                        } else {
                            called += 1;
                            bounds.push(quote! { #t: #bare });
                            let a = match m.recv {
                                Recv::Value => quote! { self.#x },
                                Recv::Ref => quote! { &self.#x },
                                Recv::Mut => quote! { &mut self.#x },
                            };
                            let tt = quote! { #t };
                            take(o, &tt, quote! { <#t as #bare>::#meth(#a, #(#an),*) })
                        };
                        lets.push(quote! { let #v: #t = #e; });
                    }
                    if called > 1 {
                        for t in &at {
                            bounds.push(quote! { #t: ::core::marker::Copy });
                        }
                    }
                    let body = wrap(o, build.clone());
                    fns.push(quote! {
                        #[allow(clippy::init_numbered_fields, unused_variables)]
                        fn #meth(#recv, #(#an: #at),*) -> #ret { #(#lets)* #body }
                    });
                }
            }
        }
        let mut seen = Vec::new();
        bounds.retain(|b| {
            let s = b.to_string();
            if seen.contains(&s) {
                false
            } else {
                seen.push(s);
                true
            }
        });
        let h = head(&[]);
        // A getter only reads a field: `const`. The trait is named without
        // its bindings, which become associated types.
        let constness = if getters_only {
            quote! { const }
        } else {
            quote! {}
        };
        out.extend(quote! {
            #constness #h #bare for #me where #(#preds,)* #(#bounds,)* {
                #(#assoc)*
                #(#fns)*
            }
        });
    }
    Ok(out)
}
