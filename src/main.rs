// trait
trait Tweet {
    fn tweet(&self);

    fn tweet_twice(&self) {
        self.tweet();
        self.tweet();
    }

    fn shout(&self) {
        println!("aaaaaaaaa")
    }
}

struct Dove;
struct Duck;

impl Tweet for Dove {
    fn tweet(&self) {
        println!("cool");
    }
}

impl Tweet for Duck {
    fn tweet(&self) {
        println!("aa");
    }
}


fn main() {
    let dove = Dove {};
    dove.tweet();
    dove.tweet_twice();
    dove.shout();
    let duck = Duck {};
    duck.tweet();
    duck.tweet_twice();
    duck.shout();


    println!("Hello, world!");
}