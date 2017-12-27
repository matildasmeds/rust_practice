use std::collections::HashMap;

// To make Cacher generic, maybe I need to Box the Fn() ?
// `std::ops::Fn() + 'a` does not have a constant size known at compile-time
// Or something else, don't yet know what


fn main() {
    let mut cacher = Cacher {
        function: &some_fn,
        cache: HashMap::new(),
    };
    println!("{:?}", cacher.call(5));
}

struct Cacher<'a> {
    function: &'a (Fn(i32) -> i32),
    cache: HashMap<i32, i32>,
}

impl<'a> Cacher<'a> {
    fn call(&mut self, key: i32) -> i32 {
        if !self.cache.contains_key(&key) {
            let val = (self.function)(key);
            self.cache.insert(key, val);
        }
        self.cache.get(&key).unwrap().clone()
    }
}

fn some_fn(i: i32) -> i32 {
    return i * 2;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
