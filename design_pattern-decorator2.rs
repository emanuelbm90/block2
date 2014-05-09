#![crate_id="design_pattern-decorator2#1.0"]
#![crate_type = "bin"]
#![license = "MIT"]
#![desc = "Example of design pattern inspired from Head First Design Patterns"]
//! Example of design pattern inspired from Head First Design Patterns
//!
//! Tested with rust-0.11-pre
//!
//! @author Eliovir <http://github.com/~eliovir>
//!
//! @license MIT license <http://www.opensource.org/licenses/mit-license.php>
//!
//! @since 2014-04-12

// This design pattern needs a common trait for Drinks and Ingredients
trait Drinkable {
	fn description(&self) -> ~str;
	fn price(&self) -> f64;
}

struct Drink{
	price: f64,
	description: ~str
}

impl Drinkable for Drink {
	fn description(&self) -> ~str {
		self.description.to_owned()
	}
	fn price(&self) -> f64 {
		self.price
	}
}

impl Drink {
	// The "constructor", optional but useful!
	pub fn new(description: ~str, price: f64) -> Drink {
		Drink { description: description, price: price }
	}
}

struct Ingredient {
	description: ~str,
	price: f64,
	// to decorate an struct, it must have the common trait
	drink: Box<Drinkable>,
}

impl Drinkable for Ingredient {
	fn description(&self) -> ~str {
		self.drink.description() + ", " + self.description
	}
	fn price(&self) -> f64 {
		self.price + self.drink.price()
	}
}

impl Ingredient {
	// The "constructor", optional but useful!
	pub fn new(description: ~str, price: f64, drink: Box<Drinkable>) -> Ingredient {
		Ingredient { description: description, price: price, drink: drink }
	}
}

fn main() {
	let columbia = Drink::new("Columbia".to_owned(), 0.89);
	println!("{} => {}", columbia.description(), columbia.price());

	let chocolate = Ingredient::new("chocolate".to_owned(), 0.2, box columbia);
	println!("{} => {}", chocolate.description(), chocolate.price());	

	let chantilly = Ingredient::new("chantilly".to_owned(), 0.1, box chocolate);
	println!("{} => {}", chantilly.description(), chantilly.price());	
}