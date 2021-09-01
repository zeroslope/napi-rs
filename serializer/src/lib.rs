use napi_derive::*;
use once_cell::sync::OnceCell;
use rkyv::{
  ser::{serializers::AllocSerializer, Serializer},
  Archive, Deserialize, Serialize,
};
use serde_derive::{Deserialize as SerdeDeserialize, Serialize as SerdeSerialize};

static FIXTURE: OnceCell<User> = OnceCell::new();

#[module_exports]
fn init(mut exports: napi::JsObject) -> napi::Result<()> {
  FIXTURE.get_or_init(create_user);
  exports.create_named_method("serdeString", serialize_via_serde_string)?;
  exports.create_named_method("serdeBuffer", serialize_via_serde_buffer)?;
  exports.create_named_method("rkyv", serialize_via_rkyv)?;
  Ok(())
}

#[derive(Debug, Serialize, Deserialize, Archive, SerdeDeserialize, SerdeSerialize)]
pub struct User {
  id: i32,
  email: String,
  name: Option<String>,
  posts: Vec<Post>,
}

#[derive(Debug, Serialize, Deserialize, Archive, SerdeDeserialize, SerdeSerialize)]
pub struct Post {
  id: i32,
  title: String,
  content: Option<String>,
  published: bool,
  author_id: i32,
}

fn create_user() -> User {
  User {
    name: Some(format!("User-{}", 1)),
    id: 1,
    email: format!("user{}@napi.rs", 1),
    posts: (0..102400i32)
      .into_iter()
      .map(|i| Post {
        id: i,
        author_id: i,
        published: i % 3 == 0,
        title: format!("title-{}", i),
        content: Some(format!("content-{}", i)),
      })
      .collect(),
  }
}

#[js_function]
pub fn serialize_via_serde_string(ctx: napi::CallContext) -> napi::Result<napi::JsString> {
  let d = FIXTURE.get_or_init(create_user);
  let b = serde_json::to_string(d)?;
  ctx.env.create_string_from_std(b)
}

#[js_function]
pub fn serialize_via_serde_buffer(ctx: napi::CallContext) -> napi::Result<napi::JsBuffer> {
  let d = FIXTURE.get_or_init(create_user);
  let b = serde_json::to_vec(d)?;
  ctx.env.create_buffer_with_data(b).map(|b| b.into_raw())
}

#[js_function]
pub fn serialize_via_rkyv(ctx: napi::CallContext) -> napi::Result<napi::JsBuffer> {
  let d = FIXTURE.get_or_init(create_user);
  let mut serializer = AllocSerializer::<256>::default();
  serializer
    .serialize_value(d)
    .map_err(|e| napi::Error::new(napi::Status::InvalidArg, format!("{}", e)))?;
  ctx
    .env
    .create_buffer_with_data(serializer.into_serializer().into_inner().to_vec())
    .map(|b| b.into_raw())
}
