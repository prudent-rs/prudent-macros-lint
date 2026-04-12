#![doc = include_str!("../README.md")]

use proc_macro::TokenStream;
use proc_macro_rules::rules;
use quote::quote_spanned;
use syn::spanned::Spanned;
use proc_macro2::Span;

#[cfg(not(debug_assertions))]
compile_error!("If you use prudent-macros-lint (usually through feature 'lint_unused_unsafe' of prudent crate), use it in debug build only.");

#[proc_macro]
pub fn unsafe_fn(input: TokenStream) -> TokenStream {
    rules!(input.into() => {
        ( $f:expr ) => {
            // Either of the following is fine: f.span() or Span::call_site()
            //
            // let span = f.span();
            let span = Span::call_site();

            // We HAVE TO use `quote::quote_spanned`. If we used `quote::quote` instead, any
            // `#[deny(unused_unsafe)]` or `#[forbid(unused_unsafe)]` on the user's side would have
            // no effect - the lint would not trigger, even though `quote::quote` docs say that it
            // uses caller site span for tokens written in it.

            // @TODO Simplify once https://github.com/rust-lang/rust/issues/15701
            // `#![feature(stmt_expr_attributes)]` is stable
            //
            // See prudent-macros-enforce for why here I put in ({ ... }). But @TODO check if we
            // need these ({ and }).
            quote_spanned! {span=>
                ({
                    #[allow(unsafe_code)]
                    unsafe {
                        #f()
                    }
                })
            }
        }
        ( $f:expr; $( $arg:expr ),+ ) => {

            let span = f.span();
            // @TODO Simplify once https://github.com/rust-lang/rust/issues/15701
            // `#![feature(stmt_expr_attributes)]` is stable
            //
            // See prudent-macros-enforce for why here I put in ({ ... }). But @TODO check if we
            // need these ({ and }).
            quote_spanned! {span=>
                ({
                    #[allow(unsafe_code)]
                    unsafe {
                        #f(
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
pub fn unsafe_method(input: TokenStream) -> TokenStream {
    rules!(input.into() => {
        ( $this:expr =>. $method:ident ) => {

            let span = method.span();
            quote_spanned! {span=>
                ({
                #[allow(unsafe_code)]
                unsafe {
                    #this.#method()
                }
                })
            }
        }

        ( $this:expr =>. $method:ident, $( $arg:expr ),* ) => {

            let span = method.span();
            quote_spanned! {span=>
                ({
                #[allow(unsafe_code)]
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

            let span = stat.span();
            quote_spanned! {span=>
                #[allow(unsafe_code)]
                unsafe {
                    #stat = #val;
                }
            }
        }

        ($stat:ident { $( $_suffix:tt )* } $_val:expr) => {
            // @TODO
            let span = stat.span();
            quote_spanned! {span=>
            }
        }
        ($stat:path { $( $_suffix:tt )* } $_val:expr) => {
            // @TODO
            let span = stat.span();
            quote_spanned! {span=>
                #[allow(unsafe_code)]
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

            let span = ptr.span();
            quote_spanned! {span=>
                ({
                    #[allow(unsafe_code)]
                    unsafe {
                        &*#ptr
                    }
                })
            }
        }
        ($ptr:expr, $lifetime:lifetime) => {

            let span = ptr.span();
            quote_spanned! {span=>
                ({
                    #[allow(unsafe_code)]
                    unsafe {
                        &*#ptr as &#lifetime _
                    }
                })
            }
        }
        ($ptr:expr, $ptr_type:ty) => {

            let span = ptr.span();
            quote_spanned! {span=>
                ({
                    #[allow(unsafe_code)]
                    unsafe {
                        &*( #ptr as *const #ptr_type)
                    }
                })
            }
        }
        ($ptr:expr, $ptr_type:ty, $lifetime:lifetime) => {

            let span = ptr.span();
            quote_spanned! {span=>
                ({
                    #[allow(unsafe_code)]
                    unsafe {
                        &*( #ptr as *const #ptr_type) as &#lifetime _
                    }
                })
            }
        }
    })
    .into()
}

#[proc_macro]
pub fn unsafe_mut(input: TokenStream) -> TokenStream {
    rules!(input.into() => {
        ( $ptr:expr ) => {

            let span = ptr.span();
            quote_spanned! {span=>
                ({
                    #[allow(unsafe_code)]
                    unsafe {
                        &mut *#ptr
                    }
                })
            }
        }
        ($ptr:expr, $lifetime:lifetime) => {

            let span = ptr.span();
            quote_spanned! {span=>
                ({
                    #[allow(unsafe_code)]
                    unsafe {
                        &mut *#ptr as &#lifetime mut _
                    }
                })
            }
        }
        ($ptr:expr, $ptr_type:ty) => {

            let span = ptr.span();
            quote_spanned! {span=>
                ({
                    #[allow(unsafe_code)]
                    unsafe {
                        &mut *( #ptr as *mut #ptr_type )
                    }
                })
            }
        }
        ($ptr:expr, $ptr_type:ty, $lifetime:lifetime) => {

            let span = ptr.span();
            quote_spanned! {span=>
                ({
                    #[allow(unsafe_code)]
                    unsafe {
                        &mut *( #ptr as *mut #ptr_type ) as &#lifetime mut _
                    }
                })
            }
        }
    })
    .into()
}

#[proc_macro]
pub fn unsafe_val(input: TokenStream) -> TokenStream {
    rules!(input.into() => {
        ( $ptr:expr ) => {

            let span = ptr.span();
            quote_spanned! {span=>
                ({
                    #[allow(unsafe_code)]
                    unsafe {
                        *#ptr
                    }
                })
            }
        }
        ( $ptr:expr => $ptr_type:ty ) => {

            let span = ptr.span();
            quote_spanned! {span=>
                ({
                    #[allow(unsafe_code)]
                    unsafe {
                        *( #ptr as *const #ptr_type)
                    }
                })
            }
        }
    })
    .into()
}

#[proc_macro]
pub fn unsafe_set(input: TokenStream) -> TokenStream {
    rules!(input.into() => {
        ( $ptr:expr, $value:expr ) => {

            let span = ptr.span();
            // @TODO Simplify once https://github.com/rust-lang/rust/issues/15701
            // `#![feature(stmt_expr_attributes)]` is stable
            //
            // See prudent-macros-enforce for why here I put in ({ ... }). But @TODO check if we
            // need these ({ and }).
            quote_spanned! {span=>
                #[allow(unsafe_code)]
                unsafe {
                    *#ptr = #value;
                }
            }
        }
    })
    .into()
}
