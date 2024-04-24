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
    value: String,
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
    fn parse_property(&mut self) -> Property {
        let property_name = self.consume_while(|c| c != ':');
        self.consume_char(); // consume :
        self.consume_whitespace();
        let property_value = self.consume_while(|c| c != ';');
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

