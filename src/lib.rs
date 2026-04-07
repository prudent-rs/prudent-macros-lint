#![doc = include_str!("../README.md")]

use proc_macro::TokenStream;
use proc_macro_rules::rules;
use quote::quote_spanned;
use syn::spanned::Spanned;

#[cfg(not(debug_assertions))]
compile_error!("If you use prudent-macros-lint (usually through feature 'lint_unused_unsafe' of prudent crate), use it in debug build only.");

// Procedural version.
#[proc_macro]
pub fn unsafe_fn(input: TokenStream) -> TokenStream {
    rules!(input.into() => {
        ( $f:expr $( => $( $arg:expr ),+ )? ) => {

            let span = f.span();
            // @TODO Simplify once https://github.com/rust-lang/rust/issues/15701
            // `#![feature(stmt_expr_attributes)]` is stable
            //
            // See prudent-macros-enforce for why here I put in ({ ... }). But @TODO check if we
            // need these ({ and }).
            if let Some(arg) = arg {

                quote_spanned! {span=>
                    ({
                    #[deny(unused_unsafe)]
                    unsafe {
                        #f(
                            #(
                                #arg
                            ),*
                        )
                    }
                    })
                }
            } else {

                quote_spanned! {span=>
                    ({
                    #[deny(unused_unsafe)]
                    unsafe {
                        #f()
                    }
                })
            }
            }
        }
    })
    .into()
}

#[proc_macro]
pub fn unsafe_method(input: TokenStream) -> TokenStream {
    rules!(input.into() => {
        ( $this:expr =>. $method:ident ) => {

            let span = method.span();
            quote_spanned! {span=>
                ({
                #[deny(unused_unsafe)]
                unsafe {
                    #this.#method()
                }
                })
            }
        }

        ( $this:expr =>. $method:ident => $( $arg:expr ),* ) => {

            let span = method.span();
            quote_spanned! {span=>
                ({
                #[deny(unused_unsafe)]
                unsafe {
                    #this.#method(
                        #(
                            #arg
                        ),*
                    )
                }
                })
            }
        }
    })
    .into()
}

#[proc_macro]
pub fn unsafe_static_set(input: TokenStream) -> TokenStream {
    rules!(input.into() => {
        ($stat:path, $val:expr) => {
            
            quote::quote! {
                #[deny(unused_unsafe)]
                unsafe {
                    #stat = #val;
                }
            }
        }

        ($stat:ident { $( $suffix:tt )* } $val:expr) => {
            
            quote::quote! {}
        }
        ($stat:path { $( $suffix:tt )* } $val:expr) => {

            quote::quote! {
                #[deny(unused_unsafe)]
                unsafe {
                }
            }
        }
                
    })
    .into()
}

#[proc_macro]
pub fn unsafe_ref(input: TokenStream) -> TokenStream {
    rules!(input.into() => {
        ($ptr:expr) => {
            
            quote::quote! {
                #[deny(unused_unsafe)]
                unsafe {
                    &*#ptr
                }
            }
        }
        ($ptr:expr, $lifetime:lifetime) => {
            
            quote::quote! {
                #[deny(unused_unsafe)]
                unsafe {
                    &*#ptr as &#lifetime _
                }
            }
        }
        ($ptr:expr, $ptr_type:ty) => {
            
            quote::quote! {
                #[deny(unused_unsafe)]
                unsafe {
                    &*( #ptr as *const $ptr_type)
                }
            }
        }
        ($ptr:expr, $ptr_type:ty, $lifetime:lifetime) => {
            
            quote::quote! {
                #[deny(unused_unsafe)]
                unsafe {
                    &*( #ptr as *const $ty) as &$lifetime _
                }
            }
        }
    })
    .into()
}

#[proc_macro]
pub fn unsafe_mut(_input: TokenStream) -> TokenStream {
    (quote::quote! {}).into()
}

#[proc_macro]
pub fn unsafe_val(_input: TokenStream) -> TokenStream {
    (quote::quote! {}).into()
}

#[proc_macro]
pub fn unsafe_set(_input: TokenStream) -> TokenStream {
    (quote::quote! {}).into()
}
