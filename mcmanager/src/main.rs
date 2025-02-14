use crossterm::{event::{self, KeyCode, KeyEvent}, ExecutableCommand, terminal::{Clear, ClearType}, cursor::{MoveTo, Hide}, style::{Color, SetForegroundColor}};
use std::io::{stdout, stdin, Write};
mod tools;

fn main(){
    
    let mut current_location = (1, 1);
    let mut stdout = stdout();
    crossterm::terminal::enable_raw_mode().unwrap();
    stdout.execute(crossterm::terminal::Clear(crossterm::terminal::ClearType::All)).unwrap();
    stdout.flush().unwrap();
    main_menu(&mut current_location); 

    loop {
        if event::poll(std::time::Duration::from_millis(100)).unwrap() {
            if let event::Event::Key(KeyEvent {code, modifiers, kind, state, ..
            }) = event::read().unwrap(){
            match code {
                KeyCode::Up | KeyCode::Char('w') => hover_location(&mut current_location, (0, -1)),
                KeyCode::Down | KeyCode::Char('s') => hover_location(&mut current_location, (0, 1)),
                KeyCode::Left | KeyCode::Char('a') => hover_location(&mut current_location, (-1, 0)),
                KeyCode::Right | KeyCode::Char('d') => hover_location(&mut current_location, (1, 0)),
                KeyCode::Enter => menu_select(&mut current_location),
                _ => continue,
            }
        }
        }

}

}


fn hover_location(current_location: &mut (i16, i16) , update_location: (i16, i16))  {


    match  (current_location.0 + update_location.0, current_location.1 + update_location.1) {
        (1..,1..) => {*current_location = (current_location.0 + update_location.0, current_location.1 + update_location.1);
                    main_menu(current_location);
        },
        _ => return,

    };
     
}

fn menu_select(current_location: &mut (i16, i16)){

    match *current_location {
        (1,1) => tools::command(String::from("systemctl start minecraft.service")),
        (1,2) => tools::command(String::from("systemctl stop minecraft.service")),
        (1,3) => tools::command(String::from("systemctl restart minecraft.service")),
        (1,4) => {
            crossterm::terminal::disable_raw_mode().unwrap();
            std::process::exit(0);
            }
        _ => {
            *current_location = (1,1);
            main_menu(current_location);
            }
        }

} 

fn main_menu(current_location: &mut (i16, i16)) {
    let mut list: Vec<(String, bool, (i16, i16))> = vec![(String::from("Start"), false, (1,1)), (String::from("Stop"), false, (1,2)), (String::from("Restart"), false, (1,3)), (String::from("Exit"), false, (1,4))];
        tools::clear_screen();
        let mut stdout = stdout();
        tools::sever_activity();
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
                stdout.execute(SetForegroundColor(Color::Blue)).unwrap();
                write!(stdout, "{}\r\n",list[i].0).unwrap();
                stdout.execute(SetForegroundColor(Color::Reset)).unwrap();
                stdout.flush().unwrap();
            } else {
                write!(stdout, "{}\r\n",list[i].0).unwrap();
                stdout.flush().unwrap();
            }
        }
            
        
}