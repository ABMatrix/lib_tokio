use std::sync::mpsc;
use std::thread;

use tokio::sync::mpsc::{channel as tokio_channel, Sender as tokio_sender, Receiver as tokio_receiver};
use tokio::*;

use async_ffi::{FfiFuture, FutureExt};

#[link(name = "my_lib", kind = "static")]
extern "C" {
    #[no_mangle]
    fn send_from_lib(sender: mpsc::Sender<u8>, sender_tokio: tokio_sender<u8>) -> FfiFuture<()>;
}

async fn call(sender: mpsc::Sender<u8>, sender_tokio: tokio_sender<u8>){ //, sender_tokio: tokio_sender<u8>
    unsafe {
        send_from_lib(sender, sender_tokio).await;
    }
}

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let (sender,receiver) = mpsc::channel();
    let (tokio_sender, mut tokio_receiver) = tokio_channel(100);
    let handle = thread::spawn(move || async {
        sender.send(1u8).unwrap();
        call(sender,tokio_sender).await;
    });

    handle.join().unwrap().await;

    println!("start revice");
    let data = receiver.recv();
    println!("receive local: {:?}",data);
    let data = receiver.recv();
    println!("receive lib: {:?}",data);
    let data = tokio_receiver.recv().await;
    println!("receive lib tokio {:?}",data);
}
