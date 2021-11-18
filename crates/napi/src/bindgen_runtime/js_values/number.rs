use super::{check_status, sys, Result};

macro_rules! impl_number_conversions {
	( $( ($name:literal, $t:ty, $get:ident, $create:ident) ,)* ) => {
		$(
      impl $crate::bindgen_prelude::TypeName for $t {
        #[inline(always)]
        fn type_name() -> &'static str {
          $name
        }

        fn value_type() -> crate::ValueType {
          crate::ValueType::Number
        }
      }

      impl $crate::bindgen_prelude::ValidateNapiValue for $t {
        #[inline(always)]
        fn type_of() -> Vec<$crate::ValueType> {
          vec![$crate::ValueType::Number]
        }
      }

      impl $crate::bindgen_prelude::ToNapiValue for $t {
        #[cfg(all(target_os = "windows", target_arch = "x86"))]
        unsafe fn to_napi_value(env: $crate::sys::napi_env, val: $t) -> Result<$crate::sys::napi_value> {
          let mut ptr = std::mem::MaybeUninit::uninit();
          let _ = sys::$create(env, val, ptr.as_mut_ptr());

          Ok(ptr.assume_init())
        }

        #[cfg(not(all(target_os = "windows", target_arch = "x86")))]
        unsafe fn to_napi_value(env: $crate::sys::napi_env, val: $t) -> Result<$crate::sys::napi_value> {
          let mut ptr = std::mem::MaybeUninit::uninit();
          check_status!(
            sys::$create(env, val, ptr.as_mut_ptr()),
						"Failed to convert rust type `{}` into napi value",
						$name,
          )?;

          Ok(ptr.assume_init())
        }
      }

      impl $crate::bindgen_prelude::FromNapiValue for $t {
        #[inline(always)]
				unsafe fn from_napi_value(env: $crate::sys::napi_env, napi_val: $crate::sys::napi_value) -> Result<Self> {
					let mut ret = 0 as $t;

          check_status!(
            sys::$get(env, napi_val, &mut ret),
						"Failed to convert napi value into rust type `{}`",
            $name
          )?;

					Ok(ret)
				}
      }
		)*
	};
}

impl_number_conversions!(
  ("u32", u32, napi_get_value_uint32, napi_create_uint32),
  ("i32", i32, napi_get_value_int32, napi_create_int32),
  ("i64", i64, napi_get_value_int64, napi_create_int64),
  ("f64", f64, napi_get_value_double, napi_create_double),
);
