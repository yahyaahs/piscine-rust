pub use std::rc::Rc;

#[derive(Debug)]
pub struct Node {
    pub ref_list: Vec<Rc<String>>,
}

impl Node {
    pub fn new(ref_list: Vec<Rc<String>>) -> Node {
        Node { ref_list: ref_list }
    }
    pub fn add_element(&mut self, element: Rc<String>) {
        self.ref_list.push(element);
    }
    pub fn rm_all_ref(&mut self, element: Rc<String>) {
        self.ref_list.retain(|el| !Rc::ptr_eq(el, &element)&& element!=*el );
    }
}

pub fn how_many_references(ref_list: &Rc<String>) -> usize {
    return Rc::strong_count(&ref_list);
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
            let a = Rc::new(String::from("a"));
    let b = Rc::new(String::from("b"));
    let c = Rc::new(String::from("c"));

    let a1 = Rc::new(String::from("a"));

    let mut new_node = Node::new(vec![a.clone()]);
    new_node.add_element(b.clone());
    new_node.add_element(a.clone());
    new_node.add_element(c.clone());
    new_node.add_element(a.clone());

    println!("a: {:?}", how_many_references(&a));
    println!("b: {:?}", how_many_references(&b));
    println!("c: {:?}", how_many_references(&c));
    // println!("{:#?}", new_node);
    new_node.rm_all_ref(a1.clone());
    // println!("{:#?}", new_node);

    new_node.rm_all_ref(a.clone());

    println!("a: {:?}", how_many_references(&a));
    println!("b: {:?}", how_many_references(&b));
    println!("c: {:?}", how_many_references(&c));
    }
}
