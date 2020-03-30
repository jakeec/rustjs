enum Operator {
    Equals,
}

struct Interpreter {
    source: Vec<char>,
    counter: usize,
    lookahead: Option<char>,
}

impl Interpreter {
    fn new(source: Vec<char>) -> Self {
        Self {
            source,
            counter: 0,
            lookahead: None,
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

    fn ident(&mut self) {
        let mut ident = String::new();
        while self.get_lookahead() != ' ' {
            ident.push(self.get_lookahead());
            self.get_char();
        }
        println!("New identifier created: {}", ident);
        self.whitespace();
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

    fn number(&mut self) {
        let mut num = String::new();
        while self.get_lookahead() != ' ' && self.get_lookahead() != ';' {
            num.push(self.get_digit());
            self.get_char();
        }

        self.whitespace();
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

    fn expression(&mut self) {
        if self.is_digit() {
            self.number();
        }

        if self.matches_char('"') {
            self.string();
        }

        if self.is_alpha() {
            println!("arithmetic!");
        }

        self.eol();
    }

    fn statement(&mut self) {
        self.keyword("var");
        self.ident();
        self.operator(Operator::Equals);
        self.expression();
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
