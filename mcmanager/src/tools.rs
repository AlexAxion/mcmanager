
use crossterm::{ExecutableCommand, terminal::{Clear, ClearType}, cursor::{MoveTo, Hide}, style::{Color, SetForegroundColor}};
use mcping;
use std::time::Duration;
use std::io::{stdin, stdout, Write};
use std::io::prelude::*;
use std::net::TcpStream;
use ssh2::Session;

pub fn sever_activity(){
    let mut stdout = stdout();
    match mcping::get_status("73.235.141.68", Duration::from_secs(1)) {
        Ok(_) => {
            stdout.execute(SetForegroundColor(Color::Green)).unwrap();
            write!(stdout, "Connection Established\r\n").unwrap();
            stdout.execute(SetForegroundColor(Color::Reset)).unwrap();
            stdout.flush().unwrap();
        }
        Err(_) => {
            stdout.execute(SetForegroundColor(Color::Red)).unwrap();
            write!(stdout, "Connection failed\r\n").unwrap();
            stdout.execute(SetForegroundColor(Color::Reset)).unwrap();
            stdout.flush().unwrap();
            },
    }

}


pub fn clear_screen(){
    let mut stdout = stdout();
    stdout.execute(Clear(ClearType::All)).unwrap();
    stdout.execute(MoveTo(0, 0)).unwrap();
    stdout.execute(Hide).unwrap();
    stdout.flush().unwrap();

    
}

pub fn command(server: String){
    let tcp = TcpStream::connect("10.0.0.226:22").unwrap();
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();
    
    sess.userauth_password("root", "Enderwalker38").unwrap();
    assert!(sess.authenticated());
    
    let mut channel = sess.channel_session().unwrap();
    channel.exec(&server).unwrap();
    let mut s = String::new();
    channel.read_to_string(&mut s).unwrap();
    println!("{}", s);
    channel.wait_close().unwrap();
    println!("{}", channel.exit_status().unwrap());

}


pub fn input_u16() -> u16 {

    loop{

        let mut input = String::new();

        std::io::stdin().read_line(&mut input).expect("fail");

        match input.trim().parse() {
            Ok(input) => return input,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                stdout().flush().unwrap();
                continue;
            }
        };
    }

}

pub fn input_str()-> String{
    println!("please input a string");

    let mut input = String::new();

    std::io::stdin().read_line(&mut input).expect("fail");

    input

}




