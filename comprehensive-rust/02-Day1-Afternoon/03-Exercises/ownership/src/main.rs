pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

pub trait IntoIterator {
    type Item;
    type IntoIter: Iterator<Item = Self::Item>;

    fn into_iter(self) -> Self::IntoIter;
}

fn main() {
    let v: Vec<i8> = vec![10, 20, 30];
    let mut iter = v.iter();

    println!("v[0]: {:?}", iter.next());
    println!("v[1]: {:?}", iter.next());
    println!("v[2]: {:?}", iter.next());
    println!("No more items: {:?}", iter.next());

    let v0: Option<&i8> = iter.next();
    println!("v0: {v0:?}");


    let w: Vec<String> = vec![String::from("foo"), String::from("bar")];
    let mut iter = w.into_iter();

    let w0: Option<String> = iter.next();
    println!("w0: {w0:?}");

    let x: Vec<String> = vec![String::from("foo"), String::from("bar")];

    for word in &x {
        println!("word: {word}");
    }

    for word in x {
        println!("word: {word}");
    }
}
