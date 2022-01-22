use rand::seq::SliceRandom;
use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
fn main() {
    //init
    let mut excluded = String::new();
    let mut known = String::new();
    let stdin = io::stdin();
    let mut word = vec!['.', '.', '.', '.', '.'];
    let mut v = get_words();
    let suggestions = vec!["irate", "flops", "munch"];

    println!("Meta starter word: {}", suggestions[0]);
    println!("First word:");
    for i in 1..5 {
        //get input
        let mut iter = stdin.lock().lines();
        let line = iter.next().unwrap().unwrap();
        let mut nextkey = false;
        let mut pos = 0;
        //parse input
        for i in line.bytes() {
            let c = i as char;
            if nextkey {
                // prev byte was an alt
                word[pos] = c;
                known.push(c);
                nextkey = false;
                pos += 1;
            } else if i == 27 {
                // 27 indicates an alt press
                nextkey = true;
            } else {
                //check capital
                if c.is_lowercase() {
                    excluded.push(c);
                } else {
                    known.push(c.to_lowercase().collect::<Vec<_>>()[0]);
                }
                pos += 1;
            }
        }
        let mut tss = vec![];
        //do word search
        for w in v {
            // bit of a hack to always add é so the regex isnt empty
            let excludedcheck = Regex::new(format!("^[^{}é]*$", excluded).as_str()).unwrap();
            // word does not have excluded letters
            if excludedcheck.is_match(&w) {
                let mut b = true;
                for k in known.chars() {
                    if !w.contains(k) {
                        b = false;
                        break;
                    }
                }
                // word has all the known letters
                if b {
                    let wordcheck =
                        Regex::new(format!("^{}$", word.iter().collect::<String>()).as_str())
                            .unwrap();
                    if wordcheck.is_match(&w) {
                        tss.push(w);
                    }
                }
            }
        }
        v = tss;
        if v.len() == 1 {
            println!("Match found! {}", v[0]);
            break;
        }
        if v.len() <= 5 {
            println!(
                "\nPick one of these (bot chooses {}): {:?}\n",
                v.choose(&mut rand::thread_rng()).unwrap(),
                v
            );
        } else if i <= 2 {
            println!("\nAll matches: {:?}\n", v);
            println!("Suggestion: {}", suggestions[i]);
        } else {
            println!(
                "\nPick one of these (bot chooses {}): {:?}\n",
                v.choose(&mut rand::thread_rng()).unwrap(),
                v
            );
        }
        println!("Next word:");
    }
}

fn get_words() -> Vec<String> {
    let file = File::open("/home/larsvc/.config/wordle-bot/words.txt").unwrap();
    let reader = BufReader::new(file);
    let mut words = vec![];
    for line in reader.lines() {
        words.push(line.unwrap());
    }
    return words;
}
