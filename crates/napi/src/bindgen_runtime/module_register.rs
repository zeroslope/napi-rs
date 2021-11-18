use std::{cell::RefCell, collections::HashMap, ptr};

use crate::{check_status, check_status_or_throw, sys, JsError, Property, Result};

pub type ExportRegisterCallback = unsafe fn(sys::napi_env) -> sys::napi_value;
pub type ModuleExportsCallback =
  unsafe fn(env: sys::napi_env, exports: sys::napi_value) -> Result<()>;

thread_local! {
  static MODULE_REGISTER_CALLBACK: RefCell<Vec<(&'static str, ExportRegisterCallback)>> = Default::default();
  static MODULE_CLASS_PROPERTIES: RefCell<HashMap<&'static str, (&'static str, Vec<Property>)>> = Default::default();
  static REGISTERED_CLASSES: RefCell<HashMap<
    /* export name */ &'static str,
    /* constructor */ sys::napi_ref,
  >> = Default::default();
  // compatibility for #[module_exports]
  #[cfg(feature = "compat-mode")]
  static MODULE_EXPORTS: std::cell::Cell<Vec<ModuleExportsCallback>> = Default::default();
}

pub fn get_class_constructor(js_name: &'static str) -> Option<sys::napi_ref> {
  REGISTERED_CLASSES.with(|registered_classes| {
    let classes = registered_classes.borrow();
    classes.get(js_name).copied()
  })
}

#[cfg(feature = "compat-mode")]
// compatibility for #[module_exports]
pub fn register_module_exports(callback: ModuleExportsCallback) {
  MODULE_EXPORTS.with(|cell| cell.set(vec![callback]));
}

pub fn register_module_export(name: &'static str, cb: ExportRegisterCallback) {
  MODULE_REGISTER_CALLBACK.with(|exports| {
    let mut list = exports.borrow_mut();
    list.push((name, cb));
  });
}

pub fn register_class(rust_name: &'static str, js_name: &'static str, props: Vec<Property>) {
  MODULE_CLASS_PROPERTIES.with(|map| {
    let mut map = map.borrow_mut();
    let val = map.entry(rust_name).or_default();

    val.0 = js_name;
    val.1.extend(props.into_iter());
  });
}

#[no_mangle]
pub unsafe extern "C" fn napi_register_module_v1(
  env: sys::napi_env,
  exports: sys::napi_value,
) -> sys::napi_value {
  MODULE_REGISTER_CALLBACK.with(|to_register_exports| {
    for (name, callback) in to_register_exports.borrow().iter() {
      unsafe {
        check_status_or_throw!(
          env,
          sys::napi_set_named_property(env, exports, name.as_ptr() as *const _, callback(env)),
          "Set exports failed [{}]",
          name
        );
      }
    }
  });

  MODULE_CLASS_PROPERTIES.with(|to_register_classes| {
    for (rust_name, (js_name, props)) in to_register_classes.borrow().iter() {
      unsafe {
        let (ctor, props): (Vec<_>, Vec<_>) = props.iter().partition(|prop| prop.is_ctor);
        // one or more or zero?
        // zero is for `#[napi(task)]`
        if ctor.is_empty() && props.is_empty() {
          continue;
        }
        let ctor = ctor.get(0).and_then(|c| c.raw().method).unwrap_or(noop);
        let raw_props: Vec<_> = props.iter().map(|prop| prop.raw()).collect();

        let mut class_ptr = std::mem::MaybeUninit::uninit();
        let js_name_c_str = std::ffi::CStr::from_bytes_with_nul_unchecked(js_name.as_bytes());
        check_status_or_throw!(
          env,
          sys::napi_define_class(
            env,
            js_name_c_str.as_ptr(),
            js_name.len() as sys::size_t - 1,
            Some(ctor),
            ptr::null_mut(),
            raw_props.len() as sys::size_t,
            raw_props.as_ptr(),
            class_ptr.as_mut_ptr(),
          ),
          "Failed to register class `{}` generate by struct `{}`",
          &js_name,
          &rust_name
        );
        let class_ptr = class_ptr.assume_init();
        let mut ctor_ref = std::mem::MaybeUninit::uninit();
        sys::napi_create_reference(env, class_ptr, 1, ctor_ref.as_mut_ptr());
        let ctor_ref = ctor_ref.assume_init();
        REGISTERED_CLASSES.with(|registered_classes| {
          let mut registered_class = registered_classes.borrow_mut();
          registered_class.insert(js_name, ctor_ref);
        });

        check_status_or_throw!(
          env,
          sys::napi_set_named_property(env, exports, js_name_c_str.as_ptr(), class_ptr),
          "Failed to register class `{}` generate by struct `{}`",
          &js_name,
          &rust_name
        );
      }
    }
  });

  #[cfg(feature = "compat-mode")]
  MODULE_EXPORTS.with(|callbacks| {
    for callback in callbacks.take().into_iter() {
      if let Err(e) = callback(env, exports) {
        JsError::from(e).throw_into(env);
      }
    }
  });

  #[cfg(all(feature = "tokio_rt", feature = "napi4"))]
  if let Err(e) = check_status!(
    sys::napi_add_env_cleanup_hook(env, Some(crate::shutdown_tokio_rt), ptr::null_mut()),
    "Failed to initialize module",
  ) {
    JsError::from(e).throw_into(env);
  }

  exports
}

pub(crate) unsafe extern "C" fn noop(
  _env: sys::napi_env,
  _info: sys::napi_callback_info,
) -> sys::napi_value {
  ptr::null_mut()
}
