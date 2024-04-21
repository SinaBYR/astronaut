use std::{char, collections::HashMap};

struct Node {
    // data common to all nodes:
    children: Vec<Node>,

    // data specific to each node type:
    node_type: NodeType,
}

enum NodeType {
    Text(String),
    Element(ElementData),
}

struct ElementData {
    tag_name: String,
    attributes: AttrMap,
}

type AttrMap = HashMap<String, String>;

fn text(data: String) -> Node {
    Node {
        children: Vec::new(),
        node_type: NodeType::Text(data),
    }
}

fn elem(tag_name: String, attributes: AttrMap, children: Vec<Node>) -> Node {
    Node {
        children,
        node_type: NodeType::Element(ElementData {
            tag_name,
            attributes
        }),
    }
}

struct Parser {
    pos: usize, // index of the next character we haven’t processed yet
    input: String,
}

impl Parser {
    fn next_char(&self) -> char {
        self.input[self.pos..].chars().next().unwrap()
    }

    fn starts_with(&self, s: &str) -> bool {
        self.input[self.pos..].starts_with(s)
    }

    fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }

    fn consume_char(&mut self) -> char {
        let mut iter = self.input[self.pos..].char_indices();
        let (_, curr_char) = iter.next().unwrap();
        let (next_pos, _) = iter.next().unwrap_or((1, ' '));
        self.pos += next_pos;
        curr_char
    }

    fn consume_while<F>(&mut self, test: F) -> String where F: Fn(char) -> bool {
        let mut result = String::new();
        while !self.eof() && test(self.next_char()) {
            result.push(self.consume_char());
        }
        result
    }

    fn consume_whitespace(&mut self) -> String {
        self.consume_while(|c| char::is_whitespace(c))
    }

    fn parse_tag_name(&mut self) -> String {
        self.consume_while(|c| match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' => true,
            _ => false,
        })
    }

    fn parse_node(&mut self) -> Node {
        match self.next_char() {
            '<' => self.parse_element(),
            _   => self.parse_text(),
        }
    }

    fn parse_text(&mut self) -> Node {
        text(self.consume_while(|c| c != '<'))
    }

    fn parse_element(&mut self) -> Node {
        let tag_name = self.parse_tag_name();
        let attributes = self.parse_attributes();
        let children = self.parse_nodes();
        elem(tag_name, attributes, children)
    }

    fn parse_attr_value(&mut self) -> (String, String) {
        let open_quote = self.consume_char();
        let name = self.parse_tag_name();
        let value = self.consume_while(|c| c != open_quote);
        (name, value)
    }

    fn parse_attributes(&mut self) -> AttrMap {
        let mut attributes: AttrMap = HashMap::new();
        loop {
            self.consume_whitespace();
            if self.next_char() == '>' {
                break;
            }
            let (name, value) = self.parse_attr_value();
            attributes.insert(name, value);
        }
        attributes
    }

    fn parse_nodes(&mut self) -> Vec<Node> {
        let mut nodes: Vec<Node> = Vec::new();

        loop {
            self.consume_whitespace();
            if self.eof() || self.starts_with("</") {
                break;
            }
            nodes.push(self.parse_node());
        }

        nodes
    }
}

pub fn parse(source: String) -> Node {
    let mut nodes = Parser { pos: 0, input: source }.parse_nodes();

    if nodes.len() == 1 {
        nodes.swap_remove(0)
    } else {
        elem("html".to_string(), HashMap::new(), nodes)
    }
}

fn main() {
    // <div>
    //     <p>hello</p>
    //     world
    // </div>

    let div_attrs: HashMap<String, String> = HashMap::new();
    let p_attrs: HashMap<String, String> = HashMap::new();
    let mut p_children: Vec<Node> = Vec::new();
    p_children.push(Node {children: Vec::new(), node_type: NodeType::Text(String::from("hello"))});
    let p = elem(String::from("p"), p_attrs, p_children);
    let mut div_children: Vec<Node> = Vec::new();
    div_children.push(p);

    let world_text = text(String::from("world"));
    div_children.push(world_text);

    let div = elem(String::from("div"), div_attrs, div_children);
}

