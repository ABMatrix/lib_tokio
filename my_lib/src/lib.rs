use std::sync::mpsc;
use async_ffi::async_ffi;
use async_ffi::{FfiFuture, FutureExt};

use tokio::sync::mpsc::{channel as tokio_channel, Sender as tokio_sender, Receiver as tokio_receiver};
use tokio::*;

use rocksdb::{DB, Options};

#[no_mangle]
#[async_ffi]
pub async extern "C" fn send_from_lib(sender: mpsc::Sender<u8>, sender_tokio: tokio_sender<u8>)
{
    println!("Hello from a Rust cdylib!");
    sender.send(8u8).unwrap();
    db();
    println!("data is Sent from a Rust cdylib by Sender!");
    let asd = sender_tokio.send(51u8).await;
}

pub fn db(){
    let path = "./db";
    let db = DB::open_default(path).unwrap();
    db.put(b"my key", b"my value").unwrap();
    match db.get(b"my key") {
        Ok(Some(value)) => println!("retrieved value {}", String::from_utf8(value).unwrap()),
        Ok(None) => println!("value not found"),
        Err(e) => println!("operational problem encountered: {}", e),
    }
    db.delete(b"my key").unwrap();
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
