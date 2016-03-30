
use std::num;

#[derive(Debug)]
pub enum YSONError {
    ParseFloatError(num::ParseFloatError),
    ScalarNotFloatError
}

pub enum YSONValue<'a> {
    Map{values: Vec<(YSONValue<'a>, YSONValue<'a>)>},
    Array{values: Vec<YSONValue<'a>>},
    RawScalar{value: &'a str},
    String{value: String}
}

struct YSONParser<'a> {
    src: &'a str
}

impl<'a> YSONParser<'a> {
    fn eat_white(&mut self) {
        self.src = self.src.trim_left();
    }
    
    fn require(&mut self, tok: &str) {
        self.eat_white();
        if self.src.starts_with(tok) {
            self.src = self.src.split_at(tok.len()).1;
            self.eat_white();
        }
        else {
            panic!("Expected token {} not found.", tok)
        }
    }
    
    fn allow(&mut self, tok: &str) {
        self.eat_white();
        if self.src.starts_with(tok) {
            self.src = self.src.split_at(tok.len()).1;
            self.eat_white();
        }
    }
    
    pub fn parse_map(&mut self) -> YSONValue<'a> {
        let mut values = Vec::new();
        self.require("{");
        while !self.src.starts_with('}') {
            let key = self.parse_scalar();
            self.require(":");
            let value = self.parse_value();
            self.allow(",");
            values.push((key, value));
        }
        self.require("}");
        println!("parsed map");
        YSONValue::Map{values: values}
    }
    
    pub fn parse_array(&mut self) -> YSONValue<'a> {
        let mut values = Vec::new();
        self.require("[");
        while !self.src.starts_with(']') {
            values.push(self.parse_value());
            self.allow(",");
        }
        self.require("]");
        println!("parsed array");
        YSONValue::Array{values: values}
    }
    
    pub fn parse_quoted_string(&mut self) -> YSONValue<'a> {
        // Parse series of characters starting and ending with '"', including whitespace and
        // newlines.
        let mut chars = self.src.char_indices();
        let mut prev_char = '\\';
        loop {
            let (idx, this_char) = match chars.next() {
                None => panic!("Expected terminating \" not found."),
                Some((idx, ch)) => ((idx, ch))
            };
            if this_char == '"' && prev_char != '\\' {
                let (s, src2) = self.src.split_at(idx + 1);
                self.src = src2;
                println!("parsed quoted string: {}", s);
                return YSONValue::RawScalar{value: s};
            }
            else {
                prev_char = this_char;
            }
        }
    }
    
    pub fn parse_literal(&mut self) -> YSONValue<'a> {
        // Parse series of characters consisting of any character but: ":,]}", or
        // whitespace.
        match self.src.find(|ch| char::is_whitespace(ch) || ":,]}".contains(ch)) {
            None => panic!("End of input reached while parsing literal."),
            Some(idx) => {
                let (s, src2) = self.src.split_at(idx);
                self.src = src2;
                println!("parsed literal: {}", s);
                return YSONValue::RawScalar{value: s};
            }
        }
    }
    
    pub fn parse_scalar(&mut self) -> YSONValue<'a> {
        if self.src.starts_with('"') {
            return self.parse_quoted_string();
        }
        else {
            return self.parse_literal();
        };
    }
    
    pub fn parse_value(&mut self) -> YSONValue<'a> {
        self.eat_white();
        if self.src.starts_with('{') {
            return self.parse_map();
        }
        else if self.src.starts_with('[') {
            return self.parse_array();
        }
        else {
            return self.parse_scalar();
        };
    }
}


impl<'a> YSONValue<'a> {
    pub fn parse(src: &'a str) -> YSONValue<'a> {
        let mut parser = YSONParser{src: src};
        parser.parse_value()
    }
    
    pub fn map_values(&self) -> Option<&Vec<(YSONValue<'a>, YSONValue<'a>)>> {
        match *self {
            YSONValue::Map{ref values} => Some(values),
            _ => None
        }
    }
    
    pub fn map_value(&self, key: &str) -> Option<&YSONValue<'a>> {
        for x in self.map_values().unwrap() {
            if x.0.scalar_value().unwrap() == key {
                return Some(&(x.1));
            }
        }
        return None;
    }
    
    pub fn array_values(&self) -> Option<&Vec<YSONValue<'a>>> {
        match *self {
            YSONValue::Array{ref values} => Some(values),
            _ => None
        }
    }
    
    pub fn scalar_value(&self) -> Option<&str> {
        match *self {
            YSONValue::RawScalar{ref value} => Some(value),
            YSONValue::String{ref value} => Some(&value),
            _ => None
        }
    }
    
    pub fn f64_value(&self) -> Result<f64, YSONError> {
        let scal_val = match self.scalar_value() {
            Some(scalar) => scalar,
            _ => return Err(YSONError::ScalarNotFloatError)
        };
        scal_val.parse::<f64>().map_err(YSONError::ParseFloatError)
    }
    
    pub fn display(&self) {
        match *self {
            YSONValue::Map{ref values} => {
                print!("{{\n");
                for x in values {
                    x.0.display();
                    print!(":");
                    x.1.display();
                    print!(",\n");
                }
                print!("}}\n");
            },
            YSONValue::Array{ref values} => {
                print!("[\n");
                for x in values {
                    x.display();
                    print!(",\n");
                }
                print!("]\n");
            },
            YSONValue::RawScalar{ref value} => {
                print!("{}", value);
            },
            YSONValue::String{ref value} => {
                print!("{}", value);
            }
        }
    }
}
