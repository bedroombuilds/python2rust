#[derive(Debug)]
enum Button {
    Left,
    Right,
}

struct Click {
    position: (u16, u16),
    button: Button,
}

struct TextColor {
    text: String,
    color: String,
}

enum Event {
    Button(Button),
    Click(Click),
    TextColor(TextColor),
    SleepText { sleep: String },
    Sleep { sleep: f64 },
}

fn main() {
    let events = vec![
        Event::Button(Button::Left),
        Event::Click(Click {
            position: (2, 4),
            button: Button::Left,
        }),
        Event::Click(Click {
            position: (32, 4),
            button: Button::Right,
        }),
        Event::TextColor(TextColor {
            text: "blah".to_string(),
            color: "blue".to_string(),
        }),
        Event::SleepText {
            sleep: "long".to_string(),
        },
        Event::Sleep { sleep: 1.3 },
        Event::Click(Click {
            position: (0, 4),
            button: Button::Right,
        }),
        Event::Click(Click {
            position: (0, 4),
            button: Button::Left,
        }),
    ];

    for event in events {
        match event {
            Event::Button(b) => println!("button found {:?}", b),
            Event::Click(Click {
                position: (x, y),
                button: Button::Right,
            }) => println!("right click {} {}", x, y),
            Event::Click(Click {
                position: (0, y),
                button: _,
            }) => println!("clickend on x-axis y: {}", y),
            Event::Click(Click {
                position: (x, y),
                button: _,
            }) => println!("Click {} {}", x, y),
            Event::TextColor(TextColor {
                text: message,
                color,
            }) => println!("showing {} in color {}", message, color),
            Event::Sleep { sleep: t } => println!("sleeping {}secs", t),
            _ => continue,
        }
    }
}
