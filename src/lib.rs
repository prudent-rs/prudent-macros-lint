#![doc = include_str!("../README.md")]

use proc_macro_rules::rules;

use proc_macro::TokenStream as ProcTokenStream;
use proc_macro2::TokenStream;

use quote::{quote_spanned, ToTokens, TokenStreamExt};
use std::str::FromStr;
use syn::spanned::Spanned;

#[cfg(not(debug_assertions))]
compile_error!("If you use prudent-macros-lint (usually through feature 'lint_unused_unsafe' of 'prudent' crate), use it with debug profile only.");

#[cfg(not(feature = "lint_unused_unsafe_all"))]
const POTENTIALLY_LINT_UNUSED_UNSAFE_ALL_CODE: &str = "";
#[cfg(feature = "lint_unused_unsafe_all")]
const POTENTIALLY_LINT_UNUSED_UNSAFE_ALL_CODE: &str = "#[forbid(unused_unsafe)]";

const POTENTIALLY_LINT_UNUSED_UNSAFE_ALL: TokenStreamFromStr<'static> =
    TokenStreamFromStr(POTENTIALLY_LINT_UNUSED_UNSAFE_ALL_CODE);

const ALLOW_UNSAFE_CODE: TokenStreamFromStr<'static> = TokenStreamFromStr("#[allow(unsafe_code)]");

struct TokenStreamFromStr<'a>(&'a str);
impl<'a> ToTokens for TokenStreamFromStr<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(TokenStream::from_str(self.0));
    }
}

#[proc_macro]
pub fn unsafe_fn(input: ProcTokenStream) -> ProcTokenStream {
    rules!(input.into() => {
        ( $f:expr ) => {
            // We HAVE TO use `span()` of an input token. We can NOT use
            // proc_macro2::Span::call_site() - it fails to trigger `unused_unsafe` lint when it
            // should.
            let span = f.span();

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
                    #ALLOW_UNSAFE_CODE
                    #POTENTIALLY_LINT_UNUSED_UNSAFE_ALL
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
                    #ALLOW_UNSAFE_CODE
                    #POTENTIALLY_LINT_UNUSED_UNSAFE_ALL
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
pub fn unsafe_method(input: ProcTokenStream) -> ProcTokenStream {
    rules!(input.into() => {
        ( $this:expr =>. $method:ident ) => {

            let span = method.span();
            quote_spanned! {span=>
                ({
                #ALLOW_UNSAFE_CODE
                #POTENTIALLY_LINT_UNUSED_UNSAFE_ALL
                unsafe {
                    #this.#method()
                }
                })
            }
        }

        ( $this:expr =>. $method:ident; $( $arg:expr ),* ) => {

            let span = method.span();
            quote_spanned! {span=>
                ({
                #ALLOW_UNSAFE_CODE
                #POTENTIALLY_LINT_UNUSED_UNSAFE_ALL
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
pub fn unsafe_static_set(input: ProcTokenStream) -> ProcTokenStream {
    rules!(input.into() => {
        ($stat:path, $val:expr) => {

            let span = stat.span();
            quote_spanned! {span=>
                #ALLOW_UNSAFE_CODE
                #POTENTIALLY_LINT_UNUSED_UNSAFE_ALL
                unsafe {
                    #stat = #val;
                }
            }
        }

        ($stat:ident { $( $_suffix:tt )* } $_val:expr) => {
            // @TODO
            let span = stat.span();
            quote_spanned! {span=>
                #ALLOW_UNSAFE_CODE
                #POTENTIALLY_LINT_UNUSED_UNSAFE_ALL
                unsafe {
                    // @TODO
                }
            }
        }
        ($stat:path { $( $_suffix:tt )* } $_val:expr) => {
            // @TODO
            let span = stat.span();
            quote_spanned! {span=>
                #ALLOW_UNSAFE_CODE
                #POTENTIALLY_LINT_UNUSED_UNSAFE_ALL
                unsafe {
                }
            }
        }

    })
    .into()
}

#[proc_macro]
pub fn unsafe_ref(input: ProcTokenStream) -> ProcTokenStream {
    rules!(input.into() => {
        ($ptr:expr) => {

            let span = ptr.span();
            quote_spanned! {span=>
                ({
                    #ALLOW_UNSAFE_CODE
                    #POTENTIALLY_LINT_UNUSED_UNSAFE_ALL
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
                    #ALLOW_UNSAFE_CODE
                    #POTENTIALLY_LINT_UNUSED_UNSAFE_ALL
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
                    #ALLOW_UNSAFE_CODE
                    #POTENTIALLY_LINT_UNUSED_UNSAFE_ALL
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
                    #ALLOW_UNSAFE_CODE
                    #POTENTIALLY_LINT_UNUSED_UNSAFE_ALL
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
pub fn unsafe_mut(input: ProcTokenStream) -> ProcTokenStream {
    rules!(input.into() => {
        ( $ptr:expr ) => {

            let span = ptr.span();
            quote_spanned! {span=>
                ({
                    #ALLOW_UNSAFE_CODE
                    #POTENTIALLY_LINT_UNUSED_UNSAFE_ALL
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
                    #ALLOW_UNSAFE_CODE
                    #POTENTIALLY_LINT_UNUSED_UNSAFE_ALL
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
                    #ALLOW_UNSAFE_CODE
                    #POTENTIALLY_LINT_UNUSED_UNSAFE_ALL
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
                    #ALLOW_UNSAFE_CODE
                    #POTENTIALLY_LINT_UNUSED_UNSAFE_ALL
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
pub fn unsafe_val(input: ProcTokenStream) -> ProcTokenStream {
    rules!(input.into() => {
        ( $ptr:expr ) => {

            let span = ptr.span();
            quote_spanned! {span=>
                ({
                    #ALLOW_UNSAFE_CODE
                    #POTENTIALLY_LINT_UNUSED_UNSAFE_ALL
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
                    #ALLOW_UNSAFE_CODE
                    #POTENTIALLY_LINT_UNUSED_UNSAFE_ALL
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
pub fn unsafe_set(input: ProcTokenStream) -> ProcTokenStream {
    rules!(input.into() => {
        ( $ptr:expr, $value:expr ) => {

            let span = ptr.span();
            // @TODO Simplify once https://github.com/rust-lang/rust/issues/15701
            // `#![feature(stmt_expr_attributes)]` is stable
            //
            // See prudent-macros-enforce for why here I put in ({ ... }). But @TODO check if we
            // need these ({ and }).
            quote_spanned! {span=>
                #ALLOW_UNSAFE_CODE
                #POTENTIALLY_LINT_UNUSED_UNSAFE_ALL
                unsafe {
                    *#ptr = #value;
                }
            }
        }
    })
    .into()
}
