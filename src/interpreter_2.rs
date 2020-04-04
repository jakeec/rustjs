use std::collections::HashMap;

use crate::operators::{OP_ADD, OP_SUB};
use crate::types::{Function, Num, Type};

struct Interpreter<'a> {
    scope_stack: Vec<Vec<char>>,
    current: usize,
    lookahead: usize,
    value_table: HashMap<String, Type<'a>>,
    function_table: HashMap<String, Function<'a>>,
}

impl<'a> Interpreter<'a> {
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
    fn lookahead(&self) -> char {
        let lookahead = self.scope()[self.lookahead];
        lookahead
    }

    fn match_char(&mut self, char_to_match: char) {
        let current = self.current();
        if current != char_to_match {
            panic!("Expected {} found {}", char_to_match, current);
        }
    }

    fn matches_char(&self, char_to_match: char) -> bool {
        self.lookahead() == char_to_match
    }

    fn is_digit(&self) -> bool {
        self.lookahead().is_digit(10)
    }

    fn is_alpha(&self) -> bool {
        self.lookahead().is_alphabetic()
    }

    fn is_alphanum(&self) -> bool {
        self.is_alpha() || self.is_digit()
    }

    fn whitespace(&mut self) {
        while self.lookahead() == ' ' {
            self.match_char(' ');
        }
    }
}

trait Expression<'a> {
    fn expression(&mut self) -> Type<'a>;
    fn term(&mut self) -> Type<'a>;
    fn add(&mut self, prev: Type<'a>) -> Type<'a>;
    fn sub(&mut self, prev: Type<'a>) -> Type<'a>;
    fn ident(&mut self) -> String;
    fn string(&mut self) -> String;
    fn number(&mut self) -> f64;
}

impl<'a> Expression<'a> for Interpreter<'a> {
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

    fn ident(&mut self) -> String {
        let mut ident = String::new();
        while self.is_alphanum() {
            ident.push(self.current());
        }
        ident
    }

    fn term(&mut self) -> Type<'a> {
        if self.is_digit() {
            return Type::Number(Num::F64(self.number()));
        }

        if self.matches_char('"') {
            return Type::TextString(self.string());
        }

        if self.is_alpha() {
            let ident = self.ident();
            if ident == "undefined" {
                return Type::Undefined;
            }
            let value = &self.value_table[&ident];
            match value {
                Type::Function(name) => {
                    if self.matches_char('(') {
                        self.match_char('(');
                        self.match_char(')');
                    // execute function body and return result
                    } else {
                        // return function
                    }
                }
                _ => return value.clone(),
            }
        }

        Type::Undefined
    }

    fn add(&mut self, prev: Type<'a>) -> Type<'a> {
        self.match_char(OP_ADD);
        self.whitespace();
        let term = self.expression();
        prev + term
    }

    fn sub(&mut self, prev: Type<'a>) -> Type<'a> {
        self.match_char(OP_SUB);
        self.whitespace();
        let term = self.expression();
        prev - term
    }

    fn expression(&mut self) -> Type<'a> {
        // if self.matches_char('(') {
        //     return Type::Function(self.arrow_function());
        // }

        let prev = self.term();
        self.whitespace();
        if [OP_ADD, OP_SUB].contains(&self.lookahead()) {
            return match self.lookahead() {
                OP_ADD => self.add(prev),
                OP_SUB => self.sub(prev),
                _ => Type::Undefined,
            };
        }

        prev
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn term_string() {
        let code = "\"jake\";";
        let mut interpreter = Interpreter::new(code.chars().collect());
        let result = interpreter.term();
        match result {
            Type::TextString(string) => {
                assert_eq!(string, "jake");
            }
            _ => panic!("Expected string!"),
        }
    }

    #[test]
    fn term_number() {
        let code = "12;";
        let mut interpreter = Interpreter::new(code.chars().collect());
        let result = interpreter.term();
        match result {
            Type::Number(Num::F64(number)) => {
                assert_eq!(number, 12f64);
            }
            _ => panic!("Expected string!"),
        }
    }

    #[test]
    fn term_variable() {
        let code = "myVar;";
        let mut interpreter = Interpreter::new(code.chars().collect());
        interpreter
            .value_table
            .insert(String::from("myVar"), Type::Number(Num::F64(10f64)));
        let result = interpreter.term();
        match result {
            Type::Number(Num::F64(number)) => {
                assert_eq!(number, 10f64);
            }
            _ => panic!("Expected string!"),
        }
    }

    #[test]
    fn expression_add_two_numbers() {
        let code = "10 + 10;";
        let mut interpreter = Interpreter::new(code.chars().collect());
        let result = interpreter.expression();
        match result {
            Type::Number(Num::F64(number)) => {
                assert_eq!(number, 20f64);
            }
            actual => panic!("Expected string found {:?}!", actual),
        }
    }

    #[test]
    fn expression_add_two_strings() {
        let code = "\"10\" + \"10\";";
        let mut interpreter = Interpreter::new(code.chars().collect());
        let result = interpreter.expression();
        match result {
            Type::TextString(string) => {
                assert_eq!(string, "1010");
            }
            actual => panic!("Expected string found {:?}!", actual),
        }
    }

    #[test]
    fn expression_add_number_to_string() {
        let code = "\"10\" + 10;";
        let mut interpreter = Interpreter::new(code.chars().collect());
        let result = interpreter.expression();
        match result {
            Type::TextString(string) => {
                assert_eq!(string, "1010");
            }
            actual => panic!("Expected string found {:?}!", actual),
        }
    }

    #[test]
    fn expression_add_string_to_number() {
        let code = "10 + \"10\";";
        let mut interpreter = Interpreter::new(code.chars().collect());
        let result = interpreter.expression();
        match result {
            Type::TextString(string) => {
                assert_eq!(string, "1010");
            }
            actual => panic!("Expected string found {:?}!", actual),
        }
    }

    #[test]
    fn expression_add_number_to_undefined() {
        let code = "undefined + 10;";
        let mut interpreter = Interpreter::new(code.chars().collect());
        let result = interpreter.expression();
        match result {
            Type::Number(num) => {
                assert_eq!(num, Num::NaN);
            }
            actual => panic!("Expected NaN found {:?}!", actual),
        }
    }

    #[test]
    fn expression_subtract_string_from_string() {
        let code = "\"jake\" - \"e\";";
        let mut interpreter = Interpreter::new(code.chars().collect());
        let result = interpreter.expression();
        match result {
            Type::Number(num) => {
                assert_eq!(num, Num::NaN);
            }
            actual => panic!("Expected NaN found {:?}!", actual),
        }
    }

    #[test]
    fn expression_subtract_number_from_number() {
        let code = "20 - 17;";
        let mut interpreter = Interpreter::new(code.chars().collect());
        let result = interpreter.expression();
        match result {
            Type::Number(Num::F64(num)) => {
                assert_eq!(num, 3f64);
            }
            actual => panic!("Expected 3 found {:?}!", actual),
        }
    }
}
