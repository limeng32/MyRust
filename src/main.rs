fn a_a2(x: char, y: char) {
	let xx = "hello";
	let yy = xx;
	println!("{}, world!", xx);
	let s = ['R', 'U', 'N', 'O', 'O', 'B'];
	let mut i = 0;
	loop {
		let ch = s[i];
		if ch == 'O' {
			break;
		}
		println!("\'{}\'", ch);
		i += 1;
	}
	println!("{0},{1}", x, y);
}
fn five() -> i64 {
	return 5 + 1;
}
#[derive(Debug)]
struct Rectangle {
	width: u32,
	height: u32,
}
struct UnitStruct1;
impl Rectangle {
	fn area(&self) -> u32 {
		self.width * self.height
	}
}

/// #asd
/// qwe
fn main() {
	fn a_a3(x: char, y: &str, z: i32) {
		let number = if z > 0 { 1 } else { -1 };
		if number > 0 {
			println!("{0};{1};{2}", x, y, z);
		}
	}
	//	const aa: i32 = 1_1__23;
	let c: char = ' ';
	let mut bb = 11__231;
	bb += 2 + five();
	//	const aa:i32 = 123;
	let mut a = "{}";
	println!("Hello, {0} {{{1}}}", a, bb);
	a = "{{0}}";
	println!("Hello, {0} {{{0}}}", a);
	println!("Hello, world!!!!2");
	println!("Hello, world!!!!4");

	let s: (&str, String) = ("aa", "asd".to_string());
	println!("Hello, {0}, {1}", s.0, s.1);
	let (x, y): (&str, String) = ("aa", "asd".to_string());
	println!("{0}", y);
	let s = 0;
	let b = "bb".to_string();
	a_a3('v', &b, 30);
	//	println!("{}", b);

	let rect1 = Rectangle::create(30, 50);

	println!("rect1 is {:?}", rect1);
	println!("rect1 is {:#?}", rect1);
	println!("rect1's area is {}", rect1.area());
	let book = Book::Papery {
		index: 1001,
		url: "url://...2".to_string(),
	};
	let b1 = Book::Papery {
		index: 1001,
		url: "url://...2".to_string(),
	};
	let ebook = Book::Electronic {
		url: "url://...".to_string(),
	};
	let opt = Option::<Book>::Some(b1);
	println!("1{:?}", opt);
	let opt = Some(book);
	println!("1{:?}", opt);
	let opt = Option::<Book>::None;
	println!("2{:?}", opt);
	let opt: Option<&str> = Option::None;
	println!("{:?}", opt);

	let t = Some(64);
	
	println!("{}", (PI / 2.0).sin());
}

impl Rectangle {
	fn create(width: u32, height: u32) -> Rectangle {
		Rectangle { width, height }
	}
}

fn calculate_length(s: &String) -> usize {
	s.len()
}
fn main2() {
	let mut s1 = String::from("run");
	// s1 是可变的

	let s2 = &mut s1;
	// s2 是可变的引用

	s2.push_str("oob");
	println!("{}", s2);
}
#[derive(Debug)]
enum Book {
	Papery { index: u32, url: String },
	Electronic { url: String },
}
fn main3() {
	let t = "abc";
	match t {
		"abc" => println!("Yes"),
		_ => {}
	}
}
#[derive(Debug)]
enum Option<T> {
	Some(T),
	None,
}
use std::f64::consts::PI;