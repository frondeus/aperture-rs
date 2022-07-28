extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote_spanned;
use syn::{parse_macro_input, spanned::Spanned, DeriveInput, Ident};
use itertools::Itertools;

#[proc_macro_derive(Lens)]
pub fn lens_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let main_name = input.ident;

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let (lenses, impls, ext): (Vec<_>, Vec<_>, Vec<_>) = match input.data {
        syn::Data::Struct(data) => match data.fields {
            syn::Fields::Named(fields) => {
                fields.named.into_iter().map(|f| {
                    let lower_name = f.ident.as_ref().unwrap();
                    let name_span = lower_name.span();
                    let name = lower_name.to_string();
                    let name = name[0..1].to_uppercase() + &name[1..];
                    let name = Ident::new(&format!("{main_name}{name}Lens"), name_span);
                    let ty = &f.ty;

                    let lens = quote_spanned! {f.span()=>

                        #[derive(Clone, Copy)]
                        pub struct #name;
                        impl Lens<AsLens, #main_name> for #name {
                            type View = #ty;
                            
                            fn impl_view(&self, source: #main_name) -> Self::View {
                                source.#lower_name
                            }
                            
                            fn impl_set<F: FnMut(Self::View) -> Self::View>(&self, mut source: #main_name, mut f: F) -> #main_name {
                                source.#lower_name = f(source.#lower_name);
                                source
                            }
                        }

                    };
                    
                    let then_name = quote::format_ident!("then_{lower_name}");
                    
                    let source_impl = quote_spanned!{f.span() =>
                        pub const #lower_name: #name = #name;
                    };
                    
                    let ext = quote_spanned!{f.span() =>
                        fn #then_name(self) -> And<Self, #name, (AsLens, AsLens), (S, #main_name)> {
                            self.then(#name)
                        }
                    };
                    
                    (lens, source_impl, ext)
                })
            }
            syn::Fields::Unnamed(_) => unreachable!("Cant derive lenses for unnamed struct"),
            syn::Fields::Unit => return TokenStream::default(),
        },
        syn::Data::Enum(_) | syn::Data::Union(_) => {
            unreachable!("Cant derive lenses for enums and unions (yet)")
        }
    }.multiunzip();
    
    let ext_name = quote::format_ident!("{main_name}LensesExt");

    TokenStream::from(quote::quote! {
        #(#lenses)*
        
        #[allow(non_upper_case_globals)]
        impl<#impl_generics> #main_name #ty_generics #where_clause {
            #(#impls)*
        }
        
        pub trait #ext_name<S>: Lens<AsLens, S> + Sized {
            #(#ext)*
        }
        impl <L, S> #ext_name<S> for L where L: Lens<AsLens, S, View = #main_name> {}
    })
}
