use std::cell::{Cell, RefCell};
use std::ops::Drop;
use std::rc::Rc;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Workers {
    pub drops: Cell<usize>,
    pub states: RefCell<Vec<bool>>,
}

impl Drop for Thread<'_> {
    fn drop(&mut self) {
        self.parent.add_drop(self.id);
    }
}
impl Workers {
    pub fn new() -> Workers {
        Workers {
            drops: Cell::new(0),
            states: RefCell::new(Vec::new()),
        }
    }
    pub fn new_worker(&self, c: String) -> (usize, Thread) {
        let pid = self.states.borrow().len();
        self.states.borrow_mut().push(false);
        let thread = Thread::new_thread(pid, c, self);
        (pid, thread)
    }
    pub fn track_worker(&self) -> usize {
        self.states.borrow().len()
    }
    pub fn is_dropped(&self, id: usize) -> bool {
        self.states.borrow_mut()[id]
    }
    pub fn add_drop(&self, id: usize) {
        if self.states.borrow()[id] == true {
            panic!("{} is already dropped", id);
        } else {
            self.states.borrow_mut()[id] = true;
            self.drops.set(self.drops.get() + 1);
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Thread<'a> {
    // expected public fields
    pub id: usize,
    pub cmd: String,
    pub parent: &'a Workers,
}

impl<'a> Thread<'a> {
    pub fn new_thread(p: usize, c: String, t: &'a Workers) -> Thread<'a> {
        Thread {
            id: p,
            cmd: c,
            parent: t,
        }
    }
    pub fn skill(self) {
        drop(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let worker = Workers::new();
        let (id, thread) = worker.new_worker(String::from("command"));
        let (id1, thread1) = worker.new_worker(String::from("command1"));

        thread.skill();

        println!("{:?}", (worker.is_dropped(id), id, &worker.drops));

        thread1.skill();
        println!("{:?}", (worker.is_dropped(id1), id1, &worker.drops));

        let (id2, thread2) = worker.new_worker(String::from("command2"));
        let thread2 = Rc::new(thread2);
        let thread2_clone = thread2.clone();

        drop(thread2_clone);

        println!(
            "{:?}",
            (
                worker.is_dropped(id2),
                id2,
                &worker.drops,
                Rc::strong_count(&thread2)
            )
        );
    }
}
