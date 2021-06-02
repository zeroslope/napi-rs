use std::ffi::c_void;
use std::pin::Pin;
use std::thread::spawn;
use std::time::Duration;

use futures::future::Future;
use once_cell::sync::OnceCell;
use tokio::runtime::Runtime;
use tokio::sync::mpsc::{unbounded_channel, UnboundedSender};

use crate::Error;

pub(crate) enum Message {
  Task(Pin<Box<dyn Future<Output = ()> + Send>>),
  Shutdown,
}

static SENDER: OnceCell<UnboundedSender<Message>> = OnceCell::new();

#[inline]
pub(crate) fn get_tokio_sender() -> &'static UnboundedSender<Message> {
  SENDER.get_or_init(|| {
    let (sender, mut receiver) = unbounded_channel();
    spawn(move || {
      let rt = Runtime::new().expect("Failed to create tokio runtime");
      rt.block_on(async {
        loop {
          match receiver.recv().await {
            Some(Message::Task(fut)) => fut.await,
            Some(Message::Shutdown) => break,
            None => {}
          }
        }
      });
      rt.shutdown_timeout(Duration::from_secs(5));
    });

    sender
  })
}

#[doc(hidden)]
pub unsafe extern "C" fn shutdown(_data: *mut c_void) {
  let sender = get_tokio_sender().clone();
  sender
    .send(Message::Shutdown)
    .map_err(|e| Error::from_reason(format!("Shutdown tokio runtime failed: {}", e)))
    .unwrap()
}
