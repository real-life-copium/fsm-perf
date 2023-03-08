// vim: noet

pub mod common {
	pub fn valid_time(string: &str) -> bool {
		let parts: Vec<&str> = string.split(':').collect();
		if parts.len() != 2
		|| parts[0].chars().all(char::is_numeric) != true
		|| parts[1].chars().all(char::is_numeric) != true {
			return false;
		}


		match parts[0].parse::<u8>() {
			Ok(value) => {
				if value >= 24 {
					return false;
				}
			},
			Err(_e) => {
				return false;
			}
		};

		match parts[1].parse::<u8>() {
			Ok(value) => value < 60,
			Err(_e) => false
		}
	}
}

pub mod fsm {
	use std::mem::transmute;

	#[repr(u8)]
	pub enum Action {
		Digit0 = 0,
		Digit1,
		Digit2,
		Digit3,
		Digit4,
		Digit5,
		Digit6,
		Digit7,
		Digit8,
		Digit9,
		Colon,
		Others,
		End,
	}

	impl From<char> for Action {
		fn from(value: char) -> Self {
			match value {
				':' => Action::Colon,
				'0'..='9' => { 
					let value = value as u8 - '0' as u8;
					unsafe { transmute(value) }
				},
				'\0' => Action::End,
				_ => Action::Others,
			}
		}
	}

	#[repr(u8)]
	#[derive(PartialEq, Clone, Copy)]
	enum State {
		Initial = 0,
		Hour1,
		Hour2,
		HourEnd,
		Delim,
		Minute,
		MinuteEnd,
		Match,
		Error,
	}

	use State::*;
	static STATES: [[State; 13]; 8]= [
		[ Hour1, Hour1, Hour2, HourEnd, HourEnd, HourEnd, HourEnd, HourEnd, HourEnd, HourEnd, Error, Error, Error, ],
		[ HourEnd, HourEnd, HourEnd, HourEnd, HourEnd, HourEnd, HourEnd, HourEnd, HourEnd, HourEnd, Delim, Error, Error, ],
		[ HourEnd, HourEnd, HourEnd, HourEnd, Error, Error, Error, Error, Error, Error, Delim, Error, Error, ],
		[ Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Delim, Error, Error, ],
		[ Minute, Minute, Minute, Minute, Minute, Minute, MinuteEnd, MinuteEnd, MinuteEnd, MinuteEnd, Error, Error, Error, ],
		[ MinuteEnd, MinuteEnd, MinuteEnd, MinuteEnd, MinuteEnd, MinuteEnd, MinuteEnd, MinuteEnd, MinuteEnd, MinuteEnd, Error, Error, Match, ],
		[ Hour1, Hour1, Hour2, Error, Error, Error, Error, Error, Error, Error, Error, Error, Match, ],
		[ Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, Error, ],
	];

	pub fn valid_time(string: &str) -> bool {
		let mut state = Initial;
		for c in string.chars() {
			let action: Action = c.into();
			state = STATES[state as usize][action as usize];
			if state == Error {
				return false;
			}
		}
		state = STATES[state as usize][Action::End as usize];
		Match == state
	}
}

#[cfg(test)]
mod tests {
	use std::sync::Once;
	use rand::random;

	use crate::{common, fsm};

	static INIT: Once = Once::new();
	static mut DATA: Vec<String> = Vec::new();

	fn initialize() {
		let count = 1_000_000;
		for _ in 0..count {
			let mut s = String::new();

			let r = random::<u8>();
			match r % 16 {
				0 => (),
				1..=2 => s.push('0'),
				3..=7 => s.push('1'),
				8..=11 => s.push('2'),
				12..=13 => { 
					let c = '0' as u8 + random::<u8>() % 10;
					s.push(c as char);
				},
				14 => s.push(':'),
				_ => s.push(random()),
			};

			let r = random::<u8>();
			match r % 16 {
				0..=13 => { 
					let c = '0' as u8 + random::<u8>() % 10;
					s.push(c as char);
				},
				14 => s.push(':'),
				_ => s.push(random()),
			};

			let r = random::<u8>();
			if r < 192 {
				s.push(':');
			} else if r < 216 {
				s.push(random());
			}

			let r = random::<u8>();
			match r % 16 {
				0 => (),
				1..=6 => {
					let c = '0' as u8 + random::<u8>() % 6;
					s.push(c as char);
				},
				7..=13 => { 
					let c = '0' as u8 + random::<u8>() % 10;
					s.push(c as char);
				},
				_ => s.push(random()),
			};

			let r = random::<u8>();
			match r % 16 {
				0 => (),
				1..=13 => {
					let c = '0' as u8 + random::<u8>() % 10;
					s.push(c as char);
				},
				_ => s.push(random()),
			};

			let r = random::<u8>();
			if r > 216 {
				s.push(random());
			}

			unsafe {
				DATA.push(s);
			}
		}
	}

	#[test]
	fn it_works() {
		INIT.call_once(initialize);
		unsafe {
			for s in &DATA {
				let s = s.as_str();
				let cr = common::valid_time(s);
				let fr = fsm::valid_time(s);
				assert_eq!(fr, cr,
					"string: \"{}\"\nfsm: {}\ncommon: {}", s, fr, cr);
			}
		}
	}

	fn print_elapsed(elapsed: u128) {
		if elapsed < 1_000 {
			println!("The test took {}ns to finish.", elapsed);
		} else if elapsed < 1_000_000 {
			let elapsed = elapsed as f64 / 1e3;
			println!("The test took {}us to finish.", elapsed);
		} else {
			let elapsed = elapsed as f64 / 1e6;
			println!("The test took {}ms to finish.", elapsed);
		}
	}

	#[test]
	fn common() {
		INIT.call_once(initialize);
		let mut elapsed = 0;
		unsafe {
			for s in &DATA {
				let s = s.as_str();
				let inst = std::time::Instant::now();
				common::valid_time(s);
				elapsed += inst.elapsed().as_nanos();
			}
		}
		print_elapsed(elapsed);
	}

	#[test]
	fn fsm() {
		INIT.call_once(initialize);
		let mut elapsed = 0;
		unsafe {
			for s in &DATA {
				let s = s.as_str();
				let inst = std::time::Instant::now();
				fsm::valid_time(s);
				elapsed += inst.elapsed().as_nanos();
			}
		}
		print_elapsed(elapsed);
	}
}
