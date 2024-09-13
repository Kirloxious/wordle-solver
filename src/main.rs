use std::collections::HashSet;

use enigo::{Direction::Click, Enigo, InputError, Key, Keyboard, Settings};
use thirtyfour::prelude::*;
use wordle_solver::*;

pub mod wordle_solver;

/*
    Things to do:
        - Check for edge cases (double letter, prevent guessing same letter twice in a word)




*/
static BEST_COMMON_STARTING_WORDS: [&str; 5] = ["adieu", "audio", "trace", "arise", "crane"];

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:9515", caps).await?;
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    let mut solver = WordleSolver::new();

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
    tokio::time::sleep(std::time::Duration::from_millis(1000)).await; //wait for pop up to close

    //Get the board
    let board_elem = driver
        .find(By::ClassName("Board-module_board__jeoPS"))
        .await?;
    let rows = board_elem
        .find_all(By::ClassName("Row-module_row__pwpBq"))
        .await?;

    // let mut guess = BEST_COMMON_STARTING_WORDS[3].to_owned();
    let mut guess = "apple".to_string();

    for i in 0..6 {
        type_word(guess, &mut enigo).unwrap();
        tokio::time::sleep(std::time::Duration::from_millis(2000)).await; //wait for animation to finish

        let squares = rows[i]
            .find_all(By::ClassName("Tile-module_tile__UWEHN"))
            .await?;

        for (i, s) in squares.iter().enumerate() {
            let state = s.attr("data-state").await?.expect("No state");
            let letter = s.inner_html().await?;
            solver.insert_letter(letter, state, Some(i));
        }

        solver.filter_words();
        println!("{:?}", solver.final_word);
        println!("{:?}", solver.present_letters);
        println!("{:?}", solver.absent_letters);
        println!("{:?}", solver.words);
        guess = solver.find_optimal_word();
    }

    // Always explicitly close the browser.
    driver.quit().await?;

    Ok(())
}

fn type_word(word: String, enigo: &mut Enigo) -> Result<(), InputError> {
    for letter in word.chars() {
        println!("{letter}");
        enigo.key(Key::Unicode(letter), Click)?
    }
    enigo.key(Key::Return, Click).unwrap();
    Ok(())
}
