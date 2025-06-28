use std::ops::Deref;
use std::rc::Rc;

use std::cell::RefCell;
use crate::Worker;

pub trait Logger {
    fn warning(&self, msg: &str);
    fn info(&self, msg: &str);
    fn error(&self, msg: &str);
}

pub struct Tracker<'a> {
    pub logger: &'a dyn Logger,
    pub value: &'a Rc<i32>, //ref
    pub max: i32,
}

impl<'a> Tracker<'a> {
    pub fn new(worker: &'a Worker, max: i32) -> Self {
        Tracker {
            logger: worker,
            value: &worker.track_value,
            max: max,
        }
    }
    pub fn set_value(&self, i: &Rc<i32>) {
        let count = Rc::strong_count(i);
        let percentage = (count as f64 / self.max as f64) * 100.0;
        let percent_int = percentage as u32;
        if percentage >= 100.0 {
            self.logger.error("you are over your quota!");
        } else if percentage >= 70.0 {
            self.logger.warning(&format!(
                "you have used up over {}% of your quota! Proceeds with precaution",
                percent_int
            ));
        }
    }

    pub fn peek(&self, i: &Rc<i32>) {
        let count = Rc::strong_count(i);
        let percentage = (count as f64 / self.max as f64) * 100.0;
        let percent_int = percentage as u32;
        self.logger
            .info(&format!("you are using up to {}% of your quota", percent_int));
    }
}
