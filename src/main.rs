use enigo::{Direction::Click, Enigo, InputError, Key, Keyboard, Settings};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::{
    collections::HashSet,
    fs::File,
    hash::RandomState,
    io::{BufRead, BufReader},
    vec,
};
use thirtyfour::prelude::*;

/*
    Things to do:
        - Allow the bot to make multiple guess
        - Check for edge cases (double letter, etc..)




*/
static BEST_COMMON_STARTING_WORDS: [&str; 5] = ["adieu", "audio", "trace", "arise", "crane"];

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:9515", caps).await?;
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    let file = File::open("src/words.csv").unwrap();
    let reader = BufReader::new(file);
    let mut words = vec![];
    let frequency_table = init_frequency_table();
    for line in reader.lines() {
        if line.is_ok() {
            words.push(line.unwrap());
        }
    }

    //Go to wordle page
    driver
        .goto("https://www.nytimes.com/games/wordle/index.html")
        .await?;

    //Click the Play button
    let play_button = driver.find(By::Css("[data-testid='Play']")).await?;
    play_button.click().await?;
    tokio::time::sleep(std::time::Duration::from_millis(200)).await; //allow pop up to show

    //Close the pop up
    let close_button = driver
        .find(By::ClassName("Modal-module_closeIcon__TcEKb"))
        .await?;
    close_button.click().await?;
    tokio::time::sleep(std::time::Duration::from_millis(200)).await; //wait for pop up to close

    //Get the board
    let board_elem = driver
        .find(By::ClassName("Board-module_board__jeoPS"))
        .await?;
    let rows = board_elem
        .find_all(By::ClassName("Row-module_row__pwpBq"))
        .await?;

    let guess = BEST_COMMON_STARTING_WORDS[3];
    type_word(guess, &mut enigo).unwrap();
    tokio::time::sleep(std::time::Duration::from_millis(2000)).await; //wait for animation to finish

    let squares = rows[0]
        .find_all(By::ClassName("Tile-module_tile__UWEHN"))
        .await?;

    #[derive(Debug)]
    struct Letter {
        letter: String,
        invalid: Option<Vec<usize>>,
    }
    impl PartialEq for Letter {
        fn eq(&self, other: &Self) -> bool {
            self.letter.eq(&other.letter)
        }
        fn ne(&self, other: &Self) -> bool {
            self.letter.ne(&other.letter)
        }
    }

    impl Eq for Letter {}

    impl Hash for Letter {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.letter.hash(state);
        }
    }

    impl From<String> for Letter {
        fn from(value: String) -> Self {
            Letter {
                letter: value,
                invalid: None,
            }
        }
    }

    let mut states = vec![];
    let mut final_word: [String; 5] = core::array::from_fn(|_| "".to_string());
    let mut known_letters = HashSet::new();
    let mut absent_letters = HashSet::new();
    for (i, s) in squares.iter().enumerate() {
        let state = s.attr("data-state").await?.expect("No state");
        let letter = s.inner_html().await?;
        match state.as_str() {
            "absent" => absent_letters.insert(letter.clone()),
            "present" => known_letters.insert(Letter {
                letter: letter.clone(),
                invalid: Some(vec![i]),
            }),
            "correct" => {
                final_word[i] = letter.clone();
                true
            }
            _ => true,
        };
        states.push(state);
    }

    let h: HashSet<Letter, RandomState> =
        HashSet::from_iter(words[1].chars().map(|c| Letter::from(String::from(c))));
    println!("{:?}", known_letters.is_subset(&h));
    println!("{h:?}");

    words = words
        .iter()
        .map(|word| word.clone())
        .filter(|word| {
            !word.chars().any(|c| absent_letters.contains(&String::from(c)))    //remove the know absent_letters letters
                && known_letters.is_subset(&HashSet::from_iter(                    //check if the word known_letters the letter in the known letter set
                    word.chars().map(|c| Letter::from(String::from(c))),
                ))
                && word.char_indices().any(|(i, c)| {           //check if the the word has the letter in the indices where it is known the letter cannot be there 
                    known_letters.iter().any(|l| {
                        l.letter == String::from(c)
                            && l.invalid.is_some()
                            && !l.invalid.as_ref().unwrap().contains(&i)
                    })
                    && word.char_indices().any(|(i, _)| final_word[i] == String::from(word.chars().nth(i).unwrap())) //check for letters in the correct positions
                })
        })
        .collect();
    println!("{:?}", final_word);
    println!("{:?}", known_letters);
    println!("{:?}", absent_letters);
    println!("{:?}", words);
    let next_guess = find_optimal_word(&words, &frequency_table);
    absent_letters.insert("o".to_string());
    absent_letters.insert("n".to_string());
    known_letters.insert(Letter {
        letter: "l".to_string(),
        invalid: Some(vec![2]),
    });
    final_word[0] = "a".to_string();
    println!("{:?}", next_guess);

    words = words
        .iter()
        .map(|word| word.clone())
        .filter(|word| {
            !word.chars().any(|c| absent_letters.contains(&String::from(c)))    //remove the know absent_letters letters
            && known_letters.is_subset(&HashSet::from_iter(                    //check if the word known_letters the letter in the known letter set
                word.chars().map(|c| Letter::from(String::from(c))),
            ))
            && word.char_indices().any(|(i, c)| {           //check if the the word has the letter in the indices where it is known the letter cannot be there 
                known_letters.iter().any(|l| {
                    l.letter == String::from(c)
                        && l.invalid.is_some()
                        && !l.invalid.as_ref().unwrap().contains(&i)
                })
                && word.char_indices().all(|(i, _)| final_word[i] == String::from(word.chars().nth(i).unwrap()) || final_word[i] == "") //check for letters in the correct positions
            })
        })
        .collect();

    let next_guess = find_optimal_word(&words, &frequency_table);
    println!("{:?}", final_word);
    println!("{:?}", known_letters);
    println!("{:?}", absent_letters);
    println!("{:?}", words);
    println!("{:?}", next_guess);

    // Always explicitly close the browser.
    driver.quit().await?;

    Ok(())
}

