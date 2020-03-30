use std::collections::HashMap;

enum Operator {
    Equals,
}

#[derive(Debug)]
enum Types {
    Null,
    Undefined,
    Number(f64),
    Boolean(bool),
    TextString(String),
    Object(HashMap<String, Types>),
    Function(String),
}

struct Interpreter {
    source: Vec<char>,
    counter: usize,
    lookahead: Option<char>,
    lookahead_counter: usize,
    lookup_table: HashMap<String, Types>,
}

impl Interpreter {
    fn new(source: Vec<char>) -> Self {
        Self {
            source,
            counter: 0,
            lookahead: None,
            lookahead_counter: 0,
            lookup_table: HashMap::new(),
        }
    }

    fn get_char(&mut self) {
        self.counter += 1;
        self.lookahead = Some(self.source[self.counter]);
    }

    fn match_char(&mut self, c: char) {
        match self.lookahead {
            Some(l) => {
                if l != c {
                    panic!("Expected {} found {}", c, l)
                }
            }
            None => panic!("Expected {} found nothing!", c),
        }

        self.get_char();
    }

    fn whitespace(&mut self) {
        while self.lookahead == Some(' ') {
            self.get_char();
        }
    }

    fn keyword(&mut self, keyword: &str) {
        for c in keyword.chars() {
            self.match_char(c);
        }

        self.whitespace();
    }

    fn get_lookahead(&mut self) -> char {
        match self.lookahead {
            Some(c) => c,
            None => panic!("End of input!"),
        }
    }

    fn ident(&mut self) -> String {
        let mut ident = String::new();
        while self.get_lookahead() != ' ' {
            ident.push(self.get_lookahead());
            self.get_char();
        }
        println!("New identifier created: {}", ident);
        self.whitespace();
        ident
    }

    fn operator(&mut self, operator: Operator) {
        use Operator::*;
        let mut op = String::new();
        while self.get_lookahead() != ' ' {
            op.push(self.get_lookahead());
            self.get_char();
        }

        match operator {
            Equals => {
                if op != String::from("=") {
                    panic!("Expected operator '=' found {}", op);
                }
            }
        }

        self.whitespace();
    }

    fn get_digit(&mut self) -> char {
        match self.lookahead {
            Some(l) => {
                if l.is_digit(10) {
                    l
                } else {
                    panic!("Expected number found {}", l)
                }
            }
            None => panic!("Expected number found nothing!"),
        }
    }

    fn number(&mut self) -> f64 {
        let mut num = String::new();
        while self.get_lookahead() != ' ' && self.get_lookahead() != ';' {
            num.push(self.get_digit());
            self.get_char();
        }

        self.whitespace();
        num.parse().unwrap()
    }

    fn eol(&mut self) {
        self.match_char(';');
        self.whitespace();
    }

    fn is_digit(&mut self) -> bool {
        if let Some(c) = self.lookahead {
            c.is_digit(10)
        } else {
            false
        }
    }

    fn matches_char(&mut self, c: char) -> bool {
        self.get_lookahead() == c
    }

    fn is_alpha(&mut self) -> bool {
        self.get_lookahead().is_alphabetic()
    }

    fn string(&mut self) {}

    fn add(&mut self) -> Option<f64> {
        self.lookahead_counter = self.counter;
        let first_ident = self.ident();
        if !self.matches_char('+') {
            None
        } else {
            let second_ident = self.ident();
            let a = &self.lookup_table[&first_ident];
            let b = &self.lookup_table[&second_ident];

            match (a, b) {
                (Types::Number(a_), Types::Number(b_)) => Some(a_ + b_),
                _ => panic!("NOT A VALID ADDITION!"),
            }
        }
    }

    fn expression(&mut self) -> Types {
        if self.is_digit() {
            return Types::Number(self.number());
        }

        if self.matches_char('"') {
            self.string();
        }

        if self.is_alpha() {
            let result = self.add();
            match result {
                Some(res) => return Types::Number(res),
                None => return Types::Undefined,
            }
        }

        Types::Undefined
    }

    fn statement(&mut self) {
        self.keyword("var");
        let ident = self.ident();
        self.operator(Operator::Equals);
        let value = self.expression();
        match value {
            Types::Number(_) => self.lookup_table.insert(ident, value),
            _ => None,
        };
        self.eol();
    }

    fn new_line(&mut self) {
        self.match_char('\n');
        self.whitespace();
    }

    fn init(&mut self) {
        self.lookahead = Some(self.source[self.counter]);
    }

    fn program(&mut self) {
        println!("{:?}", self.source);
        while self.counter < self.source.len() {
            println!("LOOKUP: {:?}", self.lookup_table);
            self.statement();
            self.new_line();
        }
    }
}

fn main() {
    let input = "var jakeVar = 10;
    var b = 20;
    var c = jakeVar + b";
    let mut interpreter = Interpreter::new(input.chars().collect());
    interpreter.init();
    interpreter.program();
}
