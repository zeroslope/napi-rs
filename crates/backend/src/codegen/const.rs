use proc_macro2::{Ident, Literal, Span, TokenStream};
use quote::ToTokens;

use crate::{codegen::get_register_ident, BindgenResult, NapiConst, TryToTokens};

impl TryToTokens for NapiConst {
  fn try_to_tokens(&self, tokens: &mut TokenStream) -> BindgenResult<()> {
    let register = self.gen_module_register();
    (quote! {
      #register
    })
    .to_tokens(tokens);

    Ok(())
  }
}

impl NapiConst {
  fn gen_module_register(&self) -> TokenStream {
    let name_str = self.name.to_string();
    let name_ident = self.name.clone();
    let js_name_lit = Literal::string(format!("{}\0", self.js_name).as_str());
    let register_name = get_register_ident(&name_str);
    let register_cb = Ident::new(
      &format!("__napi__const__{}_register_cb", name_str),
      Span::call_site(),
    );
    quote! {
      #[inline(never)]
      unsafe fn #register_cb(env: napi::sys::napi_env) -> napi::sys::napi_value {
        println!("registering {}, env: {:p}", #name_str, env);
        napi::bindgen_prelude::ToNapiValue::to_napi_value(env, #name_ident).expect(format!("Create JsValue from const failed {}", #js_name_lit).as_str())
      }
      #[allow(non_snake_case)]
      #[allow(clippy::all)]
      #[napi::bindgen_prelude::ctor]
      fn #register_name() {
        napi::bindgen_prelude::register_module_export(#js_name_lit, #register_cb);
      }
    }
  }
}
