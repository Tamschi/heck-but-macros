#![allow(non_snake_case)]
#![doc(html_root_url = "https://docs.rs/heck-but-macros/0.0.1")]
#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]

extern crate proc_macro;
use heck::{
	CamelCase, KebabCase, MixedCase, ShoutySnakeCase, ShoutySnekCase, SnakeCase, SnekCase,
	TitleCase,
};
use proc_macro::{
	Delimiter, Group as pmGroup, Ident as pmIdent, Literal as pmLiteral, TokenStream, TokenTree,
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
			let tree: TokenTree = match token {
				TokenTree::Group(group) => {
					pmGroup::new(group.delimiter(), case(group.stream(), case_str)).into()
				}
				TokenTree::Ident(ident) => {
					pmIdent::new(&case_str(&format!("{}", ident)), ident.span()).into()
				}
				punct @ TokenTree::Punct(_) => punct,
				TokenTree::Literal(_) => todo!("Literals are not supported yet."),
			};
			tree
		})
		.collect()
}

fn stringify(input: TokenStream) -> TokenStream {
	let mut input = input.into_iter();
	let result = match input.next() {
		Some(token) => match token {
			TokenTree::Group(group) if group.delimiter() == Delimiter::None => {
				stringify(group.stream())
			}
			TokenTree::Ident(ident) => {
				TokenTree::Literal(pmLiteral::string(&ident.to_string())).into()
			}
			other => todo!("Stringifcation of tokens other than Ident ({:?})", other),
		},
		None => TokenStream::new(),
	};
	if input.next().is_some() {
		todo!("Stringification of multiple input tokens")
	}
	result
}
