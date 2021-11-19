use proc_macro2::{Ident, Span, TokenStream};
use quote::ToTokens;

use crate::{
  codegen::{get_intermediate_ident, get_register_ident},
  BindgenResult, CallbackArg, FnKind, FnSelf, NapiFn, NapiFnArgKind, TryToTokens,
};

impl TryToTokens for NapiFn {
  fn try_to_tokens(&self, tokens: &mut TokenStream) -> BindgenResult<()> {
    let name_str = self.name.to_string();
    let intermediate_ident = get_intermediate_ident(&name_str);
    let args_len = self.args.len();

    let (arg_conversions, arg_names) = self.gen_arg_conversions();
    let receiver = self.gen_fn_receiver();
    let receiver_ret_name = Ident::new("_ret", Span::call_site());
    let ret = self.gen_fn_return(&receiver_ret_name);
    let register = self.gen_fn_register();
    let attrs = &self.attrs;

    let native_call = if !self.is_async {
      quote! {
        let #receiver_ret_name = {
          #receiver(#(#arg_names),*)
        };
        #ret
      }
    } else {
      let call = if self.is_ret_result {
        quote! { #receiver(#(#arg_names),*).await }
      } else {
        quote! { Ok(#receiver(#(#arg_names),*).await) }
      };
      quote! {
        napi::bindgen_prelude::execute_tokio_future(env, async move { #call }, |env, #receiver_ret_name| {
          #ret
        })
      }
    };

    let function_call = if args_len == 0
      && self.fn_self.is_none()
      && self.kind != FnKind::Constructor
      && self.kind != FnKind::Factory
    {
      quote! { #native_call }
    } else if self.kind == FnKind::Constructor {
      quote! {
        let call_from_factory = napi::bindgen_prelude::___CALL_FROM_FACTORY.load(std::sync::atomic::Ordering::Relaxed);
        // constructor function is called from class `factory`
        // so we should skip the original `constructor` logic
        if call_from_factory {
          return std::ptr::null_mut();
        }
        let cb = napi::bindgen_prelude::CallbackInfo::<#args_len>::new(env, cb, None);
        #(#arg_conversions)*
        #native_call
      }
    } else {
      quote! {
        let mut cb = napi::bindgen_prelude::CallbackInfo::<#args_len>::new(env, cb, None);
        #(#arg_conversions)*
        #native_call
      }
    };

    (quote! {
      #(#attrs)*
      #[doc(hidden)]
      #[allow(non_snake_case)]
      #[allow(clippy::all)]
      pub extern "C" fn #intermediate_ident(
        env: napi::sys::napi_env,
        cb: napi::sys::napi_callback_info
      ) -> napi::sys::napi_value {
        unsafe {
          match { #function_call } {
            Ok(v) => v,
            Err(e) => {
              napi::bindgen_prelude::JsError::from(e).throw_into(env);
              std::ptr::null_mut()
            }
          }
        }
      }

      #register
    })
    .to_tokens(tokens);

    Ok(())
  }
}

impl NapiFn {
  fn gen_arg_conversions(&self) -> (Vec<TokenStream>, Vec<TokenStream>) {
    let mut arg_conversions = vec![];
    let mut args = vec![];

    // fetch this
    if let Some(parent) = &self.parent {
      match self.fn_self {
        Some(FnSelf::Ref) => {
          arg_conversions.push(quote! { let this = cb.unwrap_borrow::<#parent>(); });
        }
        Some(FnSelf::MutRef) => {
          arg_conversions.push(quote! { let this = cb.unwrap_borrow_mut::<#parent>(); });
        }
        _ => {}
      };
    }

    let mut skipped_arg_count = 0;
    self.args.iter().enumerate().for_each(|(i, arg)| {
      let i = i - skipped_arg_count;
      let ident = Ident::new(&format!("arg{}", i), Span::call_site());

      match arg {
        NapiFnArgKind::PatType(path) => {
          if &path.ty.to_token_stream().to_string() == "Env" {
            args.push(quote! { napi::bindgen_prelude::Env::from(env) });
            skipped_arg_count += 1;
          } else {
            arg_conversions.push(self.gen_ty_arg_conversion(&ident, i, path));
            args.push(quote! { #ident });
          }
        }
        NapiFnArgKind::Callback(cb) => {
          arg_conversions.push(self.gen_cb_arg_conversion(&ident, i, cb));
          args.push(quote! { #ident });
        }
      }
    });

    (arg_conversions, args)
  }

  fn gen_ty_arg_conversion(
    &self,
    arg_name: &Ident,
    index: usize,
    path: &syn::PatType,
  ) -> TokenStream {
    let ty = &*path.ty;
    match ty {
      syn::Type::Reference(syn::TypeReference {
        mutability: Some(_),
        elem,
        ..
      }) => {
        quote! {
          let #arg_name = <#elem as napi::bindgen_prelude::FromNapiMutRef>::from_napi_mut_ref(env, cb.get_arg(#index));
        }
      }
      syn::Type::Reference(syn::TypeReference { elem, .. }) => {
        quote! {
          let #arg_name = <#elem as napi::bindgen_prelude::FromNapiRef>::from_napi_ref(env, cb.get_arg(#index));
        }
      }
      _ => {
        let type_check = if self.strict {
          quote! {
            <#ty as napi::bindgen_prelude::ValidateNapiValue>::validate(env, cb.get_arg(#index));
          }
        } else {
          quote! {}
        };

        quote! {
          #type_check
          let #arg_name = <#ty as napi::bindgen_prelude::FromNapiValue>::from_napi_value(env, cb.get_arg(#index));
        }
      }
    }
  }

  fn gen_cb_arg_conversion(&self, arg_name: &Ident, index: usize, cb: &CallbackArg) -> TokenStream {
    let mut inputs = vec![];
    let mut arg_conversions = vec![];

    for (i, ty) in cb.args.iter().enumerate() {
      let cb_arg_ident = Ident::new(&format!("callback_arg_{}", i), Span::call_site());
      inputs.push(quote! { #cb_arg_ident: #ty });
      arg_conversions.push(
        quote! { <#ty as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, #cb_arg_ident)? },
      );
    }

    let ret = match &cb.ret {
      Some(ty) => {
        quote! {
          let ret = <#ty as napi::bindgen_prelude::FromNapiValue>::from_napi_value(env, ret_ptr);

          Ok(ret)
        }
      }
      None => quote! { Ok(()) },
    };

    quote! {
      napi::bindgen_prelude::assert_type_of!(env, cb.get_arg(#index), napi::bindgen_prelude::ValueType::Function);
      let #arg_name = |#(#inputs),*| {
        let args = vec![
          #(#arg_conversions),*
        ];

        let mut ret_ptr = std::ptr::null_mut();

        napi::bindgen_prelude::check_status_or_throw!(
          env,
          napi::sys::napi_call_function(
            env,
            cb.this(),
            cb.get_arg(#index),
            args.len() as napi::sys::size_t,
            args.as_ptr(),
            &mut ret_ptr
          ),
          "Failed to call napi callback",
        );

        #ret
      };
    }
  }

  fn gen_fn_receiver(&self) -> TokenStream {
    let name = &self.name;

    match self.fn_self {
      Some(FnSelf::Value) => {
        // impossible, panic! in parser
        unimplemented!();
      }
      Some(FnSelf::Ref) | Some(FnSelf::MutRef) => quote! { this.#name },
      None => match &self.parent {
        Some(class) => quote! { #class::#name },
        None => quote! { #name },
      },
    }
  }

  fn gen_fn_return(&self, ret: &Ident) -> TokenStream {
    let js_name = &self.js_name;

    if let Some(ty) = &self.ret {
      if self.kind == FnKind::Constructor {
        if self.is_ret_result {
          quote! { #ret.and_then(|ret| cb.construct(#js_name, ret)) }
        } else {
          quote! { cb.construct(#js_name, #ret) }
        }
      } else if self.kind == FnKind::Factory {
        if self.is_ret_result {
          quote! { #ret.and_then(|ret| cb.factory(#js_name, ret)) }
        } else {
          quote! { cb.factory(#js_name, #ret) }
        }
      } else if self.is_ret_result {
        if self.is_async {
          quote! {
            <#ty as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, #ret)
          }
        } else {
          quote! {
            match #ret {
              Ok(value) => napi::bindgen_prelude::ToNapiValue::to_napi_value(env, value),
              Err(err) => {
                napi::bindgen_prelude::JsError::from(err).throw_into(env);
                Ok(std::ptr::null_mut())
              },
            }
          }
        }
      } else {
        quote! {
          <#ty as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, #ret)
        }
      }
    } else {
      quote! {
        <() as napi::bindgen_prelude::ToNapiValue>::to_napi_value(env, ())
      }
    }
  }

  fn gen_fn_register(&self) -> TokenStream {
    if self.parent.is_some() {
      quote! {}
    } else {
      let name_str = self.name.to_string();
      let js_name = format!("{}\0", self.js_name);
      let js_name_len = js_name.len();
      let module_register_name = get_register_ident(&name_str);
      let intermediate_ident = get_intermediate_ident(&name_str);
      let cb_ident = Ident::new(
        &format!("__napi_fn_{}_register_cb", name_str),
        Span::call_site(),
      );

      quote! {
        #[inline(never)]
        unsafe fn #cb_ident(env: napi::sys::napi_env) -> napi::sys::napi_value {
          let mut fn_ptr = std::mem::MaybeUninit::<napi::sys::napi_value>::uninit();
          let js_name_c_string = std::ffi::CStr::from_bytes_with_nul_unchecked(#js_name.as_bytes());
          napi::bindgen_prelude::check_status_or_throw!(
            env,
            napi::sys::napi_create_function(
              env,
              js_name_c_string.as_ptr(),
              #js_name_len as napi::sys::size_t,
              Some(#intermediate_ident),
              std::ptr::null_mut(),
              fn_ptr.as_mut_ptr(),
            ),
            "Failed to register function `{}`",
            #name_str,
          );

          fn_ptr.assume_init()
        }
        #[allow(clippy::all)]
        #[allow(non_snake_case)]
        #[napi::bindgen_prelude::ctor]
        fn #module_register_name() {
          napi::bindgen_prelude::register_module_export(#js_name, #cb_ident);
        }
      }
    }
  }
}
