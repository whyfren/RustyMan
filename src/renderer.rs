#[cfg(not(target_arch = "wasm32"))]
use crossterm::style::{
    Stylize,
    StyledContent
};

#[derive(Clone, Copy)]
pub enum Color {
    Default,
    Green,
    Yellow,
    Orange,
    Red,
    Rust,
    Blue,
    Cyan,
    Magenta,
    White
}

pub struct StyledSpan {
    pub text: String,
    pub color: Color,
    pub bold:  bool,
}

pub struct Frame {
    pub spans:Vec<StyledSpan>
}

impl Frame {
    pub fn new() -> Frame {
        Frame{spans: Vec::new()} 
    }

    pub fn push(&mut self, text: &str, color: Color, bold: bool) {
        self.spans.push(StyledSpan {
            text: text.to_string(),
            color,
            bold
        });
    }

    pub fn pushln(&mut self, text: &str, color: Color, bold: bool) {
        self.spans.push(StyledSpan {
            text: format!("{}\n", text.to_string()),
            color,
            bold
        });
    }

    pub fn pushformat(&mut self, text: String, color: Color, bold: bool) {
        self.spans.push(StyledSpan {
            text: text,
            color,
            bold
        });
    }

    pub fn append(&mut self, other: Frame) {
        self.spans.extend(other.spans);
    }

    pub fn new_line(&mut self) {
        self.push("\n", Color::Default, false);
    }
}

#[cfg(not(target_arch = "wasm32"))] pub fn terminal_render(frame: &    Frame) {
    for span in &frame.spans {
        let c: String;
        let mut stylied: StyledContent<String>;
        c = span.text.to_string();
        

        match span.color {
            Color::Green => {stylied = c.green();}
            Color::Yellow => {stylied = c.yellow();}
            Color::Orange => {stylied = c.dark_yellow();}
            Color::Red => {stylied = c.red();}
            Color::Rust => {stylied = c.dark_yellow();}
            Color::Blue => {stylied = c.blue();}
            Color::Cyan=> {stylied = c.cyan();}
            Color::Magenta => {stylied = c.magenta();}
            Color::White => {stylied = c.white();}
            Color::Default => {stylied = c.reset();}
        }
        if span.bold {stylied = stylied.bold()}
        print!("{}", stylied);
    }
}

#[cfg(target_arch = "wasm32")] pub fn web_render(frame: &Frame) -> String {
    let mut out = String::new();
    for span in &frame.spans {
        let class_name = match span.color {
            Color::Green => "Green",
            Color::Yellow => "Yellow",
            Color::Orange => "Orange",
            Color::Red => "Red",
            Color::Rust => "Rust",
            Color::Blue => "Blue",
            Color::Cyan => "Cyan",
            Color::Magenta => "Magenta",
            Color::White => "White",
            Color::Default => "Default",
        };
        if span.bold {
            out.push_str(&format!("<span class=\"{} bold\">{}</span>", class_name, span.text));
        } else {
            out.push_str(&format!("<span class=\"{}\">{}</span>", class_name, span.text));
        }
    }
    out
}