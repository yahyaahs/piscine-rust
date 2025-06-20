mod err;
use std::error::Error;
pub use err::{ParseErr, ReadErr};

#[derive(Debug, Eq, PartialEq)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub level: u32,
}

#[derive(Debug, Eq, PartialEq)]
pub struct TodoList {
    pub title: String,
    pub tasks: Vec<Task>,
}

impl TodoList {
    pub fn get_todo(path: &str) -> Result<TodoList, Box<dyn Error>> {
        let read = std::fs::read_to_string(path);
        match read {
            Ok(str) => match json::parse(&str) {
                Ok(obj) => {
                    if obj["tasks"].len() == 0 {
                        return Err(Box::new(ParseErr::Empty));
                    }
                    Ok(Self {
                        title: obj["title"].as_str().unwrap_or("").to_owned(),
                        tasks: obj["tasks"]
                            .members()
                            .map(|mem| Task {
                                id: mem["id"].as_u32().unwrap_or(0),
                                description: mem["description"].as_str().unwrap_or("").to_owned(),
                                level: mem["level"].as_u32().unwrap_or(0),
                            })
                            .collect(),
                    })
                }
                Err(e) => Err(Box::new(ParseErr::Malformed(Box::new(e)))),
            },
            Err(e) => Err(Box::new(ReadErr {
                child_err: Box::new(e),
            })),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::{fs::File, io::Write};

    #[test]
    fn it_works() {
        let files = [
            (
                "todo.json",
                r#"{
                "title" : "TODO LIST FOR PISCINE RUST",
                "tasks": [
                    { "id": 0, "description": "do this", "level": 0 },
                    { "id": 1, "description": "do that", "level": 5 }
                ]
            }"#,
            ),
            (
                "todo_empty.json",
                r#"{
                "title" : "TODO LIST FOR PISCINE RUST",
                "tasks": []
            }"#,
            ),
            (
                "malformed_object.json",
                r#"{
                "something": ,
            }"#,
            ),
        ];

        for (name, content) in files {
            File::create(name)
                .unwrap()
                .write(content.as_bytes())
                .unwrap();

            let todos = TodoList::get_todo(name);
            match todos {
                Ok(list) => println!("{:?}", list),
                Err(e) => {
                    println!("{}: {:?}", e.to_string(), e.source());
                }
            }
        }
    }
}
