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

// Container type
struct Container<T> {
    value: T,
}

impl<T> Container<T> {
    pub fn new(value: T) -> Self {
        Container { value }
    }
}

// Groups type
struct Groups<T> {
    inner: Vec<T>,
}

impl<T> Groups<T> {
    fn new(inner: Vec<T>) -> Self {
	    Groups { inner }
    }
}

impl<T: PartialEq> Iterator for Groups<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        // if the inner vector is empty, we are done
        if self.inner.is_empty() {
            return None;
        }

        // lets check the span of equal items
        let mut cursor = 1;
        let first = &self.inner[0];
        for element in &self.inner[1..] {
            if element == first {
                cursor += 1;
            } else {
                break;
            }
        }

        // we use the `Vec::drain` to extract items up until the cursor
        let items = self.inner.drain(0..cursor).collect();

        // return the extracted items
        Some(items)
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

    // Test Container type
    println!("{}", Container::new(42).value);
    println!("{}", Container::new(3.14).value);
    println!("{}", Container::new("Foo").value);
    println!("{}", Container::new(String::from("Bar")).value);
    println!("{}", Container::new(true).value);
    println!("{}", Container::new(-12).value);
    println!("{:?}", Container::new(Some("text")).value);

    // Exercise example ---> groups the vector contents with similar numbers
    let data1 = vec![4, 1, 1, 2, 1, 3, 3, -2, -2, -2, 5, 5];
    //           groups:     |->|---->|->|->|--->|----------->|--->|
    println!("data1::{:?}", data1);
    println!("{:?}",Groups::new(data1).into_iter().collect::<Vec<Vec<_>>>());

    let data2 = vec![1, 2, 2, 1, 1, 2, 2, 3, 4, 4, 3];
    //          groups:      |->|---->|---->|----|->|----->|->|
    println!("data2::{:?}", data2);
    println!("{:?}",Groups::new(data2).into_iter().collect::<Vec<Vec<_>>>());
}