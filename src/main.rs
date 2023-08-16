use core::time;
use std::{fmt::Display, path::Path, time::Instant};

#[derive(Debug, PartialEq, Eq, Clone)]
struct Literal {
    contents: String,
    count: usize,
    token: Option<String>,
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{contents:15} <=> {token:?} (count: {count})",
            contents = self.contents,
            token = self.token,
            count = self.count
        )
    }
}

impl Literal {
    fn new() -> Self {
        Self {
            contents: String::with_capacity(30),
            count: 1,
            token: None,
        }
    }
    fn get_token(&self) -> String {
        match &self.token {
            Some(tok) => tok.clone(),
            None => self.contents.clone(),
        }
    }
    fn push_char(&mut self, ch: char) {
        self.contents.push(ch);
    }
    fn get_len(&self) -> usize {
        self.contents.len()
    }
    fn increment_count(&mut self) {
        self.count += 1;
    }
    fn get_max_characters_saved(&self) -> usize {
        (self.contents.len() - 1) * self.count
    }
    fn total_characters_saved(&self) -> isize {
        let content_length = self.contents.len();
        let token_length = if let Some(tok) = &self.token {
            tok.len()
        } else {
            return -1;
        };
        // println!(
        //     "tokenLen: {token_length}, contentLen: {content_length} => content: {content}",
        //     content = self.contents
        // );
        //total no. of characters saved
        //the final text file will have words and their token for decoding so that must be decreased
        // from the total no. of characters saved.... if the system works properly, the final text file
        // will only have the order of words sorted from max characters saved to min charaters saved
        // and the tokens will be identified based on the order
        let header_map = content_length;
        if content_length > token_length {
            let chars_saved: isize = (((content_length - token_length) * self.count) as isize
                - header_map as isize) as isize;
            return chars_saved;
        } else {
            let chars_saved = (((token_length - content_length) * self.count) as isize
                - header_map as isize) as isize;
            return -chars_saved;
        }
    }

    fn characters_saved(&self, tok: &str) -> isize {
        let content_length = self.contents.len();
        let token_length = tok.len();

        // println!(
        //     "tokenLen: {token_length}, contentLen: {content_length} => content: {content}",
        //     content = self.contents
        // );

        //total no. of characters saved
        //the final text file will have words and their token for decoding so that must be decreased
        // from the total no. of characters saved.... if the system works properly, the final text file
        // will only have the order of words sorted from max characters saved to min charaters saved
        // and the tokens will be identified based on the order
        let header_map = content_length;
        if content_length > token_length {
            let chars_saved: isize = (((content_length - token_length) * self.count) as isize
                - header_map as isize) as isize;
            return chars_saved;
        } else {
            let chars_saved = (((token_length - content_length) * self.count) as isize
                - header_map as isize) as isize;
            return -chars_saved;
        }
    }

    fn set_token(&mut self, tok: String) -> bool {
        if self.characters_saved(&tok) < 0 {
            return false;
        }
        self.token = Some(tok);
        true
    }
}

