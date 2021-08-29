use napi_derive::*;
use once_cell::sync::OnceCell;
use rkyv::*;
use serde_derive::{Deserialize as SerdeDeserialize, Serialize as SerdeSerialize};

static FIXTURE: OnceCell<Vec<User>> = OnceCell::new();

#[module_exports]
fn init(mut exports: napi::JsObject) -> napi::Result<()> {
  FIXTURE.get_or_init(create_user);
  Ok(())
}

#[derive(Debug, Serialize, Deserialize, Archive, SerdeDeserialize, SerdeSerialize)]
pub struct User {
  id: i64,
  email: String,
  name: Option<String>,
  posts: Vec<Post>,
}

#[derive(Debug, Serialize, Deserialize, Archive, SerdeDeserialize, SerdeSerialize)]
pub struct Post {
  id: i64,
  title: String,
  content: Option<String>,
  published: bool,
  author_id: i64,
}

fn create_user() -> Vec<User> {
  (0..102400i64)
    .into_iter()
    .map(|i| User {
      name: Some(format!("User-{}", i)),
      id: i,
      email: format!("user{}@napi.rs", i),
      posts: vec![Post {
        id: i,
        author_id: i,
        published: i % 3 == 0,
        title: format!("title-{}", i),
        content: Some(format!("content-{}", i)),
      }],
    })
    .collect()
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
  let d = AlignedVec::new();
}
