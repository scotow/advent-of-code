pub struct Range<F>
    where F: Fn(&Vec<u8>) -> bool {
    current: Vec<u8>,
    end: Vec<u8>,
    validator: F
}

impl<F> Range<F>
    where F: Fn(&Vec<u8>) -> bool {

    pub fn new(start: u32, end: u32, validator: F) -> Range<F> {
        Range {
            current: to_digits(start),
            end: to_digits(end),
            validator: validator
        }
    }
}

impl<F> Iterator for Range<F> 
    where F: Fn(&Vec<u8>) -> bool {

    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            increment(&mut self.current);
            if self.current > self.end {
                return None
            }

            if (self.validator)(&self.current) {
                return Some(())
            }
        }
    }
}

fn to_digits(number: u32) -> Vec<u8> {
    number.to_string().chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect()
}

pub fn is_valid_serie(n: &Vec<u8>) -> bool {
    let mut has_double = false;
    for i in 1..n.len() {
        if n[i - 1] > n[i] { return false } 
        if n[i - 1] == n[i] { has_double = true; }
    }
    has_double
}

pub fn is_valid_double(n: &Vec<u8>) -> bool {
    let last = n.len() - 1;
    let mut has_double = false;
    for i in 1..n.len() {
        if n[i - 1] > n[i] { return false } 
        if n[i - 1] == n[i] {
            if i == 1 {
                if n[i + 1] != n[i] {
                    has_double = true;
                }
            } else {
                if i == last {
                    if n[i - 2] != n[i] {
                        has_double = true;
                    }
                } else {
                    if n[i - 2] != n[i] && n[i + 1] != n[i] {
                        has_double = true;
                    }
                }
            }
        }
    }
    has_double
}

fn increment(n: &mut Vec<u8>) {
    let last = n.len() - 1;
    n[last] += 1;
    for i in (1..n.len()).rev() {
        if n[i] == 10 {
            n[i] = 0;
            n[i - 1] += 1;
        }
    }
}

#[test]
fn is_valid_serie_test() {
    assert!(is_valid_serie(&vec!(1, 2, 2, 3, 4, 5)));
    assert!(is_valid_serie(&vec!(1, 1, 1, 1, 2, 3)));
    assert!(is_valid_serie(&vec!(1, 1, 1, 1, 1, 1)));

    assert!(!is_valid_serie(&vec!(2, 2, 3, 4, 5, 0)));
    assert!(!is_valid_serie(&vec!(1, 2, 3, 7, 8, 9)));
}

#[test]
fn is_valid_double_test() {
    assert!(is_valid_double(&vec!(1, 2, 2, 3, 4, 5)));
    assert!(is_valid_double(&vec!(1, 1, 2, 2, 3, 3)));
    assert!(is_valid_double(&vec!(1, 1, 1, 1, 2, 2)));

    assert!(!is_valid_double(&vec!(2, 2, 3, 4, 5, 0)));
    assert!(!is_valid_double(&vec!(1, 2, 3, 7, 8, 9)));
    assert!(!is_valid_double(&vec!(1, 1, 1, 1, 2, 3)));
    assert!(!is_valid_double(&vec!(1, 1, 1, 1, 1, 1)));
    assert!(!is_valid_double(&vec!(1, 2, 3, 4, 4, 4)));
}

#[test]
fn increment_test() {
    let mut vec = vec!(1, 2, 2);
    increment(&mut vec);
    assert_eq!(vec, vec![1, 2, 3]);

    vec = vec!(1, 2, 9);
    increment(&mut vec);
    assert_eq!(vec, vec![1, 3, 0]);

    vec = vec!(9, 9, 9);
    increment(&mut vec);
    assert_eq!(vec, vec![10, 0, 0]);
}