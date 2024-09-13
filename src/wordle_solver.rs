use std::{
    collections::{HashMap, HashSet},
    fs::File,
    hash::{Hash, Hasher},
    io::{BufRead, BufReader},
};

pub enum LetterState {
    Absent,
    Present,
    Correct,
}

impl Into<LetterState> for String {
    fn into(self) -> LetterState {
        match self.as_str() {
            "absent" => LetterState::Absent,
            "present" => LetterState::Present,
            "correct" => LetterState::Correct,
            _ => LetterState::Absent,
        }
    }
}
#[derive(Debug)]
pub struct WordleSolver {
    pub present_letters: HashSet<Letter>,
    pub absent_letters: HashSet<String>,
    pub final_word: [String; 5],
    pub words: Vec<String>,
    letter_freq_table: HashMap<String, f32>,
}

impl WordleSolver {
    pub fn new() -> Self {
        WordleSolver {
            present_letters: HashSet::<Letter>::new(),
            absent_letters: HashSet::<String>::new(),
            final_word: core::array::from_fn(|_| "".to_string()),
            words: Self::import_words(),
            letter_freq_table: Self::init_frequency_table(),
        }
    }

    pub fn insert_letter(
        &mut self,
        letter: impl Into<String>,
        state: impl Into<LetterState>,
        index: Option<usize>,
    ) -> bool {
        match state.into() {
            LetterState::Absent => self.absent_letters.insert(letter.into()),
            LetterState::Present => self.present_letters.insert(Letter {
                letter: letter.into(),
                invalid_indices: Some(vec![index.expect("No index")]),
            }),
            LetterState::Correct => {
                let l: String = letter.into();
                self.final_word[index.expect("No index")] = l.clone();
                self.present_letters.insert(Letter {
                    letter: l.clone(),
                    invalid_indices: Some((0..6).filter(|x| *x != index.unwrap()).collect()),
                })
            }
        }
    }
    pub fn filter_words(&mut self) {
        self.words = self
            .words
            .iter()
            .map(|word| word.clone())
            .filter(|word| {
                !word.chars().any(|c| self.absent_letters.contains(&String::from(c)))    //remove the know absent_letters letters
                        && self.present_letters.is_subset(&HashSet::from_iter(                    //check if the word contains the letter in the known letter set
                            word.chars().map(|c| Letter::from(String::from(c))),
                        ))
                        && word.char_indices().any(|(i, c)| {           //check if the the word has the letter in the indices where it is known the letter cannot be there 
                            self.present_letters.iter().any(|l| {
                                l.letter == String::from(c)
                                    && l.invalid_indices.is_some()
                                    && !l.invalid_indices.as_ref().unwrap().contains(&i)
                            })
                            && word.char_indices().all(|(i, _)| self.final_word[i] == String::from(word.chars().nth(i).unwrap()) || self.final_word[i] == "") //check for letters in the correct positions
                        })
            })
            .collect();
    }

    fn import_words() -> Vec<String> {
        let reader = BufReader::new(File::open("src/words.csv").expect("File not found"));
        let mut words = vec![];
        for line in reader.lines() {
            if line.is_ok() {
                words.push(line.unwrap());
            }
        }
        words
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

    pub fn find_optimal_word(&self) -> String {
        let (mut score, mut index) = (0.0, 0);

        for (i, word) in self.words.iter().enumerate() {
            let mut current_score = 0.0;
            word.chars().for_each(|c| {
                current_score += self
                    .letter_freq_table
                    .get(&String::from(c))
                    .expect("No value")
            });
            if current_score >= score {
                score = current_score;
                index = i;
            }
        }
        println!("Words: {:?}", self.words);
        return self.words[index].clone();
    }
}

#[derive(Debug)]
pub struct Letter {
    letter: String,
    invalid_indices: Option<Vec<usize>>,
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
            invalid_indices: None,
        }
    }
}
