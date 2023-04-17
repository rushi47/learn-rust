use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};


struct User {
    Name: String,
}

fn _test() {
    let user = User{Name: "check".to_string()};

    thread::spawn( move || {
        println!("Here I am : {}", user.Name);
    });
}

fn main() {
    let user = Arc::new(Mutex::new(User{Name: String::from("Rushi")}));

    let tmp_user = user.clone();

    let t1 = thread::spawn( move || {
        let mut u = tmp_user.lock().unwrap();
        u.Name = "vi".to_string();
        drop(u);
    });

    let t2_user = user.clone();

    let t2 = thread::spawn( move || {
        thread::sleep(Duration::from_secs(1));
        println!("Value for the user : {}", t2_user.lock().unwrap().Name);
    });

    println!("Hmm ok, : {}", user.lock().unwrap().Name);

    t1.join().unwrap();
    t2.join().unwrap();
    // println!("Check if its here : {}", user.Name);
}