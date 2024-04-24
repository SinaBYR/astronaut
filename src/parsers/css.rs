pub struct Selector {
    name: String,
    selector_type: SelectorType
}

pub enum SelectorType {
    Id,
    Class,
    Tag,
}

pub struct SelectorData {
    name: String,
}

pub struct Property {
    name: String,
    value: String,
}

pub struct Rule {
    selector: Selector,
    declarations: Vec<Property>,
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
    //                  UTILS END
    // ---------------------------------------------
    // ---------------------------------------------

    fn parse_id(&mut self) -> Selector {
        self.consume_char(); // parses hashtag #
        let selector_name = self.consume_while(|c| char::is_whitespace(c) || c == '#');
        Selector {
            name: selector_name,
            selector_type: SelectorType::Id,
        }
    }

    fn parse_class(&mut self) -> Selector {
        self.consume_char(); // parses dot .
        let selector_name = self.consume_while(|c| char::is_whitespace(c) || c == '.');
        Selector {
            name: selector_name,
            selector_type: SelectorType::Class,
        }
    }

    fn parse_tag(&mut self) -> Selector {
        let selector_name = self.consume_while(|c| char::is_whitespace(c) || c == '{');
        Selector {
            name: selector_name,
            selector_type: SelectorType::Tag,
        }
    }

    fn parse_selector(&mut self) -> Selector {
        self.consume_whitespace();
        match self.next_char() {
            '#' => self.parse_id(),
            '.' => self.parse_class(),
            _   => self.parse_tag(),
        }
    }
}

