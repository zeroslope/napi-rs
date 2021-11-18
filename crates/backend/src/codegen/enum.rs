use proc_macro2::{Ident, Literal, Span, TokenStream};
use quote::ToTokens;

use crate::{codegen::get_register_ident, BindgenResult, NapiEnum, TryToTokens};

impl TryToTokens for NapiEnum {
  fn try_to_tokens(&self, tokens: &mut TokenStream) -> BindgenResult<()> {
    let register = self.gen_module_register();
    let napi_value_conversion = self.gen_napi_value_map_impl();

    (quote! {
      #napi_value_conversion
      #register
    })
    .to_tokens(tokens);

    Ok(())
  }
}

impl NapiEnum {
  fn gen_napi_value_map_impl(&self) -> TokenStream {
    let name = &self.name;
    let name_str = self.name.to_string();
    let mut from_napi_branches = vec![];
    let mut to_napi_branches = vec![];

    self.variants.iter().for_each(|v| {
      let val = Literal::i32_unsuffixed(v.val);
      let v_name = &v.name;

      from_napi_branches.push(quote! { #val => Ok(#name::#v_name) });
      to_napi_branches.push(quote! { #name::#v_name => #val });
    });

    quote! {
      impl napi::bindgen_prelude::TypeName for #name {
        fn type_name() -> &'static str {
          #name_str
        }

        fn value_type() -> napi::ValueType {
          napi::ValueType::Object
        }
      }

      impl napi::bindgen_prelude::ValidateNapiValue for #name {
        unsafe fn validate(
          env: napi::sys::napi_env,
          napi_val: napi::sys::napi_value
        ) -> napi::bindgen_prelude::Result<()> {
          napi::bindgen_prelude::assert_type_of!(env, napi_val, napi::bindgen_prelude::ValueType::Number)
        }
      }

      impl napi::bindgen_prelude::FromNapiValue for #name {
        unsafe fn from_napi_value(
          env: napi::sys::napi_env,
          napi_val: napi::sys::napi_value
        ) -> napi::bindgen_prelude::Result<Self> {
          let val = i32::from_napi_value(env, napi_val).map_err(|e| {
            napi::bindgen_prelude::error!(
              e.status,
              "Failed to convert napi value into enum `{}`. {}",
              #name_str,
              e,
            )
          })?;

          match val {
            #(#from_napi_branches,)*
            _ => {
              Err(napi::bindgen_prelude::error!(
                napi::bindgen_prelude::Status::InvalidArg,
                "value `{}` does not match any variant of enum `{}`",
                val,
                #name_str
              ))
            }
          }
        }
      }

      impl napi::bindgen_prelude::ToNapiValue for #name {
        unsafe fn to_napi_value(
          env: napi::sys::napi_env,
          val: Self
        ) -> napi::bindgen_prelude::Result<napi::sys::napi_value> {
          let val = match val {
            #(#to_napi_branches,)*
          };

          i32::to_napi_value(env, val)
        }
      }
    }
  }

  fn gen_module_register(&self) -> TokenStream {
    let name_str = self.name.to_string();
    let js_name_lit = Literal::string(format!("{}\0", self.js_name).as_str());
    let register_name = get_register_ident(&name_str);

    let mut define_properties = vec![];

    for (index, variant) in self.variants.iter().enumerate() {
      let name_lit = Literal::string(format!("{}\0", &variant.name.to_string()).as_str());
      let val_lit = Literal::i32_unsuffixed(variant.val);
      let val_ident = Ident::new(format!("val_{}", index).as_str(), Span::call_site());
      let enum_value_ident =
        Ident::new(format!("enum_value_{}", index).as_str(), Span::call_site());

      define_properties.push(quote! {
        println!("enum e, env {}, val, {} {:p}", #index, #val_lit, env);
        let mut #val_ident = std::mem::MaybeUninit::uninit();
        napi::sys::napi_create_int32(env, #val_lit, #val_ident.as_mut_ptr());
        println!("enum e, {:p}", env);
        let #enum_value_ident = #val_ident.assume_init();
        let name_lit_c_str = std::ffi::CStr::from_bytes_with_nul_unchecked(#name_lit.as_bytes());
        napi::sys::napi_set_named_property(env, obj_ptr, name_lit_c_str.as_ptr(), #enum_value_ident);
        println!("enum e, {:p}", env);
      })
    }

    quote! {
      #[allow(non_snake_case)]
      #[allow(clippy::all)]
      #[napi::bindgen_prelude::ctor]
      fn #register_name() {
        #[inline(never)]
        unsafe fn cb(env: napi::sys::napi_env) -> napi::sys::napi_value {
          let mut obj_ptr = std::mem::MaybeUninit::uninit();

          napi::sys::napi_create_object(env, obj_ptr.as_mut_ptr());
          println!("enum begin, env: {:p}", env);
          let obj_ptr = obj_ptr.assume_init();
          // #(#define_properties)*

          obj_ptr

        }

        napi::bindgen_prelude::register_module_export(#js_name_lit, cb);
      }
    }
  }
}
