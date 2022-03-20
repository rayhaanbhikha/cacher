use std::hash::Hash;
use std::{collections::HashMap, thread::sleep, time};
struct Cacher<T, K, L>
where
    T: Fn(K) -> L,
    K: Eq + Hash + Clone,
{
    calculation: T,
    map: HashMap<K, L>,
}

impl<T, K, L> Cacher<T, K, L>
where
    T: Fn(K) -> L,
    K: Eq + Hash + Clone,
{
    fn new(calculation: T) -> Self {
        Cacher {
            calculation,
            map: HashMap::new(),
        }
    }

    fn value(&mut self, arg: K) -> &L {
        let arg_clone = arg.clone();
        self.map
            .entry(arg)
            .or_insert_with(|| (self.calculation)(arg_clone))
    }
}

fn main() {
    let mut num_cacher = Cacher::new(|x| {
        println!("Expensive func running");
        sleep(time::Duration::from_secs(2));
        x
    });

    let mut string_cacher = Cacher::new(|mut message: String| {
        println!("Expensive func running");
        sleep(time::Duration::from_secs(2));
        message.push_str(" yo world");
        message
    });

    println!("{}", num_cacher.value(3));
    println!("{}", num_cacher.value(4));
    println!("{}", num_cacher.value(3));

    println!("{}", string_cacher.value(String::from("hello")));
    println!("{}", string_cacher.value(String::from("boom")));
    println!("{}", string_cacher.value(String::from("hello")));
}
