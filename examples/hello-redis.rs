use mini_redis::{client, Result};

async fn say_world() -> String {
    println!("Say world");
    return  "Checking".to_string();
}

#[tokio::main]
async fn main() -> Result<()>{
    let mut client = client::connect("127.0.0.1:6379").await?;

    client.set("tokio", "hello".into()).await?;

    let op = say_world();

    println!("Got the value {:?}", client.get("tokio").await?);

    op.await;

    Ok(())
 
}