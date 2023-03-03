#![allow(unused_variables, dead_code)]

pub trait Widget {
    /// Natural width of `self`.
    fn width(&self) -> usize;

    /// Draw the widget into a buffer.
    fn draw_into(&self, buffer: &mut dyn std::fmt::Write);

    /// Draw the widget on standard output.
    fn draw(&self) {
        let mut buffer = String::new();
        self.draw_into(&mut buffer);
        println!("{buffer}");
    }
}

pub struct Label {
    label: String,
}

impl Label {
    fn new(label: &str) -> Label {
        Label {
            label: label.to_owned(),
        }
    }
}

pub struct Button {
    label: Label,
    callback: Box<dyn FnMut()>,
}

impl Button {
    fn new(label: &str, callback: Box<dyn FnMut()>) -> Button {
        Button {
            label: Label::new(label),
            callback,
        }
    }
}

pub struct Window {
    title: String,
    widgets: Vec<Box<dyn Widget>>,
}

impl Window {
    fn new(title: &str) -> Window {
        Window {
            title: title.to_owned(),
            widgets: Vec::new(),
        }
    }

    fn add_widget(&mut self, widget: Box<dyn Widget>) {
        self.widgets.push(widget);
    }
}


impl Widget for Label {
    fn width(&self) -> usize {
        self.label.len()
    }

    fn draw_into(&self, buffer: &mut dyn std::fmt::Write) {
        buffer.write_str(&self.label).unwrap();
    }
}

impl Widget for Button {
    fn width(&self) -> usize {
        self.label.width() + 2
    }

    fn draw_into(&self, buffer: &mut dyn std::fmt::Write) {
        buffer.write_char('|').unwrap();
        self.label.draw_into(buffer);
        buffer.write_char('|').unwrap();
        buffer.write_char('\n').unwrap();
    }
}

impl Widget for Window {
    fn width(&self) -> usize {
        let mut max_width = 0;
        for widget in &self.widgets {
            max_width = std::cmp::max( widget.width(), max_width);
        }
        return max_width+2;
    }

    fn draw_into(&self, buffer: &mut dyn std::fmt::Write) {
        let width = self.width();
        buffer.write_str(&"=".repeat(width)).unwrap();
        buffer.write_char('\n').unwrap();
        buffer.write_fmt(format_args!("|{: ^width$}|\n", self.title)).unwrap();
        for widget in &self.widgets {
            widget.draw_into(buffer);
        }
    }
}

fn main() {
    let mut window = Window::new("Rust GUI Demo 1.23");
    window.add_widget(Box::new(Label::new("This is a small text GUI demo.")));
    window.add_widget(Box::new(Button::new(
        "Click me!",
        Box::new(|| println!("You clicked the button!")),
    )));
    window.draw();
}
