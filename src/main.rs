use std::collections::HashMap;
use std::fs;
use std::fs::File;

const START_INTEGER : u8 = b'i';
const START_LIST : u8 = b'l';
const START_DICTIONARY : u8 = b'd';
const END_INTEGER_LIST_DICTIONARY : u8= b'e';
const INTEGER_MINUS_SIGN : u8 = b'-';
const ZERO   : u8 = b'0';
const END_SIZE_OF_STRING : u8 = b':';

const START_STRING :u8 = b'0';
const END_STRING: u8 = b'9';

#[derive(Debug)]
enum BencodeValue {
    Integer(BencodeInteger),
    String(BencodeString),
    List(BencodeList),
    Dictionary(BencodeDictionary),
    Error(String),
}

#[derive(Debug)]
struct BencodeInteger {
    value: Vec<u8>
}
impl BencodeInteger {
    fn get_integer(&self) -> i64 {
        if self.validate(){
            let num_str =  std::str::from_utf8(&self.value).expect("Not \
            possible to parse it into a string");
            let num = num_str.parse::<i64>().expect("Not possible to parse integer");
        }
        0
    }
    fn validate(&self) -> bool {
        true
    }
    fn new(value: Vec<u8>) -> Self {
        Self { value }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct BencodeString {
    value: Vec<u8>
}
impl BencodeString {
    fn get_string(&self) -> String {
        if self.validate() {
            std::str::from_utf8(&self.value)
                .unwrap()
                .to_string()
        } else {
            "".to_string()
        }
    }

    fn validate(&self) -> bool {
        true
    }

    fn new(value: Vec<u8>) -> Self {
        Self { value }
    }
}


#[derive(Debug)]
struct BencodeList {
    elements: Vec<BencodeValue>,
}

impl BencodeList {
    fn new(elements: Vec<BencodeValue>) -> Self {
        Self { elements }
    }
}

#[derive(Debug)]
struct BencodeDictionary {
    elements: HashMap<BencodeString, BencodeValue>
}

impl BencodeDictionary {
    fn new(elements: HashMap<BencodeString, BencodeValue>) -> Self {
        Self { elements }
    }
}

fn correct_use_of_zeroes(input_slice : &[u8]) -> bool {
    if input_slice[0] == INTEGER_MINUS_SIGN {
        return input_slice[1] != ZERO;
    }
    if input_slice[0] == ZERO {
        return input_slice[1] == END_INTEGER_LIST_DICTIONARY;
    }
    true
}


fn parse_string(input_slice : &[u8]) -> (BencodeValue, usize)  {
    if input_slice.len() < 1 {
        return (BencodeValue::Error("Stringa troppo corta".to_string()),0)
    }
    let pos_end_string  = input_slice.iter().position(|&b| b == END_SIZE_OF_STRING).unwrap_or(input_slice.len());
    let string_size = std::str::from_utf8(&input_slice[..pos_end_string])
                            .ok()
                            .and_then(|s| s.parse::<usize>().ok())
                            .unwrap_or(0);
    (BencodeValue::String(BencodeString::new(input_slice[pos_end_string +1 ..pos_end_string + 1 + string_size].to_vec())), pos_end_string + string_size)
}

fn parse_integer(input_slice : &[u8]) -> (BencodeValue, usize) {
    if input_slice.len() < 1 {
        return (BencodeValue::Error("Integer troppo corto".to_string()),0)
    }
    let end_of_integer = input_slice.iter().position(|&b| b == END_INTEGER_LIST_DICTIONARY).unwrap_or(input_slice.len());
    (BencodeValue::Integer(BencodeInteger::new(input_slice[1..end_of_integer].to_vec())), end_of_integer )
}

fn parse_dictionary(input_slice : &[u8]) -> (BencodeValue, usize) {
    let mut parsed_dict = HashMap::<BencodeString, BencodeValue>::new();

    let mut pos =1;
    while pos < input_slice.len() {
        if input_slice[pos] == END_INTEGER_LIST_DICTIONARY { break}
        let key = parse_string(&input_slice[pos..]);
        let start_index_value = pos + key.1 +1;
        let value = parse_bencode(&input_slice[start_index_value..]);
        pos  = pos + key.1 + value.1 + 1;
        let BencodeValue::String(key_str) = key.0 else { panic!("expected string"); };
        parsed_dict.insert(key_str, value.0);
        pos +=1;
    }

    (BencodeValue::Dictionary(BencodeDictionary::new(parsed_dict)), 0 )

}


fn parse_list(input_slice : &[u8]) -> (BencodeValue, usize) {
    let mut pos = 1;
    let mut values  = Vec::<BencodeValue>::new();
    if input_slice.len() < 1 {
        return (BencodeValue::Error("Error in parsing".to_string()),0)
    }
    while pos < input_slice.len() {
        //fix me se c'e' uan e a meta'; e non ci dovrebbe stare allora e' un errore
        if input_slice[pos] == END_INTEGER_LIST_DICTIONARY { break}
        let pars_result = parse_bencode(&input_slice[pos..]);
        pos += pars_result.1;
        values.push(pars_result.0);
        pos += 1;
    }
    (BencodeValue::List(BencodeList::new(values)), pos + 1 )
}

fn parse_bencode(input_slice : &[u8]) -> (BencodeValue, usize) {
    match input_slice[0] {
        START_INTEGER => parse_integer(&input_slice),
        x if x > START_STRING && x < END_STRING => parse_string(&input_slice),
        START_LIST => parse_list(&input_slice),
        START_DICTIONARY => parse_dictionary(&input_slice),
        _ => (BencodeValue::Error("Error in parsing".to_string()),0),
    }
}


fn main() -> std::io::Result<()> {
    let becode_input = fs::read("/Users/teospadotto/Documents/project/Rust/study/resource/integer_example.bencode")?;
    let oggetto_deserializzato = parse_bencode(&becode_input);

    println!("{:?}", oggetto_deserializzato.0);

    Ok(())
}