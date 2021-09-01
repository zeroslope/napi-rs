use core::convert::Into;
use core::iter::FromIterator;

use rkyv::*;
use wasm_bindgen::prelude::*;

#[derive(Debug, Deserialize, Archive)]
pub struct User {
  pub id: i32,
  pub email: String,
  pub name: Option<String>,
  pub posts: Vec<Post>,
}

#[derive(Debug, Deserialize, Archive)]
pub struct Post {
  pub id: i32,
  pub title: String,
  pub content: Option<String>,
  pub published: bool,
  pub author_id: i32,
}

#[wasm_bindgen]
pub fn serialize(data: &[u8]) -> js_sys::Object {
  let archived = unsafe { archived_root::<User>(&data) };
  let deserialized: User = archived.deserialize(&mut Infallible).unwrap();
  let o = js_sys::Object::new();
  let posts = js_sys::Array::from_iter(deserialized.posts.into_iter().map(|v| {
    let p = js_sys::Object::new();
    unsafe {
      js_sys::Reflect::set(&p, &"id".into(), &v.id.into()).unwrap();
      js_sys::Reflect::set(&p, &"title".into(), &v.title.into()).unwrap();
      js_sys::Reflect::set(&p, &"content".into(), &v.content.unwrap().into()).unwrap();
      js_sys::Reflect::set(&p, &"published".into(), &v.published.into()).unwrap();
      js_sys::Reflect::set(&p, &"author_id".into(), &v.author_id.into()).unwrap();
    }
    p
  }));
  unsafe {
    js_sys::Reflect::set(&o, &"id".into(), &deserialized.id.into()).unwrap();
    js_sys::Reflect::set(&o, &"email".into(), &deserialized.email.into()).unwrap();
    js_sys::Reflect::set(&o, &"name".into(), &deserialized.name.unwrap().into()).unwrap();
    js_sys::Reflect::set(&o, &"posts".into(), &posts.into()).unwrap();
  };
  o
}
