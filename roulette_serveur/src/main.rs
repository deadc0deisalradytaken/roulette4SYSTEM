use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::process;
use rand::{ Rng, thread_rng};
use std::env;

fn random_int(n: usize) -> Option<usize>  {
    Some(thread_rng().gen_range(0..n))
}

fn send_command(mut addr: &std::net::TcpStream, command: &str) -> std::io::Result<()> {
    addr.write_all(command.as_bytes())?;
    Ok(())
}

fn main() -> std::io::Result<()> {

    let args: Vec<String> = env::args().collect();

    let listener = TcpListener::bind("127.0.0.1:6666")?;
    let mut connections: Vec<TcpStream> = Vec::new();

    let mut count = 0;

    listener.incoming().for_each(|stream| {
        match stream {
            Ok(stream) => {
                count +=1 ;
                connections.push(stream.try_clone().unwrap());

                let peer_addr = stream.peer_addr().unwrap();
                println!("[+] Connection from {}, waiting for {} player",peer_addr, (10-count));


                if count == 10 { //change count
                    match connections.get(random_int(count).unwrap()) {
                        Some(choosen_one) => {
                            println!("[+] Bad day for {}, say good bye to system ! ",choosen_one.peer_addr().unwrap());
                            match send_command(&mut &choosen_one, "CHOOSEN" ) {
                                Ok(_)  => println!("Commmand was sent :) "),
                                Err(e) => eprintln!("Error sending text : {}",e)
                            }
                            process::exit(0);
                        }
                        _ => (),
                    }

                }
            }
            Err(_) => {
                eprintln!("Didn't work");
                process::exit(1)
            }
        }
    });


    Ok(())

}
