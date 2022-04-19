extern crate rand;
extern crate websocket;
use std::io;
use std::cmp::Ordering;
use std::thread;
use websocket::OwnedMessage;
use websocket::sync::Server;
use rand::Rng;
use rand::prelude::{random, thread_rng,  IteratorRandom, SliceRandom};

fn main() {
    domore();
    wsserver();
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);
    let mut cnt = 0;

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        cnt=cnt+1;

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win! in {} times",cnt);
                break;
            }
        }
    }   
}

fn domore(){
    // We can use random() immediately. It can produce values of many common types:
    let x: u8 = random();
    println!("{}", x);

    if random() { // generates a boolean
        println!("Heads!");
    }

    // If we want to be a bit more explicit (and a little more efficient) we can
    // make a handle to the thread-local generator:
    let mut rng = thread_rng();
    if rng.gen() { // random bool
        let x: f64 = rng.gen(); // random number in range [0, 1)
        let y = rng.gen_range(-10.0..10.0);
        println!("x is: {}", x);
        println!("y is: {}", y);
    }

    println!("Die roll: {}", rng.gen_range(1..=6));
    println!("Number from 0 to 9: {}", rng.gen_range(0..=10));
    
    // Sometimes it's useful to use distributions directly:
    let distr = rand::distributions::Uniform::new_inclusive(1, 100);
    let mut nums = [0i32; 3];
    for x in &mut nums {
        *x = rng.sample(distr);
    }
    println!("Some numbers: {:?}", nums);
    // We can also interact with iterators and slices:
    let arrows_iter = "➡⬈⬆⬉⬅⬋⬇⬊".chars();
    println!("Lets go in this direction: {}", arrows_iter.choose(&mut rng).unwrap());
    let mut nums = [1, 2, 3, 4, 5];
    nums.shuffle(&mut rng);
    println!("I shuffled my {:?}", nums);
}

fn wsserver(){
    let server = Server::bind("127.0.0.1:2794").unwrap();
    for request in server.filter_map(Result::ok) {
        //spawn a new thread for each connection
        thread::spawn(move||{
            if !request.protocols().contains(&"rust-websocket".to_string()){
                request.reject().unwrap();
                return;
            }
            let mut client = request.use_protocol("rust-websocket").accept().unwrap();
            let ip = client.peer_addr().unwrap();
            println!("Connect from {}",ip);
            let message = OwnedMessage::Text("Hello client, this is a message from server".to_string());
            client.send_message(&message).unwrap();
            let (mut receiver,mut sender) = client.split().unwrap();
            for message in receiver.incoming_messages() {
                let message = message.unwrap();
                match message {
                    OwnedMessage::Close(_)=>{
                        let message = OwnedMessage::Close(None);
                        sender.send_message(&message).unwrap();
                    }
                    OwnedMessage::Ping(ping)=>{
                        let message = OwnedMessage::Pong(ping);
                        sender.send_message(&message).unwrap();
                    }

                    _=>sender.send_message(&message).unwrap(),
                }
            }
        });
    }
}

