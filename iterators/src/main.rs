fn main() {
    simple_iterator();
    map_collect_iterator();
}

fn simple_iterator() {
    let v1 = vec![5, 10, 13];

    let mut iter = v1.iter();

    assert_eq!(iter.next(), Some(&5));
    assert_eq!(iter.next(), Some(&10));
    assert_eq!(iter.next(), Some(&13));
    assert_eq!(iter.next(), None);

    let total: i32 = iter.sum();
    assert_eq!(total, 0);

    let iter = v1.iter();
    let total: i32 = iter.sum();
    assert_eq!(total, 28);

    // assert_eq!(iter.next(), None); // fails
}

fn map_collect_iterator() {
    let v1 = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == size). collect()
}

#[test]
fn filters_by_size() {
    let shoes = vec![
        Shoe { size: 10, style: String::from("foo") },
        Shoe { size: 13, style: String::from("bar") },
        Shoe { size: 10, style: String::from("zab") },
    ];

    let mut my_size = shoes_in_size(shoes, 10);
    assert_eq!(my_size,
               vec![
                    Shoe { size: 10, style: String::from("foo") },
                    Shoe { size: 10, style: String::from("zab") },
                   ]);

    my_size[0].size = 9;
    let my_size = shoes_in_size(my_size, 10);
    assert_eq!(my_size,
               vec![
                    Shoe { size: 10, style: String::from("zab") },
                   ]);
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0, }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

#[test]
fn test_counter() {
    let mut cnt = Counter::new();

    assert_eq!(cnt.next(), Some(1));
    assert_eq!(cnt.next(), Some(2));
    assert_eq!(cnt.next(), Some(3));
    assert_eq!(cnt.next(), Some(4));
    assert_eq!(cnt.next(), Some(5));
    assert_eq!(cnt.next(), None);
}
