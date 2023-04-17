use std::{net::SocketAddr};
use std::collections::HashMap;

use bytes::Bytes;
use tokio::net::{TcpListener, TcpStream};
use mini_redis::{Connection, Frame};
use std::sync::{Arc, Mutex};



async fn process(socket: TcpStream, _addr: SocketAddr, key_store: Arc<Mutex<HashMap<String, Bytes>>>) {
    use mini_redis::Command::{self, Get, Set};
    
    let mut connection = Connection::new(socket);

    

    while let Some(frame) = connection.read_frame().await.unwrap() {
        let respone = match Command::from_frame(frame).unwrap() {
            
            Set(cmd) => {
                println!("Set the cmd key : {:?}, cmd value : {:?}", cmd.key().to_string(), cmd.value().to_vec());
                let mut store = key_store.lock().unwrap();
                store.insert(cmd.key().to_string(), cmd.value().clone());
                drop(store);
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                println!("Check what is inside get : {:?}", cmd);
                let store = key_store.lock().unwrap();
                if let Some(value) = store.get(cmd.key()){
                    Frame::Bulk(value.clone().into())
                } else {
                    Frame::Null
                }
         
            }
            cmd => panic!("unimplemented {:?}", cmd),
        };

        connection.write_frame(&respone).await.unwrap();
    };

}

#[tokio::main]
async fn main() {

    let key_store: Arc<Mutex<HashMap<String, Bytes>>> = Arc::new(Mutex::new(HashMap::new()));

    let listener =  TcpListener::bind("127.0.0.1:6379").await.unwrap();
    loop {
        let (socket, addr) = listener.accept().await.unwrap();

        println!("Accepted the connection on : {}", addr);

        let db_store = key_store.clone();

        tokio::spawn(async move {
            process(socket, addr, db_store).await;
        });
    }
    
}