use std::fs::File;
use std::io;
use std::io::{BufReader, BufRead, Error};

fn main() -> Result<(), Error> {
    let path = "wordlist.txt";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let mut wordlist: Vec<String> = Vec::new();

    println!("Lade alle Wörter, bitte warten!");
    for line in buffered.lines() {
        wordlist.push(line?);
    }
    println!("Alle Wörer wurden geladen. Gebe etwas ein um das Spiel zu starten");
    let mut start_dingsdabums = String::new();
    io::stdin().read_line(&mut start_dingsdabums).expect("Could not read the line");
    drop(start_dingsdabums);

    let mut input = String::new();
    for x in 0..6 {
        println!("Bitte gebe das Wort ein:");

        let mut is_valid = false;
        while is_valid == false {
            io::stdin().read_line(&mut input).expect("Could not read the line");

            if input.trim().len() == 5 {
                is_valid = true;
                println!("yup");
            }
            else {
                println!("Das eingegebene Wort ist zu lang oder zu kurz");
            }

            input.clear();
        }
    }

    Ok(())
}