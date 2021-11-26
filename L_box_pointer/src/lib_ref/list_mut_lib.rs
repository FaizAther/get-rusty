// interior mutability

trait Messenger {
    fn send(&self, msg: &str);
}

struct LimitTracker<'a, T>
where
    T:  Messenger
{
    messenger:  &'a T,
    value:      usize,
    max:        usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T:  Messenger
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        return LimitTracker {
            messenger,
            value:  0,
            max,
        };
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let max_per = self.value as f64 / self.max as f64;

        if max_per >= 1.0 {
            self.messenger.send("Error: Limit over!");
        } else if max_per >= 0.9 {
            self.messenger.send("Warning: Limit about to reach!");
        } else {
            self.messenger.send("Info: Limit fine.");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_msgs:  RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            return MockMessenger {
                sent_msgs: RefCell::new(vec![]),
            };
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_msgs.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn over_75() {
        let mock_msgr = MockMessenger::new();
        let mut lt = LimitTracker::new(&mock_msgr, 100);
        lt.set_value(80);
        assert_eq!(mock_msgr.sent_msgs.into_inner().len(), 1);
    }
}

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List<T> {
    Cons(Rc<RefCell<T>>, Rc<List<T>>),
    Nil,
}

fn rc_runner_1() {
    println!("rc_runner_1");

    let v = Rc::new(RefCell::new(5));
    let lv = Rc::new(List::Cons(Rc::clone(&v), Rc::new(List::Nil)));
    
    let l1 = List::Cons(Rc::new(RefCell::new(9)), Rc::clone(&lv));
    let l2 = List::Cons(Rc::new(RefCell::new(40)), Rc::clone(&lv));

    println!("\t{:?}\n\t{:?}", l1, l2);
    println!("\t{:?}", lv);
    *(v.borrow_mut()) += 500;
    println!("\t{:?}", lv);
    println!("\t{:?}\n\t{:?}", l1, l2);
}

pub fn list_mut_lib_runner() {
//  let a = 6;
//  let b = &mut a;
    rc_runner_1();
} 
