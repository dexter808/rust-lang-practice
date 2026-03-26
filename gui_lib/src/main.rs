use gui_lib::{Draw, Screen, Button};

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // draw Select Box
    }
}

fn main() {
    let screen = Screen {
        components: vec![
            // Box::new(String::from(String::new("Something"))),
            Box::new(
                SelectBox {
                    width: 100,
                    height: 100,
                    options: vec![String::from("A"), String::from("B"), String::from("C")]
                }
            ),
            Box::new(
                Button {
                    width: 100,
                    height: 100,
                    label: String::from("My Button"),
                }
            ),
        ],
    };
}
