use std::{thread, time::Duration};

struct ComputeCache<T>
where
    T: Fn() -> String,
{
    computation: T,
    value: Option<String>,
}

impl<T> ComputeCache<T>
where
    T: Fn() -> String,
{
    fn new(computation: T) -> Self {
        ComputeCache {
            computation,
            value: None,
        }
    }

    fn get_result(&mut self) -> String {
        match &self.value {
            Some(v) => {
                println!("Retrieved from cache instantly!");
                v.clone()
            }
            None => {
                println!("Computing (this will take 2 seconds)...");
                thread::sleep(Duration::from_secs(2));
                let result = (self.computation)();
                self.value = Some(result.clone());
                result
            }
        }
    }
}

fn track_changes() {
    let mut tracker = 0;
    let mut update = || {
        tracker += 1;
        println!("Tracker value: {}", tracker);
    };

    update();
    update();
}

fn process_vector<F>(vec: Vec<i32>, f: F) -> Vec<i32>
where
    F: Fn(i32) -> i32,
{
    vec.into_iter().map(f).collect()
}

fn process_vector_with_loop<F>(vec: Vec<i32>, f: F) -> Vec<i32>
where
   F: Fn(i32) -> i32,
{
   let mut result = Vec::new();
   for x in vec {
       result.push(f(x));
   }
   result
}

fn main() {
 let operation = |a: i32, b: i32| {
        a * b
    };

    println!("Result: {}", operation(10, 5));

    track_changes();

    let numbers = vec![1, 2, 3];

    //Using process_vector with map and collect
    let doubled = process_vector(numbers.clone(), |x| x * 2);
    let replaced = process_vector(numbers.clone(), |x| if x > 2 { 0 } else { x });

    println!("Doubled (map): {:?}", doubled);
    println!("Replaced (map): {:?}", replaced);

    //Using process_vector_with_loop
    let doubled_loop = process_vector_with_loop(numbers.clone(), |x| x * 2);
    let replaced_loop = process_vector_with_loop(numbers, |x| if x > 2 { 0 } else { x });

    println!("Doubled (loop): {:?}", doubled_loop);
    println!("Replaced (loop): {:?}", replaced_loop);


    let mut cache = ComputeCache::new(|| {
        println!("Performing expensive computation...");
        thread::sleep(Duration::from_secs(2));
        "Hello, world!".to_string()
    });

    println!("First call:");
    println!("Result: {}", cache.get_result());
    
    println!("\nSecond call:");
    println!("Result (cached): {}", cache.get_result());
}
