//! # Example
//! ```
//! #[derive(ReadModel)]
//! #[hasmany(
//!     authers(users, User, Relation),
//!     posts(posts, Post)
//! )]
//! pub struct User {
//!     pub id: i32,
//!     pub name: String,
//!     pub age: i32,
//!     pub created_at: Timestamp
//! }
//! ```

#![recursion_limit = "1024"]

#[macro_use] extern crate quote;
#[macro_use] extern crate syn;
extern crate proc_macro;
extern crate proc_macro2;

mod hasmany;
mod helpers;

use proc_macro2::TokenStream;
use syn::{DeriveInput, Ident, MetaList, NestedMeta};
use syn::Meta::{List, Word};
use std::iter::FromIterator;

use readmodel::ReadModel;

#[proc_macro_derive(ReadModel, attributes(readmodel, belongs_to))]
pub fn preload (input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    impl_readmodel(input).into()
}

pub fn main (input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let preload = Preload::new(input);

    let token_stream_2 = quote! {
        impl #name {
        }
    }

    token_stream_2.into()
}