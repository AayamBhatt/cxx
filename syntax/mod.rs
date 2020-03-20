// Functionality that is shared between the cxxbridge macro and the cmd.

pub mod atom;
mod attrs;
pub mod check;
mod doc;
pub mod error;
pub mod ident;
mod impls;
mod parse;
pub mod set;
mod tokens;
pub mod types;

use proc_macro2::{Ident, Span, TokenStream};
use syn::{token::Brace, LitStr, Token};

pub use self::atom::Atom;
pub use self::doc::Doc;
pub use self::parse::parse_items;
pub use self::types::Types;

pub enum Api {
    Include(LitStr),
    Struct(Struct),
    CxxType(ExternType),
    CxxFunction(ExternFn),
    RustType(ExternType),
    RustFunction(ExternFn),
}

pub struct ExternType {
    pub doc: Doc,
    pub type_token: Token![type],
    pub ident: Ident,
}

pub struct Struct {
    pub doc: Doc,
    pub derives: Vec<Ident>,
    pub struct_token: Token![struct],
    pub ident: Ident,
    pub brace_token: Brace,
    pub fields: Vec<Var>,
}

pub struct ExternFn {
    pub lang: Lang,
    pub doc: Doc,
    pub ident: Ident,
    pub sig: Signature,
    pub semi_token: Token![;],
}

pub struct Signature {
    pub fn_token: Token![fn],
    pub receiver: Option<Receiver>,
    pub args: Vec<Var>,
    pub ret: Option<Type>,
    pub throws: bool,
    pub tokens: TokenStream,
}

#[derive(Eq, PartialEq, Hash)]
pub struct Var {
    pub ident: Ident,
    pub ty: Type,
}

pub struct Receiver {
    pub mutability: Option<Token![mut]>,
    pub ident: Ident,
}

pub enum Type {
    Ident(Ident),
    RustBox(Box<Ty1>),
    UniquePtr(Box<Ty1>),
    Ref(Box<Ref>),
    Str(Box<Ref>),
    Fn(Box<Signature>),
    Void(Span),
}

pub struct Ty1 {
    pub name: Ident,
    pub langle: Token![<],
    pub inner: Type,
    pub rangle: Token![>],
}

pub struct Ref {
    pub ampersand: Token![&],
    pub mutability: Option<Token![mut]>,
    pub inner: Type,
}

#[derive(Copy, Clone, PartialEq)]
pub enum Lang {
    Cxx,
    Rust,
}

#[derive(Copy, Clone, PartialEq)]
pub enum Derive {
    Clone,
    Copy,
}
