use std::{
  pin::Pin,
  task::{Context, Poll},
};

use futures::stream::Stream;

use crate::{Env, JsBuffer, JsFunction, JsObject, Result, Value};

#[repr(u8)]
enum StreamState {
  Polling,
  Complete,
  Error,
}

pub struct NodeStream<T: 'static> {
  pub(crate) js_value: Value,
  buf: Vec<T>,
  state: StreamState,
}

impl<T: 'static> NodeStream<T> {
  #[inline(always)]
  pub(crate) fn from_object(mut value: JsObject) -> Result<Self> {
    let on_func: JsFunction = value.get_named_property("on")?;
    let env = Env(value.0.env);
    let mut js_stream = Self {
      js_value: value.0,
      buf: vec![],
      state: StreamState::Polling,
    };
    env.wrap(&mut value, js_stream)?;
    on_func.call(
      Some(&value),
      &[
        env.create_string("data")?.into_unknown(),
        env
          .create_function_from_closure("OnData", |ctx| {
            ctx
              .get(0)
              .and_then(|value: JsBuffer| ctx.env.get_undefined())
          })?
          .into_unknown(),
      ],
    )?;
    Ok(js_stream)
  }
}

impl<T> Stream for NodeStream<T> {
  type Item = T;

  fn poll_next(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
    Poll::Pending
  }
}
