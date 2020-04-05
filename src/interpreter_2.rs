use std::collections::HashMap;

use crate::keywords::{KW_CONST, KW_FUNCTION, KW_LET, KW_VAR};
use crate::operators::{OP_ADD, OP_DIV, OP_EQ, OP_MUL, OP_SUB};
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

    fn matches_any(&self, chars_to_match: &[char]) -> bool {
        chars_to_match.contains(&self.lookahead())
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
    fn operation(&mut self, operator: char, prev: Type<'a>) -> Type<'a>;
    fn add(&mut self, prev: Type<'a>) -> Type<'a>;
    fn sub(&mut self, prev: Type<'a>) -> Type<'a>;
    fn mul(&mut self, prev: Type<'a>) -> Type<'a>;
    fn div(&mut self, prev: Type<'a>) -> Type<'a>;
    fn factor(&mut self) -> Type<'a>;
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
        self.whitespace();
        ident
    }

    fn expression(&mut self) -> Type<'a> {
        let mut prev = self.factor();
        while [OP_ADD, OP_SUB].contains(&self.lookahead()) {
            prev = match self.lookahead() {
                OP_ADD => self.add(prev),
                OP_SUB => self.sub(prev),
                _ => prev,
            };
        }

        prev
    }

    fn factor(&mut self) -> Type<'a> {
        let mut prev = Type::Undefined;
        if self.matches_char('(') {
            self.match_char('(');
            prev = self.expression();
            self.match_char(')');
        } else {
            prev = self.term();
        }
        prev = match self.lookahead() {
            OP_MUL => self.mul(prev),
            OP_DIV => self.div(prev),
            _ => prev,
        };

        prev
    }

    fn term(&mut self) -> Type<'a> {
        let mut ret = Type::Undefined;
        if self.is_digit() {
            ret = Type::Number(Num::F64(self.number()));
        } else if self.matches_any(&['"', '\'']) {
            ret = Type::TextString(self.string());
        } else if self.is_alpha() {
            let ident = self.ident();
            match &ident[..] {
                "undefined" => ret = Type::Undefined,
                id => {
                    let value = &self.value_table[id];
                    match value {
                        Type::Function(name) => {
                            // evaluate function and return result
                        }
                        _ => ret = value.clone(),
                    }
                }
            };
        }

        self.whitespace();

        ret
    }

    fn operation(&mut self, operator: char, prev: Type<'a>) -> Type<'a> {
        self.match_char(operator);
        self.whitespace();
        let this = self.factor();
        match operator {
            OP_ADD => prev + this,
            OP_SUB => prev - this,
            OP_MUL => prev * this,
            OP_DIV => prev / this,
            _ => Type::Undefined,
        }
    }

    fn add(&mut self, prev: Type<'a>) -> Type<'a> {
        println!("add");
        self.operation(OP_ADD, prev)
    }

    fn sub(&mut self, prev: Type<'a>) -> Type<'a> {
        println!("sub");
        self.operation(OP_SUB, prev)
    }

    fn mul(&mut self, prev: Type<'a>) -> Type<'a> {
        self.operation(OP_MUL, prev)
    }

    fn div(&mut self, prev: Type<'a>) -> Type<'a> {
        self.operation(OP_DIV, prev)
    }
}

trait Assign {
    fn function(&mut self);
    fn assign(&mut self);
}

impl<'a> Assign for Interpreter<'a> {
    fn function(&mut self) {
        if self.matches_char('f') {
            let ident = self.ident();
            match &ident[..] {
                KW_FUNCTION => {
                    self.whitespace();
                    let name = self.ident();
                    self.match_char('(');
                    self.match_char(')');
                    self.whitespace();
                    self.match_char('{');
                    self.match_char('\n');
                    let mut code_block = String::new();
                    while self.lookahead() != '}' {
                        code_block.push(self.current());
                    }
                    let function = Function {
                        arguments: HashMap::new(),
                        body: code_block,
                    };
                    self.function_table.insert(name, function);
                }
                _ => (),
            }
        } else if self.matches_char('(') {
        }
    }

    fn assign(&mut self) {
        let keyword = self.ident();
        match &keyword[..] {
            KW_VAR => {
                let id = self.ident();
                self.match_char(OP_EQ);
                self.whitespace();
                let value = self.expression();
                self.value_table.insert(id, value);
            }
            KW_CONST => {
                let id = self.ident();
                self.match_char(OP_EQ);
                self.whitespace();
                let value = self.expression();
                self.value_table.insert(id, value);
            }
            KW_LET => {
                let id = self.ident();
                self.match_char(OP_EQ);
                self.whitespace();
                let value = self.expression();
                self.value_table.insert(id, value);
            }
            id => {
                self.match_char(OP_EQ);
                self.whitespace();
                let value = self.expression();
                self.value_table.insert(String::from(id), value);
            }
        }
    }
}

trait Program {
    fn program(&mut self);
}

impl<'a> Program for Interpreter<'a> {
    fn program(&mut self) {
        loop {
            self.assign();
            if self.matches_char(';') {
                self.match_char(';');
            }
            if self.current >= self.scope_stack.last().unwrap().len() {
                break;
            }
            if self.matches_char('\n') {
                self.match_char('\n');
            }
            self.whitespace();
        }
    }
}

#[cfg(test)]
mod expression_tests {
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
        println!("{:?}", result);
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

    #[test]
    fn expression_multiple_operators() {
        let code = "20 - 17 + 7;";
        let mut interpreter = Interpreter::new(code.chars().collect());
        let result = interpreter.expression();
        match result {
            Type::Number(Num::F64(num)) => {
                assert_eq!(num, 10f64);
            }
            actual => panic!("Expected 3 found {:?}!", actual),
        }
    }

    #[test]
    fn expression_multiply_two_numbers() {
        let code = "3 * 3;";
        let mut interpreter = Interpreter::new(code.chars().collect());
        let result = interpreter.expression();
        match result {
            Type::Number(Num::F64(num)) => {
                assert_eq!(num, 9f64);
            }
            actual => panic!("Expected 3 found {:?}!", actual),
        }
    }

    #[test]
    fn expression_multiply_two_numbers_add_one() {
        let code = "3 * 3 + 4;";
        let mut interpreter = Interpreter::new(code.chars().collect());
        let result = interpreter.expression();
        match result {
            Type::Number(Num::F64(num)) => {
                assert_eq!(num, 13f64);
            }
            actual => panic!("Expected 3 found {:?}!", actual),
        }
    }

    #[test]
    fn expression_add_number_to_multiplied_numbers() {
        let code = "4 + 3 * 3;";
        let mut interpreter = Interpreter::new(code.chars().collect());
        let result = interpreter.expression();
        match result {
            Type::Number(Num::F64(num)) => {
                assert_eq!(num, 13f64);
            }
            actual => panic!("Expected 3 found {:?}!", actual),
        }
    }

    #[test]
    fn expression_lots_of_operators() {
        let code = "10 + 9 + 6 - 8 * 10 / 2 + 9 - 4;";
        let mut interpreter = Interpreter::new(code.chars().collect());
        let result = interpreter.expression();
        match result {
            Type::Number(Num::F64(num)) => {
                assert_eq!(num, -10f64);
            }
            actual => panic!("Expected 3 found {:?}!", actual),
        }
    }

    #[test]
    fn expression_parentheses() {
        let code = "4 * (4 + 2);";
        let mut interpreter = Interpreter::new(code.chars().collect());
        let result = interpreter.expression();
        match result {
            Type::Number(Num::F64(num)) => {
                assert_eq!(num, 24f64);
            }
            actual => panic!("Expected 3 found {:?}!", actual),
        }
    }

    #[test]
    fn expression_lots_of_operators_with_parentheses() {
        let code = "10 + 9 + 6 - 8 * 10 / (2 + 9 - 4);";
        let mut interpreter = Interpreter::new(code.chars().collect());
        let result = interpreter.expression();
        match result {
            Type::Number(Num::F64(num)) => {
                assert_eq!(num, 13.571428571428571f64);
            }
            actual => panic!("Expected 3 found {:?}!", actual),
        }
    }
}

#[cfg(test)]
mod assign_tests {
    use super::*;

    #[test]
    fn assign_number() {
        let source = "var jake = 26;";
        let mut interpreter = Interpreter::new(source.chars().collect());
        interpreter.assign();
        let value = interpreter.value_table.get("jake");
        match value.unwrap() {
            Type::Number(Num::F64(val)) => assert_eq!(*val, 26f64),
            actual => panic!("Expected f64 found {:?}", actual),
        }
    }

    #[test]
    fn assign_string() {
        let source = "var jake = \"jake\";";
        let mut interpreter = Interpreter::new(source.chars().collect());
        interpreter.assign();
        let value = interpreter.value_table.get("jake");
        match value.unwrap() {
            Type::TextString(val) => assert_eq!(*val, "jake"),
            actual => panic!("Expected string found {:?}", actual),
        }
    }

    #[test]
    fn assign_string_no_keyword() {
        let source = "jake = \"jake\";";
        let mut interpreter = Interpreter::new(source.chars().collect());
        interpreter.value_table.insert(
            String::from("jake"),
            Type::TextString(String::from("carrington")),
        );
        interpreter.assign();
        let value = interpreter.value_table.get("jake");
        match value.unwrap() {
            Type::TextString(val) => assert_eq!(*val, "jake"),
            actual => panic!("Expected string found {:?}", actual),
        }
    }
}

#[cfg(test)]
mod program_tests {
    use super::*;

    #[test]
    fn program_test() {
        let source = "var a = 10;
        let b = 20;
        const c = a + b;";
        let mut interpreter = Interpreter::new(source.chars().collect());
        interpreter.program();
        match interpreter.value_table.get("c").unwrap() {
            Type::Number(Num::F64(val)) => assert_eq!(*val, 30f64),
            actual => panic!("Expected 30 found {:?}", actual),
        }
    }

    #[test]
    fn function_test() {
        let source = "function myFunction() {
            var a = 10;
            var b = 20;
            var c = a + b;
        }";
        let mut interpreter = Interpreter::new(source.chars().collect());
        interpreter.function();
        println!("{:?}", interpreter.function_table);
        // panic!("PRINT");
    }
}
