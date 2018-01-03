use std::collections::HashMap;
use std::hash::Hash;

fn main() {
    let mut cacher = Cacher::new(some_fn);
    println!("{:?}", cacher.call(5));
    println!("{:?}", cacher.call(5));
}

struct Cacher<I, O, F>
    where F: Fn(I) -> O
{
    function: F,
    cache: HashMap<I, O>,
}

impl<I, O, F> Cacher<I, O, F>
    where I: Eq + Hash + Copy,
          O: Copy,
          F: Fn(I) -> O
{
    fn new(function: F) -> Self{
        let cache = HashMap::new();
        Self { // Here the order doesn't matter, if types are different.
               // Not naming the members is just syntactic sugar
            function,
            cache,
       }
    }
    fn call(&mut self, key: I) -> O {
        // Here it would be possible to use stream-like methods
        // .or_insert_with
        // or match against cache.entry: Entry::Occupied, Entry::Vacant
        if !self.cache.contains_key(&key) {
            let val = (self.function)(key);
            self.cache.insert(key, val);
            return val;
        }
        // It looks like the return value always needs to be cloned?
        self.cache.get(&key).unwrap().clone()
    }
}

fn some_fn(i: i32) -> i32 {
    println!("Calling the function here...");
    i * 2
}
