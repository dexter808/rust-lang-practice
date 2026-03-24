use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx2 = tx.clone();

    thread::spawn(move || {
        let messages = vec![
            String::from("Hi"),
            String::from("How"),
            String::from("are"),
            String::from("you!!"),
            ];
        
        for message in messages {
            thread::sleep(Duration::from_secs(1));
            tx.send(message).unwrap();
        }
    });

    thread::spawn(move || {
        let messages = vec![
            String::from("I"),
            String::from("am"),
            String::from("good"),
            String::from("wby!!"),
            ];
        
        for message in messages {
            thread::sleep(Duration::from_secs(1));
            tx2.send(message).unwrap();
        }
    });
    
    for message in rx {
        println!("Recieved: {:?}", message);
    }
}
