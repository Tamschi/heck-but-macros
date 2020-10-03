#![forbid(unsafe_code)]
#![allow(non_snake_case)]

extern crate proc_macro;
use {
    heck::{
        CamelCase, KebabCase, MixedCase, ShoutySnakeCase, ShoutySnekCase, SnakeCase, SnekCase,
        TitleCase,
    },
    proc_macro::{
        Group as pmGroup, Ident as pmIdent, Literal as pmLiteral, TokenStream, TokenTree,
    },
};

macro_rules! case_macro {
    ($Trait:ident::$fn:ident => $name:ident $stringify_name:ident) => {
        #[proc_macro]
        pub fn $name(input: TokenStream) -> TokenStream {
            case(input, $Trait::$fn)
        }

        #[proc_macro]
        pub fn $stringify_name(input: TokenStream) -> TokenStream {
            stringify(case(input, $Trait::$fn))
        }
    };
}

macro_rules! case_macros {
    ($($Trait:ident::$fn:ident => $name:ident  $stringify_name:ident,)*) => {
        $(case_macro!($Trait::$fn => $name $stringify_name);)*
    };
}

case_macros! {
    CamelCase::to_camel_case =>
        camel_case stringify_camel_case,
    KebabCase::to_kebab_case =>
        kebab_case stringify_kebab_case,
    MixedCase::to_mixed_case =>
        mixed_case stringify_mixed_case,
    ShoutySnakeCase::to_shouty_snake_case =>
        shouty_snake_case stringify_shouty_snake_case,
    ShoutySnekCase::TO_SHOUTY_SNEK_CASE =>
        SHOUTY_SNEK_CASE stringify_SHOUTY_SNEK_CASE,
    SnakeCase::to_snake_case =>
        snake_case stringify_snake_case,
    SnekCase::to_snek_case =>
        snek_case stringify_snek_case,
    TitleCase::to_title_case =>
        title_case stringify_title_case,
}

fn case(input: TokenStream, case_str: fn(&str) -> String) -> TokenStream {
    input
        .into_iter()
        .map(|token| {
            use TokenTree::*;
            let tree: TokenTree = match token {
                Group(group) => {
                    pmGroup::new(group.delimiter(), case(group.stream(), case_str)).into()
                }
                Ident(ident) => pmIdent::new(&case_str(&format!("{}", ident)), ident.span()).into(),
                punct @ Punct(_) => punct,
                Literal(_) => todo!("Literals are not supported yet."),
            };
            tree
        })
        .collect()
}

fn stringify(input: TokenStream) -> TokenStream {
    TokenTree::Literal(pmLiteral::string(&format!("{}", input).trim())).into()
}