
use std::num;

#[derive(Debug)]
pub enum YSONError {
    ParseError,
    ParseNoTerminatingQuote(String),
    ParseBadEscapeCode(String),
    ParseExpect(String),
    ParseUnexpectedEndOfInput,
    ParseFloatError(num::ParseFloatError),
    ValueNotMapError,
    ValueNotArrayError,
    ValueNotScalarError,
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
    
    fn require(&mut self, tok: &str) -> Result<(), YSONError> {
        self.eat_white();
        if self.src.starts_with(tok) {
            self.src = self.src.split_at(tok.len()).1;
            self.eat_white();
            Ok(())
        }
        else {
            Err(YSONError::ParseExpect(tok.to_string()))
        }
    }
    
    fn allow(&mut self, tok: &str) {
        self.eat_white();
        if self.src.starts_with(tok) {
            self.src = self.src.split_at(tok.len()).1;
            self.eat_white();
        }
    }
    
    pub fn parse_map(&mut self) -> Result<YSONValue<'a>, YSONError> {
        let mut values = Vec::new();
        try!(self.require("{"));
        while !self.src.starts_with('}') {
            let key = try!(self.parse_scalar());
            try!(self.require(":"));
            let value = try!(self.parse_value());
            self.allow(",");
            values.push((key, value));
        }
        try!(self.require("}"));
        println!("parsed map");
        Ok(YSONValue::Map{values: values})
    }
    
    pub fn parse_array(&mut self) -> Result<YSONValue<'a>, YSONError> {
        let mut values = Vec::new();
        try!(self.require("["));
        while !self.src.starts_with(']') {
            values.push(try!(self.parse_value()));
            self.allow(",");
        }
        try!(self.require("]"));
        println!("parsed array");
        Ok(YSONValue::Array{values: values})
    }
    
    pub fn parse_quoted_string(&mut self) -> Result<YSONValue<'a>, YSONError> {
        // Parse series of characters starting and ending with '"', including whitespace and
        // newlines.
        let mut chars = self.src.char_indices();
        chars.next();// skip opening '"'
        let mut string_value = String::new();
        loop {
            let (idx, this_char) = match chars.next() {
                None => return Err(YSONError::ParseNoTerminatingQuote(string_value)),
                Some((idx, ch)) => ((idx, ch))
            };
            
            if this_char == '\\' {
                string_value.push(match chars.next() {
                    Some((_, 'r')) => '\r',
                    Some((_, 'n')) => '\n',
                    Some((_, 't')) => '\t',
                    Some((_, '"')) => '"',
                    Some((_, '\'')) => '\'',
                    Some((_, '\\')) => '\\',
                    _ => return Err(YSONError::ParseBadEscapeCode(this_char.to_string()))
                })
            }
            else if this_char == '"' {
                self.src = self.src.split_at(idx + 1).1;
                println!("parsed quoted string: {}", string_value);
                return Ok(YSONValue::String{value: string_value});
            }
            else {
                string_value.push(this_char);
            }
        }
    }
    
    pub fn parse_literal(&mut self) -> Result<YSONValue<'a>, YSONError> {
        // Parse series of characters consisting of any character but: ":,]}", or
        // whitespace.
        match self.src.find(|ch| char::is_whitespace(ch) || ":,]}".contains(ch)) {
            None => return Err(YSONError::ParseUnexpectedEndOfInput),
            Some(idx) => {
                let (s, src2) = self.src.split_at(idx);
                self.src = src2;
                println!("parsed literal: {}", s);
                return Ok(YSONValue::RawScalar{value: s});
            }
        }
    }
    
    pub fn parse_scalar(&mut self) -> Result<YSONValue<'a>, YSONError> {
        if self.src.starts_with('"') {
            return self.parse_quoted_string();
        }
        else {
            return self.parse_literal();
        };
    }
    
    pub fn parse_value(&mut self) -> Result<YSONValue<'a>, YSONError> {
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
    pub fn parse(src: &'a str) -> Result<YSONValue<'a>, YSONError> {
        let mut parser = YSONParser{src: src};
        parser.parse_value()
    }
    
    pub fn map_values(&self) -> Result<&Vec<(YSONValue<'a>, YSONValue<'a>)>, YSONError> {
        match *self {
            YSONValue::Map{ref values} => Ok(values),
            _ => Err(YSONError::ValueNotMapError)
        }
    }
    
    pub fn map_value(&self, key: &str) -> Result<&YSONValue<'a>, YSONError> {
        for x in self.map_values().unwrap() {
            if x.0.scalar_value().unwrap() == key {
                return Ok(&(x.1));
            }
        }
        return Err(YSONError::ValueNotMapError);
    }
    
    pub fn array_values(&self) -> Result<&Vec<YSONValue<'a>>, YSONError> {
        match *self {
            YSONValue::Array{ref values} => Ok(values),
            _ => Err(YSONError::ValueNotArrayError)
        }
    }
    
    pub fn scalar_value(&self) -> Result<&str, YSONError> {
        match *self {
            YSONValue::RawScalar{ref value} => Ok(value),
            YSONValue::String{ref value} => Ok(&value),
            _ => Err(YSONError::ValueNotScalarError)
        }
    }
    
    pub fn f64_value(&self) -> Result<f64, YSONError> {
        match self.scalar_value() {
            Ok(scalar) => return scalar.parse::<f64>().map_err(YSONError::ParseFloatError),
            Err(e) => return Err(e)
        };
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
