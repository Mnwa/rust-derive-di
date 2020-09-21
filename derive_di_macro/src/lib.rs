//! Macro for dependency injection pattern realization
extern crate proc_macro;
use proc_macro::TokenStream;

use quote::{format_ident, quote};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, Data, DeriveInput, Expr, PathSegment, Token, Type};

/// Derive `Container` macro, will be implement getters, setters and `Default` trait for the struct.
#[proc_macro_derive(Container, attributes(inject))]
pub fn derive_container_fn(input: TokenStream) -> TokenStream {
    let derived_container = parse_macro_input!(input as DeriveInput);
    let name = &derived_container.ident;
    let data_struct = match &derived_container.data {
        Data::Struct(m) => m,
        _ => unimplemented!(),
    };
    let (impl_generics, ty_generics, where_clause) = &derived_container.generics.split_for_impl();

    let getters = data_struct.fields.iter().map(|field| {
        let field_name = field.ident.clone().expect("tuples not supported");
        let field_name_lc = field_name.clone().to_string().to_lowercase();

        let fn_getter_name = format_ident!("get_{}", field_name_lc);
        let fn_getter_mut_name = format_ident!("get_mut_{}", field_name_lc);
        let fn_setter_name = format_ident!("set_{}", field_name_lc);
        let fn_into_name = format_ident!("into_{}", field_name_lc);
        let fn_type = field.ty.clone();
        quote! {
            pub fn #fn_getter_name(&self) -> &#fn_type {
                &self.#field_name
            }
            pub fn #fn_getter_mut_name(&mut self) -> &mut #fn_type {
                &mut self.#field_name
            }
            pub fn #fn_setter_name(&mut self, #field_name: #fn_type) {
                self.#field_name = #field_name
            }
            pub fn #fn_into_name(self) -> #fn_type {
                self.#field_name
            }
        }
    });

    let new_constructor = data_struct.fields.iter().map(|field| {
        let var_name = field.ident.clone().expect("tuples not supported");

        field.attrs.first()
            .and_then(|origin_struct_name| {
                if let Type::Path(fn_type) = field.ty.clone() {
                    Some((
                        origin_struct_name.parse_args::<PathSegment>().ok().map(|v| v.ident)?,
                        fn_type.path.segments.first().cloned().map(|v| v.ident)?,
                    ))
                } else {
                    None
                }
            })
            .map(|(origin_struct_name, fn_type)| quote!(#var_name: #fn_type::from(#origin_struct_name::get_service())))
            .unwrap_or_else(|| quote!(#var_name: Injectable::get_service()))
    });

    let out = quote! {
        impl #impl_generics #name #ty_generics #where_clause {
            #(
                #getters
            )*
        }

        impl #impl_generics Default for #name #ty_generics #where_clause {
            fn default() -> Self {
                Self {
                    #(
                        #new_constructor
                    )*
                }
            }
        }
    };

    out.into()
}

/// Auto implementation of Injectable trait
#[proc_macro_attribute]
pub fn injectable(args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let input_name = &input.ident;

    let (impl_generics, ty_generics, where_clause) = &input.generics.split_for_impl();

    let InjectableArgsStruct { factory } = parse_macro_input!(args as InjectableArgsStruct);

    let out = quote! {
        #input

        impl #impl_generics Injectable for #input_name #ty_generics #where_clause {
            fn get_service() -> Self {
                #factory
            }
        }
    };

    out.into()
}

type InjectableArgs = Punctuated<Punctuated<Expr, Token![=>]>, Token![,]>;

struct InjectableArgsStruct {
    factory: proc_macro2::TokenStream,
}

impl Parse for InjectableArgsStruct {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let args: InjectableArgs =
            Punctuated::parse_terminated_with(input, Punctuated::parse_separated_nonempty)?;

        let factory = args
            .iter()
            .find(|arg| {
                arg.first()
                    .cloned()
                    .and_then(expr_to_token_stream)
                    .filter(|key| key.to_string() == "factory".to_owned())
                    .is_some()
            })
            .and_then(|arg| arg.last().cloned().and_then(expr_to_token_stream))
            .unwrap_or_else(|| {
                quote! {
                    Default::default()
                }
            });

        Ok(InjectableArgsStruct { factory })
    }
}

fn expr_to_token_stream(expr: Expr) -> Option<proc_macro2::TokenStream> {
    match expr {
        Expr::Call(_)
        | Expr::Lit(_)
        | Expr::MethodCall(_)
        | Expr::Path(_)
        | Expr::Box(_)
        | Expr::Return(_)
        | Expr::Paren(_)
        | Expr::Struct(_)
        | Expr::Index(_)
        | Expr::Field(_)
        | Expr::Binary(_)
        | Expr::Block(_)
        | Expr::Cast(_) => Some(quote!(#expr)),
        Expr::Closure(_) => Some(quote! {
            (#expr)()
        }),
        _ => None,
    }
}
