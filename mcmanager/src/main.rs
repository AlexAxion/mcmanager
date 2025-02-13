use termion::event::Key;
use termion::raw::IntoRawMode;
use termion::input::TermRead;
use termion::style::CrossedOut;
use termion::{clear, cursor, color};
use std::io::{stdin, stdout, Write};
mod tools;

fn main(){
    
    let mut current_location = (1, 1);
    main_menu(&mut current_location);
    let stdin = stdin();
    let mut _stdout = stdout().into_raw_mode().unwrap();

        for key in stdin.keys() {
            match key.unwrap() {
                Key::Up | Key::Char('w') => hover_location(&mut current_location, (0,1)),
                Key::Down | Key::Char('s') => hover_location(&mut current_location, (0,-1)),
                Key::Left | Key::Char('a') => hover_location(&mut current_location, (-1,0)),
                Key::Right | Key::Char('d') => hover_location(&mut current_location, (1,0)),
                Key::Char('q') => break,
                _ => continue,
            }
        }

}


fn hover_location(current_location: &mut (i16, i16) , update_location: (i16, i16))  {


    match  (current_location.0 + update_location.0, current_location.1 + update_location.1) {
        (1..2,1..) => {*current_location = (current_location.0 + update_location.0, current_location.1 + update_location.1);
                    main_menu(current_location);
        },
        _ => return,

    };
     
    //write!(stdout(), "{}{}\r{:?}", clear::All, cursor::Goto(1,1), current_location).unwrap();
    //stdout().flush().unwrap();
     
}


pub fn main_menu(current_location: &mut (i16, i16)) {
    let mut list: Vec<(String, bool, (i16, i16))> = vec![(String::from("Start"), false, (1,1)),    (String::from("Stop"), false, (1,2)),  (String::from("Restart"), false, (1,3)),];
        write!(stdout(), "{}{}\r", clear::All, cursor::Goto(1,1)).unwrap();
        stdout().flush().unwrap();

        for i in 0..list.len() {

            match list[i].2 == *current_location {

                true => {
                    list[i].1 = true;
                    break;
                    },

                _=> continue,
            
                }
        }

        for i in 0..list.len() {
            if list[i].1 == true {
                write!(stdout(), "{}{}{}\r\n", color::Fg(color::Red), list[i].0, color::Fg(color::Reset)).unwrap();
                stdout().flush().unwrap();
            } else {
                write!(stdout(), "{}\r\n", list[i].0).unwrap();
                stdout().flush().unwrap();
            }
        }
            
        
}