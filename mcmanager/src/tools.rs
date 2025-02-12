use termion::{clear, cursor};
use mcping;
use std::time::Duration;
use std::io::{stdin, stdout, Write};


pub fn sever_activity(){
    match mcping::get_status("10.0.0.226", Duration::from_secs(2)) {
        Ok(_) => println!("Connection Established"),
        Err(_) => println!("Connection Failed"),
    }

}

pub fn input_u16() -> u16 {

    loop{

        let mut input = String::new();

        std::io::stdin().read_line(&mut input).expect("fail");

        match input.trim().parse() {
            Ok(input) => return input,
            Err(_) => {
                write!(stdout(), "{}{}Please input a number\n", clear::All, cursor::Goto(1,1) ).unwrap();
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




