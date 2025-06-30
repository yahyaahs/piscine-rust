use std::ops::Deref;

#[derive(Clone, Debug, PartialEq)]
pub struct List<T> {
    pub head: Option<Box<Node<T>>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Node<T> {
    pub value: T,
    pub next: Option<Box<Node<T>>>,
}

impl<T> List<T> {
    pub fn new() -> List<T> {
        List { head: None }
    }

    pub fn push(&mut self, value: T) {
        match &self.head {
            Some(_) => {
                let new = Box::new(Node {
                    value,
                    next: self.head.take(),
                });
                self.head = Some(new);
            }
            _ => self.head = Some(Box::new(Node { value, next: None })),
        }
    }

    pub fn pop(&mut self) {
        self.head.take().map(|f|{
            self.head = f.next;
        });
    }

    pub fn len(&self) -> usize {
        let mut counter = 0;
        let mut curr = &self.head;
        while let Some(node) = curr  {
            curr = &node.next;
            counter+=1;
        }
        return counter as usize;
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut new_list_str = List::new();
        new_list_str.push("String Test 1");
        println!("The size of the list is {:#?}", new_list_str.len());

        new_list_str.push("String Test 2");
        println!("The size of the list is {}", new_list_str.len());

        new_list_str.push("String Test 3");
        println!("The size of the list is {}", new_list_str.len());

        new_list_str.pop();
        println!("The size of the list is {}", new_list_str.len());
    }
}
