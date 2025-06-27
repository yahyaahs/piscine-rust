#[derive(Debug)]
pub struct WorkEnvironment {
    pub grade: Link,
}

pub type Link = Option<Box<Worker>>;

#[derive(Debug)]
pub struct Worker {
    pub role: String,
    pub name: String,
    pub next: Link,
}

impl WorkEnvironment {
    pub fn new() -> WorkEnvironment {
        return WorkEnvironment{
            grade : None,
        };
    }
    pub fn add_worker(&mut self, role: String, name: String) {
        match self.grade.take() {
            Some(item)=>  {

               self.grade =  Some(Box::new(Worker{
                    role,
                    name,
                    next : Some(item),
                }));
            },
            None => {self.grade = Some(Box::new(Worker{
                role,
                name,
                next : None,
            }));
        },
        }

    }
    pub fn remove_worker(&mut self) -> Option<String> {
        if self.grade.is_none(){
            return None;
        }else {
            let mut head = self.grade.as_mut().unwrap();
            while !head.next.is_none() {
                head = head.next.as_mut().unwrap();
            }
            head.next = None;
            return Some(head.name.clone());
        }
    }
    pub fn last_worker(&self) -> Option<(String, String)> {
        let mut  head = self.grade.as_ref()?;
        while !head.next.is_none(){
            head = head.next.as_ref().unwrap();
        }
        return Some((head.name.clone(), head.role.clone()));
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    let mut list = WorkEnvironment::new();
    list.add_worker(String::from("CEO"), String::from("Marie"));
    list.add_worker(String::from("Manager"), String::from("Monica"));
    list.add_worker(String::from("Normal Worker"), String::from("Ana"));
    list.add_worker(String::from("Normal Worker"), String::from("Alice"));
    println!("{:#?}", list);
    list.remove_worker();
    println!("{:#?}", list);
    }
}
