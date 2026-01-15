#[proc_macro_attribute]
pub fn test_type_for_algebraic_laws(
    attr: proc_macro::TokenStream, 
    item: proc_macro::TokenStream
) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(item as syn::ItemImpl);

    let trait_name = attr.to_string().to_lowercase().trim().to_lowercase();
    let self_ty = &input.self_ty;
    let type_name = quote::quote!(#self_ty).to_string().to_lowercase()
    .replace(" ", "")
    .replace("<", "_")
    .replace(">", "_")
    .replace("::", "_");

    let test_module_name = quote::format_ident!("test_{}_{}_algebraic_laws", type_name, trait_name);


    let mut checks = quote::quote! {};

    match trait_name.as_str() {
        "semigroup" => {
            checks = quote::quote! {
                crate::traits::check_associativity(a.clone(), b.clone(), c.clone())
            };
        }
        "monoid" => {
            checks = quote::quote! {
                crate::traits::check_associativity(a.clone(), b.clone(), c.clone());
                crate::traits::check_identity(a.clone())
            };
        }
        "group" => {
            checks = quote::quote! {
                crate::traits::check_associativity(a.clone(), b.clone(), c.clone());
                crate::traits::check_identity(a.clone());
                crate::traits::check_inverse(a.clone())
            };
        }
        _ => panic!("The trait {} is not supported by verify_laws", trait_name)
    }


    let expanded = quote::quote! {
        #input

        #[cfg(test)]
        mod #test_module_name {
            use super::*;
            
            ::proptest::proptest! {

                #[test]
                fn verify_algebraic_laws(
                    a in ::proptest::prelude::any::<#self_ty>(),
                    b in ::proptest::prelude::any::<#self_ty>(),
                    c in ::proptest::prelude::any::<#self_ty>()
                ) {

                    #checks 
                }
            }
        }
    };


    proc_macro::TokenStream::from(expanded)
}
