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
pub fn magma(_attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    wrap_and_impl(item, quote::quote! { Debug, Clone, PartialEq, ::proptest_derive::Arbitrary }, &[])
}

#[proc_macro_attribute]
pub fn semigroup(_attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    wrap_and_impl(item, quote::quote! { Debug, Clone, PartialEq, ::proptest_derive::Arbitrary }, &["Magma"])
}

#[proc_macro_attribute]
pub fn monoid(_attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    wrap_and_impl(item, quote::quote! { Default, Debug, Clone, PartialEq, ::proptest_derive::Arbitrary }, &["Magma", "Semigroup"])
}

#[proc_macro_attribute]
pub fn group(_attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    wrap_and_impl(item, quote::quote! { Default, Debug, Clone, PartialEq, ::proptest_derive::Arbitrary }, &["Magma", "Semigroup", "Monoid"])
}

#[proc_macro_attribute]
pub fn abeliangroup(_attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    wrap_and_impl(item, quote::quote! { Default, Debug, Clone, PartialEq, ::proptest_derive::Arbitrary }, &["Magma", "Semigroup", "Monoid", "Group"])
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
        let trait_path: syn::Path = syn::parse_str(t).expect("Invalid trait path");
        quote::quote! { 
            impl #impl_generics crate::#trait_path for #name #ty_generics #where_clause {} 
        }
    });

    let expanded = quote::quote! {
        #[derive(#derives)]
        #input
        #(#marker_impls)*
    };
    
    proc_macro::TokenStream::from(expanded)
}