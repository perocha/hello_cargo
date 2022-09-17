#[derive(Debug)]
struct Counter {
    length: usize,
    count: usize,
}

// New method, it will control how the iterator will be initiated
impl Counter {
    fn new (length: usize) -> Counter {
        Counter {
            count: 0,
            length,
        }
    }
}

// Iterator method
impl Iterator for Counter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count <= self.length {
            Some (self.count)
        } else {
            None
        }
    }
}


pub fn iterator_example() {
    // Create a counter of 6 and iterate using next method
    let mut my_counter = Counter::new(6);
    println!("iterator_example::New counter created {:#?}", my_counter);
    assert_eq!(my_counter.next(), Some(1));
    assert_eq!(my_counter.next(), Some(2));
    assert_eq!(my_counter.next(), Some(3));
    assert_eq!(my_counter.next(), Some(4));
    assert_eq!(my_counter.next(), Some(5));
    assert_eq!(my_counter.next(), Some(6));
    assert_eq!(my_counter.next(), None);    // Any further call will always return 'None'
    assert_eq!(my_counter.next(), None);
    assert_eq!(my_counter.next(), None);
    println!("iterator_example::Counter state {:#?}", my_counter);

    // Iterate using for
    for number in Counter::new(10) {
        println! ("{}", number);
    }

    // Use other methods available by default in Iterator trait
    let my_sum: usize = Counter::new(10).sum();
    println!("iterator_example::my_sum = {}", my_sum);
    println!("iterator_example::count = {}", Counter::new(5).count());
    println!("iterator_example::length = {}", Counter::new(5).length);
}