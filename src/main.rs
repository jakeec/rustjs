use std::collections::HashMap;

enum Operator {
    Equals,
    Add,
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

struct Function {
    arguments: HashMap<String, Types>,
    code: String,
}

struct Interpreter {
    source: Vec<char>,
    counter: usize,
    lookahead: Option<char>,
    lookahead_counter: usize,
    lookup_table: HashMap<String, Types>,
    scope_table: HashMap<String, Function>,
    scope_index: usize,
}

impl Interpreter {
    fn new(source: Vec<char>) -> Self {
        Self {
            source,
            counter: 0,
            lookahead: None,
            lookahead_counter: 0,
            lookup_table: HashMap::new(),
            scope_table: HashMap::new(),
            scope_index: 0,
        }
    }

    fn get_char(&mut self) -> Option<char> {
        self.counter += 1;
        if self.source[self.counter] == '\x03' {
            return None;
        }
        self.lookahead = Some(self.source[self.counter]);
        self.lookahead
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

        if self.counter < self.source.len() - 1 {
            self.get_char();
        }
    }

    fn optionally_match_char(&mut self, c: char) {
        if self.get_lookahead() == c {
            self.match_char(c);
        }
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

    fn lookahead_is_not(&mut self, chars: &[char]) -> bool {
        for c in chars {
            if self.get_lookahead() == *c {
                return false;
            }
        }

        true
    }

    fn ident(&mut self) -> String {
        let mut ident = String::new();
        while self.lookahead_is_not(&[' ', ';', ',']) {
            ident.push(self.get_lookahead());
            self.get_char();
        }
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
            Add => {
                if op != String::from("+") {
                    panic!("Expected operator '+' found {}", op);
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
            self.operator(Operator::Add);
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

        if self.matches_char('(') {
            return Types::Function(self.arrow_function());
        }

        Types::Undefined
    }

    fn arrow_function(&mut self) -> String {
        self.match_char('(');
        while self.get_lookahead() != ')' {
            self.ident();
        }
        self.match_char(')');
        self.whitespace();
        self.match_char('=');
        self.match_char('>');
        self.whitespace();
        self.match_char('{');
        self.optionally_match_char('\n');
        let mut function_body = String::new();
        while self.lookahead_is_not(&['}']) {
            match self.get_char() {
                Some(c) => function_body.push(c),
                None => panic!("End of stream"),
            }
        }
        // self.whitespace();
        // self.block();
        self.match_char('}');
        self.scope_table.insert(
            self.scope_index.to_string(),
            Function {
                arguments: HashMap::new(),
                code: function_body,
            },
        );
        let function_id = self.scope_index.to_string();
        self.scope_index += 1;
        function_id
    }

    fn statement(&mut self) {
        self.keyword("var");
        let ident = self.ident();
        self.operator(Operator::Equals);
        let value = self.expression();
        println!("STATEMENT: {}, {:?}", ident, value);
        self.lookup_table.insert(ident, value);
        // match value {
        //     Types::Number(_) => self.lookup_table.insert(ident, value),
        //     Types::Function(_) => self.lookup_table.insert(ident, value),
        //     _ => None,
        // };
        self.eol();
    }

    fn new_line(&mut self) {
        if self.counter >= self.source.len() - 1 {
            return;
        }
        self.match_char('\n');
        self.whitespace();
    }

    fn init(&mut self) {
        self.lookahead = Some(self.source[self.counter]);
    }

    fn block(&mut self) {
        while self.lookahead_is_not(&['}']) {
            println!("{:?}", self.get_lookahead());
            self.statement();
            self.new_line();
        }
    }

    fn program(&mut self) {
        while self.counter < self.source.len() - 1 {
            self.statement();
            self.new_line();
        }
    }
}

fn main() {
    let input = "var jakeVar = 10;
    var b = 20;
    var c = jakeVar + b;
    var myFunc = () => {
        var scopedVar = 10;
    };
    \x03";
    let mut interpreter = Interpreter::new(input.chars().collect());
    interpreter.init();
    interpreter.program();
}
