
use bytes::Bytes;
use mini_redis::client;
use tokio::{sync::mpsc, sync::oneshot};

#[derive(Debug)]
enum Command {
    Get {
        key: String,
        res: Responder<Option<Bytes>>,
    },
    Set {
        key: String,
        val: Bytes,
        res: Responder<()>,
    }
}

type Responder<T> = oneshot::Sender<mini_redis::Result<T>>;

async fn manager(mut rx: mpsc::Receiver<Command>) {
    
    let mut client = client::connect("127.0.0.1:6379").await.unwrap();

    while let Some(message) = rx.recv().await {
        match message {
            Command::Get { key, res } => {
                
                let val = client.get(&key).await;
                let _ = res.send(val);
                
            },
            Command::Set { key, val , res} => {
                
                let op = client.set(&key, val).await;
                let _ = res.send(op);
                
                
            },
        }
    }
}

async fn send_msg(cmd: Command, tx: mpsc::Sender<Command>) -> Result<String, String> {
    match tx.send(cmd).await {
            Ok(_) => Ok("Worked".into()),
            Err(_) => Err("Issue while sending".into()),
        }
}

#[tokio::main]
async fn main () {
    //create new channel with capacity 
    let (tx, rx) = mpsc::channel(32);

    
    let manage = tokio::spawn(async move {
        manager(rx).await;
    });

    let sender = tx.clone();
    
    let t1 = tokio::spawn(async move {
        //send and receive the client received from redis
        let( se, receiver) = oneshot::channel();

        let msg = Command::Set { key: "setting".to_string(), val: "cache".into(), res: se };
        // sender.send(msg).await.unwrap();
        send_msg(msg, sender).await.unwrap();
        match receiver.await.unwrap() {
            Ok(resp) => println!("Received = {:?}", resp),
            Err(resp) => println!("Some issue : {:?}", resp),
        }
        
    });

    let sender2 = tx.clone();
    
    let t2 = tokio::spawn(async move {
       
        let( se, receiver) = oneshot::channel();
       
        let msg = Command::Get { key: "setting".to_string(), res: se };
       
        send_msg(msg, sender2).await.unwrap();
       
        match receiver.await.unwrap() {
            Ok(msg) => println!("Received [X] : {:?}", msg),
            Err(msg) => println!("Some issue : {:?}", msg),
        }
    });

    
    t1.await.unwrap();
    t2.await.unwrap();
    manage.await.unwrap();

}