use std::sync::mpsc::channel;
use std::thread;
use tokio::runtime::Runtime;
use futures::Future;

pub fn make_sync<F: Future<Output=R> + Send + 'static, R: Send + 'static>(future: F) -> R {
    let (sender, receiver) = channel();
    thread::spawn(move || {
        let mut runtime = Runtime::new().unwrap();
        let result = runtime.block_on(async move {
            future.await
        });

        sender.send(result).expect("The receiver closed")
    });

    receiver.recv().unwrap()
}