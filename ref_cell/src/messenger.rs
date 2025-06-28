use std::rc::Rc;


pub trait Logger {
     fn warning(&self, msg: &str);
    fn info(&self, msg: &str);
    fn error(&self, msg: &str);
}

pub struct Tracker<'a> {
    pub logger: &'a dyn Logger,
    pub value: Rc<usize>, //ref
    pub max: i32,
}

impl<'a> Tracker<'a> {
    pub fn new(worker: &'a dyn Logger, max: i32) -> Self {
        Tracker {
            logger: worker,
            value: Rc::new(0),
            max: max,
        }
    }
    pub fn set_value<T>(&self, i: &Rc<T>) {
        let count = Rc::strong_count(i);
        let percentage = (count as f64 / self.max as f64) * 100.0;
        let percent_int = percentage as u32;
        if percentage >= 100.0 {
            self.logger.error("Error: you are over your quota!");
        } else if percentage >= 70.0 {
            self.logger.warning(&format!(
                "Warning: you have used up over {}% of your quota! Proceeds with precaution",
                percent_int
            ));
        }
    }

    pub fn peek<T>(&self, i: &Rc<T>) {
        let count = Rc::strong_count(i);
        let percentage = (count as f64 / self.max as f64) * 100.0;
        let percent_int = percentage as u32;
        self.logger
            .info(&format!("Info: you are using up to {}% of your quota", percent_int));
    }
}
