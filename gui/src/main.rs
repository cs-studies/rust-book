use gui::{Draw, Button, Screen};

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("I drew a {} x {} Select Box with {:?} options",
                 self.width,
                 self.height,
                 self.options,
        );
    }
}

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("Submit"),
            }),
        ],
    };

    screen.run();

    // let failedScreen = Screen {
    //     components: vec![Box::new(String::from("I cannot draw."))],
    // };

    // failedScreen.run();
}
