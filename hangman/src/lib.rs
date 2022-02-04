use wasm_bindgen::prelude::*;
use rand::prelude::SliceRandom;

// Import the `window.alert` function from the Web.
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

// Export a `greet` function from Rust to JavaScript, that alerts a
// hello message.
#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn test(name: String) -> bool {
	return name == String::from("LEO");
}

#[wasm_bindgen]
pub struct Hangman {
    word_to_guess: String,
    guessed_letters: Vec<char>,
    wrong_letters: Vec<char>,
	partial_guess: String,
    lives: i32,
	game_status: UserStatus,
}

#[wasm_bindgen]
pub enum UserStatus {
    StillAlive,
    Dead,
    Win,
}

#[wasm_bindgen]
impl Hangman {
	#[wasm_bindgen(constructor)]
    pub fn new() -> Hangman {
		let word_list: Vec<String> = vec![
			String::from("traditional"),
			String::from("institution"),
			String::from("property"),
			String::from("collection"),
			String::from("partner"),
			String::from("positive"),
			String::from("responsibility"),
			String::from("interesting"),
			String::from("account"),
			String::from("attorney"),
			String::from("citizen"),
			String::from("majority"),
			String::from("neighborhood"),
			String::from("immediately"),
			String::from("absolutely"),
			String::from("freedom"),
			String::from("conversation"),
			String::from("university"),
			String::from("willing"),
			String::from("apartment"),
		];
		let random_word : String = String::from(word_list.choose(&mut rand::thread_rng()).unwrap());
		let hidden_word : String = (0..random_word.len()).map(|_| "❔").collect::<String>();
        return Hangman {
            word_to_guess: random_word,
            guessed_letters: Vec::new(),
            wrong_letters: Vec::new(),
            lives: 7,
			partial_guess: hidden_word,
			game_status: UserStatus::StillAlive,
        };
    }

    pub fn validate_user_guess(&mut self, guess: char) -> String {
		self.validate_user_status();
		match self.game_status {
			UserStatus::Dead => String::from("You are dead, you can't play anymore!"),
			UserStatus::Win => String::from("You won!"),
			UserStatus::StillAlive => {
				if self.wrong_letters.contains(&guess) || self.guessed_letters.contains(&guess) {
					return String::from("You have already guessed this letter!");
				} else if self.word_to_guess.contains(guess) {
					self.guessed_letters.push(guess);
					let mut partial_guess_update : String = String::new();
					for c in self.word_to_guess.chars() {
						if self.guessed_letters.contains(&c) {
							partial_guess_update.push_str(&String::from(c));
						} else {
							partial_guess_update.push_str(&String::from("❔"));
						}
					}
					self.partial_guess = partial_guess_update;
					if self.partial_guess == self.word_to_guess {
						self.game_status = UserStatus::Win;
						return String::from("You won! 🎉");
					}
					return String::from("Well played! This letter is in the word");
				} else {
					self.lives -= 1;
					self.wrong_letters.push(guess);
					if self.lives <= 0 {
						return String::from("You are dead! 💀 ");
					} else {
						return String::from("Missed! This letter is not in the word, you lose a live!");
					}
				}
			}
		}
    }

    pub fn validate_user_status(&mut self) {
		if self.lives <= 0 {
			self.game_status = UserStatus::Dead;
		} else {
			let mut is_win : bool = true;
			for c in self.get_word().chars() {
				if !self.guessed_letters.contains(&c) {
					is_win = false;
				}
			}
			if is_win {
				self.game_status = UserStatus::Win;
			} else {
				self.game_status = UserStatus::StillAlive;
			}
		}
    }

	pub fn get_word(&self) -> String {
		return self.word_to_guess.clone();
	}

	pub fn get_lives(&self) -> i32 {
		return self.lives;
	}

	pub fn is_alive(&self) -> bool {
		match self.game_status {
			UserStatus::StillAlive => true,
			UserStatus::Dead => false,
			UserStatus::Win => false,
		}
	}

	pub fn is_already_guessed(&self, c: char) -> bool {
		return self.wrong_letters.contains(&c);
	}

	pub fn get_guess_status(&self) -> String {
		return self.partial_guess.clone();
	}
}
