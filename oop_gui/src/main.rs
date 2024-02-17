use oop_gui::{Button, Screen, TextInput};

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(Button {
                width: 200,
                height: 50,
                label: String::from("My Button"),
            }),
            Box::new(TextInput {
                width: 400,
                height: 50,
                placeholder: String::from("Enter Text"),
                value: String::from("value"),
            }),
        ],
    };

    screen.run();
}
