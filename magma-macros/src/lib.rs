// ALGEBRAIC LAW TESTS BROKEN

// #[proc_macro_attribute]
// pub fn test_type_for_algebraic_laws(
//     attr: proc_macro::TokenStream, 
//     item: proc_macro::TokenStream
// ) -> proc_macro::TokenStream {
//     let input = syn::parse_macro_input!(item as syn::ItemImpl);

//     let trait_name = attr.to_string().to_lowercase().trim().to_lowercase();
//     let self_ty = &input.self_ty;
//     let type_name = quote::quote!(#self_ty).to_string().to_lowercase()
//     .replace(" ", "")
//     .replace("<", "_")
//     .replace(">", "_")
//     .replace("::", "_");

//     let test_module_name = quote::format_ident!("test_{}_{}_algebraic_laws", type_name, trait_name);


//     let mut checks = quote::quote! {};

//     match trait_name.as_str() {
//         "semigroup" => {
//             checks = quote::quote! {
//                 crate::traits::check_associativity(a.clone(), b.clone(), c.clone())
//             };
//         }
//         "monoid" => {
//             checks = quote::quote! {
//                 crate::traits::check_associativity(a.clone(), b.clone(), c.clone());
//                 crate::traits::check_identity(a.clone())
//             };
//         }
//         "group" => {
//             checks = quote::quote! {
//                 crate::traits::check_associativity(a.clone(), b.clone(), c.clone());
//                 crate::traits::check_identity(a.clone());
//                 crate::traits::check_inverse(a.clone())
//             };
//         }
//         _ => panic!("The trait {} is not supported by verify_laws", trait_name)
//     }

//     let expanded = quote::quote! {
//         #input

//         #[cfg(test)]
//         mod #test_module_name {
//             use super::*;
            
//             ::proptest::proptest! {

//                 #[test]
//                 fn verify_algebraic_laws(
//                     a in ::proptest::prelude::any::<#self_ty>(),
//                     b in ::proptest::prelude::any::<#self_ty>(),
//                     c in ::proptest::prelude::any::<#self_ty>()
//                 ) {

//                     #checks 
//                 }
//             }
//         }
//     };

//     proc_macro::TokenStream::from(expanded)
// }


// Macros for required derivations and implementations

#[proc_macro_attribute]
pub fn addmagma(_attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    wrap_and_impl(item, quote::quote! { Debug, Clone, PartialEq, ::proptest_derive::Arbitrary }, &[])
}
#[proc_macro_attribute]
pub fn multmagma(_attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    wrap_and_impl(item, quote::quote! { Debug, Clone, PartialEq, ::proptest_derive::Arbitrary }, &[])
}

#[proc_macro_attribute]
pub fn addsemigroup(_attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    wrap_and_impl(item, quote::quote! { Debug, Clone, PartialEq, ::proptest_derive::Arbitrary }, &["AddMagma"])
}
#[proc_macro_attribute]
pub fn multsemigroup(_attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    wrap_and_impl(item, quote::quote! { Debug, Clone, PartialEq, ::proptest_derive::Arbitrary }, &["MultMagma"])
}

#[proc_macro_attribute]
pub fn addmonoid(_attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    wrap_and_impl(item, quote::quote! { Debug, Clone, PartialEq, ::proptest_derive::Arbitrary }, &["AddMagma", "AddSemigroup"])
}
#[proc_macro_attribute]
pub fn multmonoid(_attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    wrap_and_impl(item, quote::quote! { Debug, Clone, PartialEq, ::proptest_derive::Arbitrary }, &["MultMagma", "MultSemigroup"])
}

#[proc_macro_attribute]
pub fn addgroup(_attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    wrap_and_impl(item, quote::quote! { Debug, Clone, PartialEq, ::proptest_derive::Arbitrary }, &["AddMagma", "AddSemigroup", "AddMonoid"])
}
#[proc_macro_attribute]
pub fn multgroup(_attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    wrap_and_impl(item, quote::quote! { Debug, Clone, PartialEq, ::proptest_derive::Arbitrary }, &["MultMagma", "MultSemigroup", "MultMonoid"])
}

#[proc_macro_attribute]
pub fn addabeliangroup(_attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    wrap_and_impl(item, quote::quote! { Debug, Clone, PartialEq, ::proptest_derive::Arbitrary }, &["AddMagma", "AddSemigroup", "AddMonoid", "AddGroup"])
}
#[proc_macro_attribute]
pub fn multabeliangroup(_attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    wrap_and_impl(item, quote::quote! { Debug, Clone, PartialEq, ::proptest_derive::Arbitrary }, &["MultMagma", "MultSemigroup", "MultMonoid", "MultGroup"])
}


// Internal helper function

fn wrap_and_impl(
    item: proc_macro::TokenStream, 
    derives: proc_macro2::TokenStream, 
    marker_traits: &[&str] 
) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(item as syn::ItemStruct);
    let name = &input.ident;
    
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let marker_impls = marker_traits.iter().map(|t| {
        let trait_ident = quote::format_ident!("{}", t); 
        quote::quote! { 
            impl #impl_generics crate::#trait_ident for #name #ty_generics #where_clause {} 
        }
    });

    let expanded = quote::quote! {
        #[derive(#derives)]
        #input
        
        impl #impl_generics Default for #name #ty_generics #where_clause {
            fn default() -> Self {
                Self(0) 
            }
        }

        #(#marker_impls)*
    };
    
    proc_macro::TokenStream::from(expanded)
}