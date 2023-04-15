
use std::{net::SocketAddr};
use std::collections::HashMap;

use tokio::net::{TcpListener, TcpStream};
use mini_redis::{Connection, Frame};



async fn process(socket: TcpStream, addr: SocketAddr) {
    use mini_redis::Command::{self, Get, Set};
    
    let mut connection = Connection::new(socket);

    let mut key_store: HashMap<String, Vec<u8>> = HashMap::new();

    while let Some(frame) = connection.read_frame().await.unwrap() {
        let respone = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                println!("Set the cmd key : {:?}, cmd value : {:?}", cmd.key().to_string(), cmd.value().to_vec());
                key_store.insert(cmd.key().to_string(), cmd.value().to_vec());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                print!("Check what is inside get : {:?}", cmd);
         
                if let Some(value) = key_store.get(cmd.key()){
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

    let listener =  TcpListener::bind("127.0.0.1:6379").await.unwrap();
    loop {
        let (socket, addr) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            process(socket, addr,).await;
        });
    }
    
}