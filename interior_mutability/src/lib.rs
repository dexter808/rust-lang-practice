pub trait Messenger {
    fn send(&self, message: &str);
}

struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl <'a, T> LimitTracker<'a, T>
where T: Messenger,
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker { messenger, value: 0, max }
    }    

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percent_max = value as f64 / self.max as f64;
        if percent_max >= 1.0 {
            self.messenger.send("Error: Value has exceeded 100% of max");
        } else if percent_max >= 0.9 {
            self.messenger.send("Urgent Warning: Value is above 90% of max");
        } else if percent_max >= 0.75 {
            self.messenger.send("Warning: Value is above 75% of max");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct mock_messenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl mock_messenger {
        fn new() -> mock_messenger {
            mock_messenger { sent_messages: RefCell::new(vec![])}
        }
    }

    impl Messenger for mock_messenger {
        fn send(&self, message: &str) {
            let mut first_borrow = self.sent_messages.borrow_mut();
            self.sent_messages.borrow_mut().push(String::from(message));
            first_borrow.push(String::from(message));
        }
    }

    #[test]
    fn warning_messege_test_75() {
        let mock_messenger = mock_messenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(76);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 2);
    }
}