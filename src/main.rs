use hello_rust::module_a::Foo;
use hello_rust::module_b::add;

fn main() {
    let foo = Foo{ a: 1, b: 2};
    let bar = &foo;
    println!("{}{}", bar.a, bar.b);
    let a = add(1, 2);
    println!("{}", a);
}

// // trait
// trait Tweet {
//     fn tweet(&self);

//     fn tweet_twice(&self) {
//         self.tweet();
//         self.tweet();
//     }

//     fn shout(&self) {
//         println!("aaaaaaaaa")
//     }
// }

// struct Dove;
// struct Duck;

// impl Tweet for Dove {
//     fn tweet(&self) {
//         println!("cool");
//     }
// }

// impl Tweet for Duck {
//     fn tweet(&self) {
//         println!("aa");
//     }
// }


// fn main() {
//     let dove = Dove {};
//     dove.tweet();
//     dove.tweet_twice();
//     dove.shout();
//     let duck = Duck {};
//     duck.tweet();
//     duck.tweet_twice();
//     duck.shout();


//     println!("Hello, world!");
// }

// generics
// fn make_tuple<T, S>(t: T, s: S) -> (T, S) {
//     (t, s)
// }

// fn main() {
//     make_tuple(1, 2);
//     make_tuple("foo", "bar");
// }
