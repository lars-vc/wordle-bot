use regex::Regex;
use std::fs;
use std::fs::File;
use std::io::{self, prelude::*, BufRead, BufReader};
fn main() {
    //known letters = []
    //excluded letters = []
    //
    let mut excluded = vec![];
    let mut known = vec![];
    let stdin = io::stdin();
    let mut word = vec!['.', '.', '.', '.', '.'];
    let mut v = get_words();
    while true {
        let mut iter = stdin.lock().lines();
        let line = iter.next().unwrap().unwrap();
        let mut nextkey = false;
        let mut pos = 0;
        for i in line.bytes() {
            if nextkey {
                word[pos] = i as char;
                known.push(i as char);
                nextkey = false;
                pos += 1;
            } else if i == 27 {
                nextkey = true;
            } else {
                //check capital
                if (i as char).is_lowercase() {
                    excluded.push(i as char);
                } else {
                    known.push((i as char).to_lowercase().collect::<Vec<_>>()[0]);
                }
                pos += 1;
            }
            println!("{}", i);
        }
        println!("{:?}", known);
        println!("{:?}", excluded);
        println!("{:?}", word);
        let mut tss = vec![];
        for w in v {
            // bit of a hack to always add é idk
            let re = Regex::new(format!("^[^{}é]*$", excluded.iter().collect::<String>()).as_str())
                .unwrap();
            // word does not have excluded letters
            if re.is_match(&w) {
                let mut b = true;
                for k in &known {
                    if !w.contains(&k.to_string()) {
                        b = false;
                        break;
                    }
                }
                if b {
                    let re2 =
                        Regex::new(format!("^{}*$", word.iter().collect::<String>()).as_str())
                            .unwrap();
                    if re2.is_match(&w) {
                        tss.push(w);
                    }
                }
            }
            //if rege
        }
        v = tss;
        println!("{:?}", v);
        //repeat
        if v.len() == 1 {
            break;
        }
        // print recommendation
    }
}

fn get_words() -> Vec<String> {
    let file = File::open("src/words.txt").unwrap();
    let reader = BufReader::new(file);
    let mut words = vec![];
    for line in reader.lines() {
        words.push(line.unwrap());
    }
    return words;
}
