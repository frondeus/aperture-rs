use itertools::Itertools;
use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::{parse_quote, spanned::Spanned, DeriveInput, Ident};

pub fn prism_derive(input: DeriveInput) -> TokenStream {
    let main_name = input.ident;

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let (prisms, impls): (Vec<_>, Vec<_>) = match input.data {
        syn::Data::Enum(data) => data.variants.into_iter().map(|v| {
            let pascal_name = &v.ident;
            let name_span = pascal_name.span();
            let name = pascal_name.to_string();
            let lower_name = name[0..1].to_lowercase() + &name[1..];
            let lower_name = Ident::new(&lower_name, name_span);
            let name = Ident::new(&format!("{main_name}{pascal_name}Prism"), name_span);
            assert!(v.fields.len() <= 1);
            let field = v.fields.iter().next();
            let ty = field.map(|f| &f.ty);
            let variant_ty = ty.cloned().unwrap_or_else(|| parse_quote! { () });
            let preview = if ty.is_some() {
                quote!( #main_name::#pascal_name(t) => Some(t) )
            } else {
                quote!( #main_name::#pascal_name => Some(()))
            };
            let preview_ref = if ty.is_some() {
                quote!( #main_name::#pascal_name(t) => Some(t) )
            } else {
                quote!( #main_name::#pascal_name => Some(&()))
            };
            let review = if ty.is_some() {
                quote!( #main_name::#pascal_name(variant) )
            } else {
                quote!( #main_name::#pascal_name)
            };
            let set = if ty.is_some() {
                quote!( #main_name::#pascal_name(variant) => #main_name::#pascal_name(f(variant)), )
            } else {
                quote!()
            };
            let mut_set = if ty.is_some() {
                quote!( #main_name::#pascal_name(variant) => f(variant), )
            } else {
                quote!()
            };
            // let ty = &v.fields
            let prism = quote_spanned! { v.span() =>
                #[derive(Clone, Copy)]
                pub struct #name;
                #[allow(unused_variables, unused_mut)]
                impl Prism<AsPrism, #main_name> for #name {
                    type Variant = #variant_ty;

                    fn impl_preview(&self, source: #main_name) -> Option<Self::Variant> {
                        match source {
                            #preview,
                            _ => None
                        }
                    }

                    fn impl_review(&self, variant: Self::Variant) -> #main_name {
                        #review
                    }

                    fn impl_set<F>(&self, source: #main_name, mut f: F) -> #main_name
                    where F: Clone + FnMut(Self::Variant) -> Self::Variant {
                        match source {
                            #set
                            a => a
                        }
                    }
                 }

                #[allow(unused_variables, unused_mut)]
                 impl PrismMut<AsPrism, #main_name> for #name {
                     fn impl_set_mut<F>(&self, source: &mut #main_name, mut f: F)
                     where
                         F: Clone + FnMut(&mut Self::Variant),
                     {
                         match source {
                            #mut_set
                             _ => ()
                         };
                     }
                 }

                #[allow(unused_variables, unused_mut)]
                 impl PrismRef<AsPrism, #main_name> for #name {
                     fn impl_preview_ref<'a>(&self, source: &'a #main_name) -> Option<&'a Self::Variant> {
                         match source {
                             #preview_ref,
                             _ => None
                         }
                     }
                 }
            };

            let source_impl = quote_spanned! { v.span() =>
                pub const #lower_name: #name = #name;
            };
            (prism, source_impl)
        }),
        syn::Data::Struct(_) | syn::Data::Union(_) => {
            unreachable!("Cant derive prisms for structs and unions (yet)")
        }
    }
    .multiunzip();

    // let ext_name = quote::format_ident!("{main_name}PrismsExt");

    quote::quote! {
        #(#prisms)*

        #[allow(non_upper_case_globals)]
        impl<#impl_generics> #main_name #ty_generics #where_clause {
            #(#impls)*
        }

        // pub trait #ext_name<S>: PrismRef<AsLens, S> + Sized {
        //     #(#ext)*
        // }
        // // impl <L, S> #ext_name<S> for L where L: Lens<AsLens, S, View = #main_name> {}
        // impl <L, S> #ext_name<S> for L where L: PrismRef<AsLens, S, View = #main_name> {}
        // impl <L, S> #ext_name<S> for L where L: LensMut<AsLens, S, View = #main_name> {}
    }
}
