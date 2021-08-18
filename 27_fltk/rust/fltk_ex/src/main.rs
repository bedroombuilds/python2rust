use fltk::{
    app,
    button::Button,
    enums::{Event, Key},
    frame::Frame,
    group::{Pack, PackType},
    prelude::*,
    window::Window,
};
use num_enum::TryFromPrimitive;
use std::cell::RefCell;
use std::convert::TryFrom;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq, TryFromPrimitive)]
#[repr(i32)]
enum CustomEvents {
    AddOne = 41, // Values below 30 are reserved
    AddTwo,
    SubThree,
    OpenFile,
}

struct Adder {}

impl Adder {
    pub fn new() -> Self {
        let mut container = Pack::new(80, 50, 200, 20, "Adder Widget");
        let mut button = Button::new(0, 0, 50, 20, "add 1");
        button.set_callback(move |_widg| {
            let _ = app::handle_main(CustomEvents::AddOne as i32);
        });
        let mut button2 = Button::new(0, 0, 50, 20, "add 2");
        button2.set_callback(move |_widg| {
            let _ = app::handle_main(CustomEvents::AddTwo as i32);
        });
        let mut button3 = Button::new(0, 0, 80, 20, "subtract 3");
        button3.set_callback(move |_widg| {
            let _ = app::handle_main(CustomEvents::SubThree as i32);
        });
        container.end();
        container.set_type(PackType::Horizontal);
        let mut container2 = Pack::new(80, 130, 200, 20, "File Widget");
        let mut button4 = Button::new(0, 0, 80, 20, "open file");
        button4.set_callback(move |_widg| {
            let _ = app::handle_main(CustomEvents::OpenFile as i32);
        });
        container2.end();
        container2.set_type(PackType::Horizontal);
        Adder {}
    }
}

struct MyWindow {}

impl MyWindow {
    pub fn new() -> Self {
        let counter = Rc::from(RefCell::from(0));

        let mut win = Window::new(200, 200, 300, 200, "Ordinal Events");
        let mut disp_frame = Frame::new(200, 0, 200, 200, "0").center_of_parent();
        let _adder = Adder::new();

        let counter_cl = counter.clone();

        disp_frame.handle(move |widg, ev| match CustomEvents::try_from(ev.bits()) {
            Ok(ce) => {
                dbg!("also handled event here");
                match ce {
                    CustomEvents::AddOne | CustomEvents::AddTwo | CustomEvents::SubThree => {
                        widg.set_label(&*counter_cl.borrow_mut().to_string());
                        true
                    }
                    CustomEvents::OpenFile => true,
                }
            }
            Err(_) => false,
        });

        win.end();
        win.show();

        win.handle(move |_widg, ev| {
            if ev == Event::Shortcut && app::event_key() == Key::Escape {
                dbg!("ignoring ESC key window-close");
                return true;
            }
            match CustomEvents::try_from(ev.bits()) {
                Ok(ce) => {
                    match ce {
                        CustomEvents::AddOne => *counter.borrow_mut() += 1,
                        CustomEvents::AddTwo => *counter.borrow_mut() += 2,
                        CustomEvents::SubThree => *counter.borrow_mut() -= 3,
                        CustomEvents::OpenFile => {
                            let file = fltk::dialog::file_chooser("Choose File", "*.rs", ".", true)
                                .unwrap();
                            dbg!(file);
                        }
                    }
                    true
                }
                Err(_) => false,
            }
        });

        MyWindow {}
    }
}

fn main() {
    let fltk_app = app::App::default();
    let _ = MyWindow::new();
    dbg!(CustomEvents::AddOne as i32);
    dbg!(CustomEvents::AddTwo as i32);
    dbg!(CustomEvents::SubThree as i32);
    fltk_app.run().unwrap();
}
