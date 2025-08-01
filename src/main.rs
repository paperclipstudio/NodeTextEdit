#![allow(dead_code)]
#![allow(unused_variables)]

enum Edit {
    Add(),
    Sub(),
}

enum Node {
    Edit(),
    Text(String, usize, usize),
}

struct Section {
    node: Node,
    start: usize,
    length: usize,
}

struct Start {
    length: usize,
    length_out_of_date: bool,
    sections: Vec<Section>,
}

impl Start {
    fn length(&self) -> usize {
        0
    }

    fn new() -> Self {
        Start {
            length: 0,
            length_out_of_date: false,
            sections: Vec::new(),
        }
    }

    fn render(&self) {
        for section in &self.sections {
            match &section.node {
                Node::Edit() => todo!(),
                Node::Text(text, start, len) => {
                    print!("{}", &text[*start..*start + *len]);
                    //println!("{}", text.chars().skip(*start).take(*len).collect::<&str>());
                }
            }
        }
    }

    fn get_section_at(&self, pos: usize) -> Option<&Section> {
        self.sections
            .iter()
            .skip_while(|s| s.start + s.length >= pos)
            .take_while(|s| s.start <= pos)
            .last()
    }

    fn add(mut self, start: usize, text: String) -> Self {
        let len = text.len();
        let section = Section {
            length: len,
            start,
            node: Node::Text(text, 0, len),
        };
        self.sections.push(section);
        self
    }
}

fn main() {
    println!("Hello, world!");
    let start = Start::new()
        .add(0, String::from("Hello wold"))
        .add(8, String::from("r"))
        .render();
}
