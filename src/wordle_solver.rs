use std::{
    collections::{HashMap, HashSet},
    fs::File,
    hash::{Hash, Hasher},
    io::{BufRead, BufReader},
    vec,
};
#[derive(PartialEq, Eq)]
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

#[derive(Debug, Clone)]
pub struct WordleSolver {
    pub present_letters: Vec<String>,
    pub absent_letters: HashMap<String, Vec<usize>>,
    pub final_word: [String; 5],
    pub words: Vec<String>,
    letter_freq_table: HashMap<String, f32>,
}

impl WordleSolver {
    pub fn new() -> Self {
        WordleSolver {
            present_letters: Vec::<String>::new(),
            absent_letters: HashMap::<String, Vec<usize>>::new(),
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
            LetterState::Absent => {
                let l: String = letter.into();
                if !self.absent_letters.contains_key(&l) {
                    if !self.present_letters.contains(&l) || !self.final_word.contains(&l) {
                        self.absent_letters
                            .insert(l, (0..6).collect::<Vec<usize>>());
                    } else {
                        self.absent_letters
                            .insert(l, vec![index.expect("No Index value")]);
                    }
                } else {
                    let mut indicies = self
                        .absent_letters
                        .get(&l)
                        .expect("No vec of indicies")
                        .to_owned();
                    if let Some(i) = index {
                        indicies.push(i);
                    }
                    self.absent_letters.insert(l, indicies);
                }

                true
            } //check if the letter is in the present letters set and remove
            LetterState::Present => {
                let l: String = letter.into();
                self.present_letters.push(l.clone());
                if self.absent_letters.contains_key(&l) {
                    let mut indicies = self
                        .absent_letters
                        .get(&l)
                        .expect("No vec of indicies")
                        .to_owned();
                    if let Some(i) = index {
                        indicies.push(i);
                    }
                    self.absent_letters.insert(l, indicies);
                } else {
                    self.absent_letters
                        .insert(l, vec![index.expect("No Index")]);
                }

                true
            }
            LetterState::Correct => {
                let l: String = letter.into();
                self.final_word[index.expect("No index")] = l.clone();
                self.present_letters.push(l.clone());
                if self.absent_letters.contains_key(&l) {
                    let mut indicies = self
                        .absent_letters
                        .get(&l)
                        .expect("No vec of indicies")
                        .to_owned();
                    if let Some(i) = index {
                        if indicies.len() > i {
                            indicies.remove(i);
                            self.absent_letters.insert(l, indicies);
                        }
                    }
                }

                true
            }
        }
    }
    pub fn filter_words(&mut self) {
        self.words = self
            .words
            .iter()
            .map(|word| word.clone())
            .filter(|word| {
                //check for letters in the correct positions;
                let filter_correct = word.char_indices().all(|(i, _)| {
                    self.final_word[i] == String::from(word.chars().nth(i).unwrap())
                        || self.final_word[i] == ""
                });
                filter_correct
            })
            .filter(|word| {
                //remove the know absent_letters letters
                let filter_absent = !word.chars().any(|c| {
                    self.absent_letters.contains_key(&String::from(c))
                //check if the the word has the letter in the indices where it is known the letter cannot be there
                        && word.char_indices().any(|(i, c)| {
                            match self.absent_letters.get(&String::from(c)) {
                                Some(result) => result.contains(&i),
                                None => false,
                            }
                        })
                });
                filter_absent
            })
            .filter(|word| {
                //check if the word contains the letter in the known letter set
                let filter_known = self
                    .present_letters
                    .iter()
                    .all(|letter| word.contains(letter));

                filter_known
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

        // Wordle words freq analysis
        // map.insert("a".to_string(), 8.45);
        // map.insert("b".to_string(), 2.43);
        // map.insert("c".to_string(), 4.11);
        // map.insert("d".to_string(), 3.40);
        // map.insert("e".to_string(), 10.65);
        // map.insert("f".to_string(), 1.98);
        // map.insert("g".to_string(), 2.69);
        // map.insert("h".to_string(), 3.35);
        // map.insert("i".to_string(), 5.80);
        // map.insert("j".to_string(), 0.23);
        // map.insert("k".to_string(), 1.82);
        // map.insert("l".to_string(), 6.20);
        // map.insert("m".to_string(), 2.74);
        // map.insert("n".to_string(), 4.96);
        // map.insert("o".to_string(), 6.52);
        // map.insert("p".to_string(), 3.16);
        // map.insert("q".to_string(), 0.25);
        // map.insert("r".to_string(), 7.77);
        // map.insert("s".to_string(), 5.79);
        // map.insert("t".to_string(), 6.31);
        // map.insert("u".to_string(), 4.04);
        // map.insert("v".to_string(), 1.32);
        // map.insert("w".to_string(), 1.68);
        // map.insert("x".to_string(), 0.32);
        // map.insert("y".to_string(), 3.67);
        // map.insert("z".to_string(), 0.35);

        return map;
    }

    pub fn find_optimal_word(&self) -> String {
        let (mut score, mut index) = (0.0, 0);
        let mut dup_set = HashSet::<char>::new();
        for (i, word) in self.words.iter().enumerate() {
            let mut current_score = 0.0;
            word.chars().for_each(|c| {
                let c_score = self
                    .letter_freq_table
                    .get(&String::from(c))
                    .expect("No value");

                if dup_set.insert(c) {
                    current_score += c_score;
                } else {
                    current_score += c_score / 2.0; //reduce score for duplicate letters
                }
            });
            if current_score >= score {
                score = current_score;
                index = i;
            }
        }
        assert!(!self.words.is_empty(), "Word list is empty");
        return self.words[index].clone();
    }
}

#[derive(Debug)]
pub struct Letter {
    letter: String,
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
        Letter { letter: value }
    }
}
