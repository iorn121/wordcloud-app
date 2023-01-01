extern crate serde_json;
use wasm_bindgen::prelude::*;
mod utils;
use lindera::tokenizer::Tokenizer;
use lindera::tokenizer::{TokenizerConfig,DictionaryConfig};
use lindera::{LinderaResult,DictionaryKind};
use lindera_core::{viterbi::Mode};
use serde::{Serialize,Deserialize};
use std::collections::HashMap;


#[derive(Serialize,Deserialize,Debug)]
pub struct Contributor {
    value: String
}

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
    pub fn console_log(s: &str);
}

#[wasm_bindgen]
pub fn sums(value: i32) -> i32 {
    let mut a: i32 = 0;
    for i in 1..value+1 {
        a += i;
    }
    a
}

#[wasm_bindgen]
pub fn test(value: &JsValue){
    let vs: Contributor=value.into_serde().unwrap();
    console_log(&vs.value);
}

#[wasm_bindgen]
pub fn wakati(input_csv_data: &str)->String{
    let dictionary = DictionaryConfig {
        kind: Some(DictionaryKind::IPADIC),
        path: None,
    };
    let config = TokenizerConfig {
        dictionary,
        user_dictionary: None,
        mode: Mode::Normal,
        with_details: false,
    };
    let mut tokenizer= Tokenizer::new(config).unwrap();

    let tokens=tokenizer.tokenize(&input_csv_data).unwrap();
    let mut ret_token=Vec::<String>::new();
    for token in tokens{
        console_log(&token.text);
        ret_token.push(String::from(token.text));
    }
    ret_token.join("\n")
}

#[wasm_bindgen]
pub fn main()->String {
let test="大きな栗の木の下で";
let ans=wakati(test);
ans
}