fn type_word(word: &'static str, enigo: &mut Enigo) -> Result<(), InputError> {
    for letter in word.chars() {
        println!("{letter}");
        enigo.key(Key::Unicode(letter), Click)?
    }
    enigo.key(Key::Return, Click).unwrap();
    Ok(())
}

fn init_frequency_table() -> HashMap<String, f32> {
    let mut map = HashMap::<String, f32>::new();
    map.insert("e".to_string(), 56.88);
    map.insert("a".to_string(), 43.31);
    map.insert("r".to_string(), 38.64);
    map.insert("i".to_string(), 38.45);
    map.insert("o".to_string(), 36.51);
    map.insert("t".to_string(), 35.43);
    map.insert("n".to_string(), 33.92);
    map.insert("s".to_string(), 29.23);
    map.insert("l".to_string(), 27.98);
    map.insert("c".to_string(), 23.13);
    map.insert("u".to_string(), 18.51);
    map.insert("d".to_string(), 17.25);
    map.insert("p".to_string(), 16.14);
    map.insert("m".to_string(), 15.36);
    map.insert("h".to_string(), 15.31);
    map.insert("g".to_string(), 12.59);
    map.insert("b".to_string(), 10.56);
    map.insert("f".to_string(), 9.24);
    map.insert("y".to_string(), 9.06);
    map.insert("w".to_string(), 6.57);
    map.insert("k".to_string(), 5.61);
    map.insert("v".to_string(), 5.13);
    map.insert("x".to_string(), 1.48);
    map.insert("z".to_string(), 1.39);
    map.insert("j".to_string(), 1.01);
    map.insert("q".to_string(), 1.00);

    return map;
}

fn find_optimal_word(words: &Vec<String>, freq_map: &HashMap<String, f32>) -> String {
    let (mut score, mut index) = (0.0, 0);

    for (i, word) in words.iter().enumerate() {
        let mut current_score = 0.0;
        word.chars()
            .for_each(|c| current_score += freq_map.get(&String::from(c)).expect("No value"));
        if current_score >= score {
            score = current_score;
            index = i;
        }
    }
    return words[index].clone();
}
