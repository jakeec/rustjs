use std::collections::HashMap;

use crate::types::{Function, Types};

struct Interpreter {
    scope_stack: Vec<Vec<char>>,
    current: usize,
    lookahead: usize,
    value_table: HashMap<String, Types>,
    function_table: HashMap<String, Function>,
}

impl Interpreter {
    pub fn new(source: Vec<char>) -> Self {
        let mut scope_stack = vec![source];
        Self {
            scope_stack,
            current: 0,
            lookahead: 0,
            value_table: HashMap::new(),
            function_table: HashMap::new(),
        }
    }

    /// Returns the current scope.
    fn scope(&self) -> &Vec<char> {
        &self.scope_stack[self.scope_stack.len() - 1]
    }

    /// Consumes the current character and returns it.
    fn current(&mut self) -> char {
        let current = self.scope()[self.current];
        self.current += 1;
        self.lookahead += 1;
        current
    }

    /// Gets the next character but doesn't consume it. Increments the lookahead counter.
    fn lookahead(&mut self) -> char {
        let lookahead = self.scope()[self.lookahead];
        lookahead
    }

    fn match_char(&mut self, char_to_match: char) {
        let current = self.current();
        if current != char_to_match {
            panic!("Expected {} found {}", char_to_match, current);
        }
    }

    fn matches_char(&mut self, char_to_match: char) -> bool {
        self.lookahead() == char_to_match
    }

    fn is_digit(&mut self) -> bool {
        self.lookahead().is_digit(10)
    }

    fn is_alpha(&mut self) -> bool {
        self.lookahead().is_alphabetic()
    }
}

trait Expression {
    fn expression(&mut self) -> Types;
    fn string(&mut self) -> String;
    fn number(&mut self) -> f64;
}

impl Expression for Interpreter {
    fn string(&mut self) -> String {
        let mut string = String::new();
        self.match_char('"');
        while self.lookahead() != '"' {
            string.push(self.current());
        }
        self.match_char('"');
        string
    }

    fn number(&mut self) -> f64 {
        let mut number = String::new();
        while self.is_digit() {
            number.push(self.current());
        }
        number.parse().unwrap()
    }

    fn expression(&mut self) -> Types {
        if self.is_digit() {
            return Types::Number(self.number());
        }

        if self.matches_char('"') {
            return Types::TextString(self.string());
        }

        // if self.is_alpha() {
        //     let result = self.add();
        //     match result {
        //         Some(res) => return Types::Number(res),
        //         None => return Types::Undefined,
        //     }
        // }

        // if self.matches_char('(') {
        //     return Types::Function(self.arrow_function());
        // }

        Types::Undefined
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_interpret_expression_string() {
        let code = "\"jake\";";
        let mut interpreter = Interpreter::new(code.chars().collect());
        let result = interpreter.expression();
        match result {
            Types::TextString(string) => {
                assert_eq!(string, "jake");
            }
            _ => panic!("Expected string!"),
        }
    }

    #[test]
    fn should_interpret_expression_number() {
        let code = "12;";
        let mut interpreter = Interpreter::new(code.chars().collect());
        let result = interpreter.expression();
        match result {
            Types::Number(number) => {
                assert_eq!(number, 12f64);
            }
            _ => panic!("Expected string!"),
        }
    }
}
