
pub enum YSONValue<'a> {
    Map{values: Vec<(YSONValue<'a>, YSONValue<'a>)>},
    Array{values: Vec<YSONValue<'a>>},
    Scalar{value: &'a str}
}

impl<'a> YSONValue<'a> {
    fn require(tok: &str, mut src: &'a str) -> &'a str {
        src = src.trim_left();
        if src.starts_with(tok) {
            src.split_at(tok.len()).1.trim_left()
        }
        else {
            panic!("Expected token {} not found.", tok)
        }
    }
    
    fn allow(tok: &str, mut src: &'a str) -> &'a str {
        src = src.trim_left();
        if src.starts_with(tok) {
            src.split_at(tok.len()).1.trim_left()
        }
        else {
            src
        }
    }
    
    pub fn parse_map(mut src: &str) -> (YSONValue, &str) {
        let mut values = Vec::new();
        src = YSONValue::require("{", src);
        while !src.starts_with('}') {
            let (key, src2) = YSONValue::parse_scalar(src);
            src = YSONValue::require(":", src2);
            let (value, src2) = YSONValue::parse(src);
            src = YSONValue::allow(",", src2);
            values.push((key, value));
        }
        YSONValue::require("}", src);
        println!("parsed map");
        (YSONValue::Map{values: values}, src)
    }
    
    pub fn parse_array(mut src: &str) -> (YSONValue, &str) {
        let mut values = Vec::new();
        src = YSONValue::require("[", src);
        while !src.starts_with(']') {
            let (value, src2) = YSONValue::parse(src);
            src = YSONValue::allow(",", src2);
            values.push(value);
        }
        YSONValue::require("]", src);
        println!("parsed array");
        (YSONValue::Array{values: values}, src)
    }
    
    pub fn parse_quoted_string(src: &str) -> (YSONValue, &str) {
        // Parse series of characters starting and ending with '"', including whitespace and
        // newlines.
        let mut chars = src.char_indices();
        let mut prev_char = '\\';
        loop {
            let (idx, this_char) = match chars.next() {
                None => panic!("Expected terminating \" not found."),
                Some((idx, ch)) => ((idx, ch))
            };
            if this_char == '"' && prev_char != '\\' {
                let (s, src) = src.split_at(idx + 1);
                println!("parsed quoted string: {}", s);
                return (YSONValue::Scalar{value: s}, src);
            }
            else {
                prev_char = this_char;
            }
        }
    }
    
    pub fn parse_literal(src: &str) -> (YSONValue, &str) {
        // Parse series of characters consisting of any character but: ":,]}", or
        // whitespace.
        match src.find(|ch| char::is_whitespace(ch) || ":,]}".contains(ch)) {
            None => panic!("End of input reached while parsing literal."),
            Some(idx) => {
                let (s, src) = src.split_at(idx);
                println!("parsed literal: {}", s);
                return (YSONValue::Scalar{value: s}, src);
            }
        }
    }
    
    pub fn parse_scalar(src: &str) -> (YSONValue, &str) {
        if src.starts_with('"') {
            return YSONValue::parse_quoted_string(src);
        }
        else {
            return YSONValue::parse_literal(src);
        };
    }
    
    pub fn parse(src: &str) -> (YSONValue, &str) {
        let src = src.trim_left();// strip leading whitespace
        if src.starts_with('{') {
            return YSONValue::parse_map(src);
        }
        else if src.starts_with('[') {
            return YSONValue::parse_array(src);
        }
        else {
            return YSONValue::parse_scalar(src);
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
            YSONValue::Scalar{ref value} => print!("{}", value)
        }
    }
}
