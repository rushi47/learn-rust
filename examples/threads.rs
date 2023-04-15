use std::thread;
use std::time::Duration;


struct User {
    Name: String,
}

fn main() {
    let user = User{Name: String::from("Rushi")};
    let t1 = thread::spawn( move || {
        for i in 1..10 {
            println!("Checking the name : {} : {}", user.Name, i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..10 {
        println!("Main thread print : {}", i);
        thread::sleep(Duration::from_millis(1));
    }
    t1.join().unwrap();
    // println!("Check if its here : {}", user.Name);
}