fn decode(input: String) -> String {
    //read header and make it a list
    // take a token
    // find its word
    // add it to output
    let mut char_iter = input.chars();
    let mut header_map = Vec::<String>::new();
    let mut output = Vec::<String>::new();
    let mut curr_word = String::new();
    for ch in char_iter.clone() {
        //scan till semicolon and add each word to a list -> this list is the descending order of words from most chars saved
        if ch == ';' {
            char_iter.next();
            break;
        } else {
            if ch == ',' {
                header_map.push(curr_word);
                curr_word = String::new();
                char_iter.next();
                continue;
            }
            curr_word.push(ch);
            char_iter.next();
        }
        // println!("ch: {ch} and index: {idx}");
    }
    // println!("header map: {header_map:?}");

    header_map.push(curr_word);
    curr_word = String::new();
    let mut peekable_char_iter = char_iter.peekable();
    while let Some(ch) = peekable_char_iter.next() {
        /*
        G @ Jo M CE I &strings ** k
        */
        match ch {
            '&' => {
                while let Some(c) = peekable_char_iter.next() {
                    // println!("it's &:{c}");
                    match c {
                        '&' => continue, //for some reason iterator doesn't go to next element {while let Some((_, c)) = peekable_char_iter.next() }
                        ' ' => {
                            // println!("pushing: {curr_word}");

                            output.push(curr_word);
                            curr_word = String::new();
                            break;
                        }
                        _ => curr_word.push(c),
                    }
                }
            }
            'A'..='Z' | 'a'..='z' | '@' => {
                // println!("it's a letter or @:{ch}");

                curr_word.push(ch);
                continue;
            }
            _ => {
                if !curr_word.is_empty() {
                    // println!("it's not a letter or @ or &:{ch}");

                    let mut curr_word_peekable_iter = curr_word.chars().peekable();
                    match curr_word_peekable_iter.next().unwrap() {
                        //check if the element in curr_word is an alphabet or '@' then it has a word and not a symbol
                        'A'..='Z' | 'a'..='z' | '@' => {
                            // println!("tok: {curr_word}");
                            let decoded_string_idx = decode_token(&curr_word);
                            let decoded_string = header_map[decoded_string_idx].to_string();
                            // println!("decoded idx: {decoded_string_idx}");
                            // println!("decoded: {decoded_string}");
                            output.push(decoded_string);
                            curr_word.clear();
                        }
                        _ => {
                            match curr_word_peekable_iter.peek() {
                                Some(c) => {
                                    if c.is_ascii_alphabetic() || *c == '@' {
                                        output.push(curr_word);
                                        curr_word = String::new();
                                    } else {
                                        curr_word.push(ch);
                                    }
                                }
                                None => {
                                    curr_word.push(ch);
                                    output.push(curr_word);
                                    break;
                                    // curr_word = String::new();
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    // println!("output: {output:?}");
    output.into_iter().collect()
}

// next patterns => <ws><ws><ws> : -3 |  <Symbol><Symbol><Symbol> : &*4
// this model is case sensitive

fn decode_token(tok: &str) -> usize {
    if tok.len() == 0 {
        return 0;
    }
    let tok_iter = tok.bytes();

    let mut num: Vec<u8> = Vec::new();
    for ch in tok_iter {
        match ch {
            b'@'..=b'Z' => num.push(ch - 64),
            b'a'..=b'z' => num.push(ch - 71),
            _ => unreachable!(),
        }
    }

    let num_iter = num.into_iter().rev();
    let mut sum: usize = 0;
    const BASE: usize = 52;
    for (idx, num) in num_iter.enumerate() {
        sum += (num as usize * BASE.pow(idx as u32)) as usize;
    }

    sum
}

fn base52_token(mut num: usize) -> String {
    const BASE: usize = 52;
    let mut rem: u8 = (num % BASE) as u8;
    num /= BASE;
    let mut base52_vec = vec![rem];
    let mut base52 = String::with_capacity(30);

    while num >= BASE {
        rem = (num % BASE) as u8;
        num /= BASE;
        base52_vec.push(rem);
    }
    if num > 0 {
        base52_vec.push(num as u8);
    }

    // 10/5/2/1
    // [0,1,0,1] => 1010
    // 104/2
    // [0,2] => 20

    // let msb = *base52_vec.last().unwrap();
    for num in base52_vec.into_iter() {
        // let mut factor: u8 = 0;
        //MSB of the number cannot be zero unless num is 0, so if the current no. is msb
        // then its character will be - 1 for A, 2 for B and so on
        // if num == msb {
        //     factor = 1;
        // }

        let ch: u8 = match num {
            0..=25 => num + 65 - 1,
            _ => num + 71,
        };
        base52.push(ch as char);
    }
    base52.chars().rev().collect::<String>()
}

fn main() {
    let start_time = Instant::now();
    let mut args = std::env::args();
    //skip first argument which is the absolute program path
    args.next();
    let file_name = args.next().unwrap();
    println!("file name: {file_name}");
    let contents = std::fs::read_to_string(Path::new(&file_name)).unwrap();

    let mut list = Vec::<Literal>::new();
    let mut curr_word = Literal::new();
    let mut char_iter = contents.chars();
    for char in char_iter.clone() {
        match char {
            'a'..='z' | 'A'..='Z' => {
                curr_word.push_char(char);
            }
            _ => {
                if curr_word.get_len() != 0 {
                    if let Some(literal) =
                        list.iter_mut().find(|x| x.contents == curr_word.contents)
                    {
                        literal.increment_count();
                    } else {
                        list.push(curr_word);
                    }
                    curr_word = Literal::new();
                } else {
                    let mut i: usize = 0;
                    while i < contents.len() && !char.is_alphanumeric() {
                        char_iter.next();
                        i += 1;
                    } //while
                } //else
            }
        } //match
    } //for

    list.sort_by_key(|l| l.get_max_characters_saved());
    // println!("{list:?}");

    let list_iter = list.iter_mut().rev().enumerate();
    let mut skipped = 0;
    for (idx, item) in list_iter {
        let next_index = idx - skipped; //if a token is skipped it will be used for the next item on the list
        let token = base52_token(next_index);
        // println!(
        //     "generated token: {token} : for '{content}' with index: {idx} and count: {count}",
        //     content = item.contents,
        //     count = item.count
        // );
        // println!("{item:?}");

        if !item.set_token(token) {
            skipped += 1;
        }
    }
    // println!("{list:?}");

    for literal in list.iter() {
        println!(
            "{literal} => saved {saved} chars",
            saved = literal.total_characters_saved()
        );
    }

    let total_chars_saved = list.iter().fold(0, |acc, x| {
        let mut saved = x.total_characters_saved();
        if saved < 0 {
            saved = 0;
        }
        acc + saved
    });
    println!("\nsaved: {total_chars_saved}");
    let mut input_strings = Vec::<String>::new();
    let mut curr_word = String::new();
    for ch in contents.chars() {
        match ch {
            'a'..='z' | 'A'..='Z' => {
                curr_word.push(ch);
            }
            _ => {
                if curr_word.len() != 0 {
                    input_strings.push(curr_word);
                    curr_word = String::new();
                } else {
                    let mut i: usize = 0;

                    while i < contents.len() && !ch.is_alphanumeric() {
                        char_iter.next();
                        i += 1;
                    } //while
                } //else
            }
        } //match
    } //for

    // println!("{input_strings:?}");

    //refine the string replace 3 or more white-spaces with ~n...
    // eg: 3 ws => ~3, 10=> ~10
    // if there is 10 ws \n 5 ws.... its not going to be ~10\n~5 instead it will be => \n~5

    println!("total no. of words: {length}", length = input_strings.len());
    println!("total no. of distinct words: {length}", length = list.len());

    let mut output = String::new();
    let header_list = list.iter().rev();

    for (_, literal) in header_list.enumerate() {
        if literal.token == None {
            continue;
        }
        let string = format!("{word},", word = literal.contents);

        output.push_str(&string);
    }
    output.push(';');

    for string in input_strings {
        if let Some(literal) = list.iter().find(|l| l.contents == string) {
            //if the token does'nt save characters, it shouldnt be used
            // make token an option variable
            // println!("{literals}");
            let tok = match &literal.token {
                Some(tok) => tok.clone(),
                None => {
                    format!("&{string}")
                }
            };
            output.push_str(&tok);
        } else {
            output.push_str(&string);
        }
        output.push(' ');
    }

    std::fs::write("output.txt", output.clone()).unwrap();
    decode(output);

    println!("completed in {elapsed:?}", elapsed = start_time.elapsed());
}

#[test]
fn comp() {
    for i in 0..1000000 {
        let tok = base52_token(i);
        let decoded_num = decode_token(&tok);
        println!("index is {i} <=> decoded_num {decoded_num}");

        assert_eq!(i, decoded_num);
    }
}
