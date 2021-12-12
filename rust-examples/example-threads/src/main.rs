use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main(){
  let (tx ,rx) = mpsc::channel();

  let tx1 = tx.clone();

  let delay = 100;

  thread::spawn(move || {
    let string_vec = vec! {
      String::from("One"),
      String::from("Two"),
      String::from("Three"),
      String::from("Four"),
    };

    for value in string_vec {
      tx.send(value).unwrap();
      thread::sleep(Duration::from_millis(delay));
    }
  });

  thread::spawn(move || {
    let string_vec = vec! {
      String::from("A"),
      String::from("B"),
      String::from("C"),
      String::from("D"),
    };

    for value in string_vec {
      tx1.send(value).unwrap();
      thread::sleep(Duration::from_millis(delay));
    }
  });


  for received in rx {
    println!("Received: {}", received);
  }
}