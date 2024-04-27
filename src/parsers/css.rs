// TODO add specifity support
// TODO add is_numeric util

pub struct StyleSheet {
    styles: Vec<Rule>,
}

pub struct Rule {
    selector: Selector,
    declarations: Vec<Property>,
}

pub struct Selector {
    tag_name: Option<String>,
    id: Option<String>,
    class: Vec<String>,
}

pub struct Property {
    name: String,
    value: Value,
}

pub enum Value {
    Keyword(String),
    Numeric(f32, Unit),
    ColorValue(Color),
}

pub enum Unit {
    Px,
}

pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

pub struct Parser {
    pub input: String,
    pub pos: usize,
}

impl Parser {
    // ---------------------------------------------
    // ---------------------------------------------
    //                  UTILS START
    // ---------------------------------------------
    // ---------------------------------------------

    fn next_char(&mut self) -> char {
        self.input[self.pos..].chars().next().unwrap()
    }

    fn consume_char(&mut self) -> char {
        let mut iter = self.input[self.pos..].char_indices();
        let (_, curr_char) = iter.next().unwrap();
        let (next_pos, _) = iter.next().unwrap_or((1, ' '));
        self.pos += next_pos;
        curr_char
    }

    fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }

    fn consume_while<F>(&mut self, test: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let mut result = String::new();
        while !self.eof() && test(self.next_char()) {
            result.push(self.next_char())
        }
        result
    }

    fn consume_whitespace(&mut self) -> String {
        self.consume_while(|c| char::is_whitespace(c))
    }

    fn starts_with(&mut self, s: &str) -> bool {
        self.input[self.pos..].starts_with(s)
    }

    // ---------------------------------------------
    // ---------------------------------------------
    //                  Selector Parsing
    // ---------------------------------------------
    // ---------------------------------------------
    fn parse_id(&mut self) -> String {
        self.consume_char(); // parses hashtag #
        let id = self.consume_while(|c| !char::is_whitespace(c) || c != '.' || 'c' != '{');
        id
    }

    fn parse_class(&mut self) -> String {
        self.consume_char(); // parses dot .
        let class = self.consume_while(|c| !char::is_whitespace(c) || c != '.' || c != '#');
        class
    }

    fn parse_tag(&mut self) -> String {
        let tag_name = self.consume_while(|c| char::is_whitespace(c) || c == '{' || c == '#' || c == '.');
        tag_name
    }

    fn parse_selector(&mut self) -> Selector {
        let mut selector: Selector = Selector {tag_name: None, id: None, class: Vec::new()};
        while !char::is_whitespace(self.next_char()) || self.next_char() != '{' {
            match self.next_char() {
                '#' => {
                    selector.id = Some(self.parse_id());
                }
                '.' => {
                    selector.class.push(self.parse_class());
                }
                _   => {
                    selector.tag_name = Some(self.parse_tag());
                },
            }
        }
        selector
    }

    // ---------------------------------------------
    // ---------------------------------------------
    //                  Declarations Parsing
    // ---------------------------------------------
    // ---------------------------------------------
    fn parse_rgba(&mut self) -> Color {
        self.consume_while(|c| !c.is_numeric());                                   // parse rgba(

        let r = self.consume_while(|c| c.is_numeric()).parse::<u8>().unwrap();     // parse red
        self.consume_while(|c| !c.is_numeric());                                   // parse whitespace and comma

        let g = self.consume_while(|c| c.is_numeric()).parse::<u8>().unwrap();     // parse green
        self.consume_while(|c| !c.is_numeric());                                   // parse whitespace and comma

        let b = self.consume_while(|c| c.is_numeric()).parse::<u8>().unwrap();     // parse blue
        self.consume_while(|c| !c.is_numeric());                                   // parse whitespace and comma

        let a = self.consume_while(|c| c.is_numeric()).parse::<u8>().unwrap();     // parse alpha
        self.consume_while(|c| !c.is_numeric());                                   // parse whitespace and comma

        Color { r, g, b, a }
    }

    fn parse_value(&mut self) -> Value {
        if char::is_numeric(self.next_char()) {
            let numeric_value = self.consume_while(|c| c.is_numeric()).parse::<f32>().unwrap();
            // TODO: didn't parse unit because we are expecting px for now
            // let unit_value = self.consume_while(|c| c != ';' || !c.is_whitespace());
            Value::Numeric(numeric_value, Unit::Px)
        } else if self.starts_with("rgba(") {
            let color = self.parse_rgba();
            Value::ColorValue(Color { r: color.r, g: color.g, b: color.b, a: color.a })
        } else {
            let keyword_value = self.consume_while(|c| c != ';' || !c.is_whitespace());
            Value::Keyword(keyword_value)
        }
    }

    fn parse_property(&mut self) -> Property {
        let property_name = self.consume_while(|c| c != ':');
        self.consume_char(); // consume :
        self.consume_whitespace();
        let property_value = self.parse_value();
        self.consume_whitespace();
        self.consume_char(); // consume ;
        Property {
            name: property_name,
            value: property_value,
        }
    }

    fn parse_declarations(&mut self) -> Vec<Property> {
        let mut properties: Vec<Property> = Vec::new();
        self.consume_char(); // consume {
        self.consume_whitespace();
        while self.next_char() != '}' {
            properties.push(self.parse_property());
            self.consume_whitespace();
        }
        self.consume_char(); // consume }
        properties
    }

    fn parse_rule(&mut self) -> Rule {
        let selector = self.parse_selector();
        self.consume_whitespace();
        let declarations = self.parse_declarations();
        Rule {
            selector,
            declarations
        }
    }

    fn parse_rules(&mut self) -> Vec<Rule> {
        self.consume_whitespace();
        let mut rules: Vec<Rule> = Vec::new();
        while !self.eof() {
            rules.push(self.parse_rule());
            self.consume_whitespace();
        }
        rules
    }

    fn parse_stylesheet(&mut self) -> StyleSheet {
        StyleSheet {
            styles: self.parse_rules(),
        }
    }
}

