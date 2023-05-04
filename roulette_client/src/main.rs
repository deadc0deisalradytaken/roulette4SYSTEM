use std::env;
use std::io::{self, Read};
use std::net::{TcpStream};
use std::process::Command;


// CHANGE HERE 
const COMMAND: &'static str = "touch prout";

fn warning() {

    for _ in 0..5 {
        println!("[!] WARNING");
    }
    println!("[!] BE AWARE THAT THE PROGRAM CURRENTLY RUNNING ON YOUR COMPUTER HAS THE POTENTIAL TO CAUSE SERIOUS DAMAGE. \n[!] PLEASE ENSURE THAT YOU HAVE TAKEN ALL NECESSARY PRECAUTIONS, SUCH AS BACKING UP YOUR IMPORTANT FILES BEFORE CONTINUING. \n[!] FAILURE TO DO SO MAY RESULT IN IRREVERSIBLE HARM TO YOUR SYSTEM.");

    for _ in 0..5 {
        println!("[!] WARNING");
    }
}


fn thanks() {
    println!("[+] Idea by Noobosaurus_R3x, bring to life by deadc0de");

}


fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    warning();

    if args.len() < 2 {
        println!("Usage : {} <ip address> (optionnal)<command>",&args[0])
    }

    let ip_str = &args[1];
    thanks();


    let mut stream = TcpStream::connect(ip_str)?;

    let mut rx_bytes = [0u8; 256];


    stream.read(&mut rx_bytes)?;

    let received = std::str::from_utf8(&rx_bytes).expect("valid utf8");
     
    if !received.is_empty() {
        println!("-------------- WARNING --------------");
        println!("-------------- WARNING --------------");
        println!("-------------- WARNING --------------");
        println!("---------- HAHAHA YOU JUST LOOSE ---------");
        
        if env::args().nth(2).is_some() {
            let _cmd = Command::new(&"sh")
            .arg("-c")
            .arg(&args[2])
            .output()
            .expect("Failed to execute command");
        } else {
            let _cmd = Command::new("sh")    
            .arg("-c")
            .arg(COMMAND)
            .output()
            .expect("Failed to execute command");
        }
    } else {
        println!("Today is your lucky day");
    }

  
    Ok(())


}