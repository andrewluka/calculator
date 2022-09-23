use proc_macro2::{
    TokenStream,
    TokenTree::{Group, Ident, Literal, Punct},
};

pub enum OutputType {
    Char,
    Num,
}

pub struct Config {
    output: OutputType,
    // discriminant values to reject (raise a compiler error over)
    exceptions: Vec<u8>,
}

impl Config {
    pub fn output(&self) -> &OutputType {
        &self.output
    }

    pub fn exceptions(&self) -> &Vec<u8> {
        &self.exceptions
    }
}

fn parse_literal(lit: String) -> u8 {
    let is_char = lit.starts_with('\'') && lit.ends_with('\'') && lit.len() == 3;
    let is_byte = lit.starts_with("b'") && lit.ends_with('\'') && lit.len() == 4;

    if let Ok(val) = lit.parse::<u8>() {
        val
    } else if is_char || is_byte {
        let mut chars = lit.chars();

        // ignore starting '\'' or 'b'
        chars.next().unwrap();

        if is_char {
            chars.next().unwrap() as u8
        // not char, so byte
        } else {
            // ignore '\''; 'b' was already ignored
            chars.next().unwrap();
            chars.next().unwrap() as u8
        }
    } else {
        panic!("unknown value in 'except' in striminant macro attribute: {lit}")
    }
}

pub fn parse_attribute_arguments(args: TokenStream) -> Config {
    let mut iterator = args.into_iter();

    let mut all_exceptions: Vec<u8> = Vec::new();
    let mut output_type: Option<OutputType> = None;

    while let Some(token) = iterator.next() {
        match token {
            // argument name (all the arguments must be named)
            Ident(ident) => {
                let ident = ident.to_string();
                match &ident[..] {
                    "except" => {
                        // equal sign; ignore it
                        iterator.next().unwrap();

                        let exceptions = iterator.next().unwrap();
                        let mut exceptions: Vec<u8> = if let Group(exceptions) = exceptions {
                            exceptions
                                .stream()
                                .into_iter()
                                .filter_map(|exception| {
                                    match exception {
                                        Literal(lit) => Some(parse_literal(lit.to_string())),
                                        Punct(_) => None,
                                        _ => panic!(
                                            "Unknown value for 'except' in striminant macro attribute: {}", 
                                            exception
                                        )
                                    }
                                })
                                .collect()
                        } else {
                            panic!(
                                "expected array for the 'except' in striminant macro attribute, found {}", 
                                exceptions
                            )
                        };

                        all_exceptions.append(&mut exceptions);
                    }
                    "output" => {
                        // equal sign; ignore it
                        iterator.next().unwrap();

                        let output_type_str = iterator.next().unwrap().to_string();

                        output_type = match &output_type_str[..] {
                            "\"num\"" => Some(OutputType::Num),
                            "\"char\"" => Some(OutputType::Char),
                            _ => panic!("Unknown output type: {output_type_str}"),
                        };
                    }
                    _ => panic!("invalid argument to striminant macro: {}", ident),
                }
            }
            _ => {}
        }
    }

    let output_type = match output_type {
        Some(o) => o,
        None => OutputType::Char,
    };

    Config {
        output: output_type,
        exceptions: all_exceptions,
    }
}
