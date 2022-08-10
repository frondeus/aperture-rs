use crate::prelude::*;

pub mod lenses;

#[derive(Clone, Debug, PartialEq, Lens)]
pub struct Person {
    pub age: u32,
    pub name: String,
    pub parents: Vec<Person>,
    pub boss: Option<Box<Person>>,
}

#[derive(Clone, Debug, PartialEq, Prism)]
pub enum TestEnum {
    V1(String),
    V2,
}

#[derive(Clone, Debug, PartialEq, Lens)]
pub struct SomeStructure {
    pub person: Person,
    pub person_opt: Option<Person>,
    pub person_res: Result<Person, String>,
    pub persons: Vec<Person>,
}

#[derive(Clone, Debug, PartialEq, Lens)]
pub struct SomeNestedStructure {
    pub inner: Vec<SomeStructure>,
}

impl SomeStructure {
    pub fn test() -> Self {
        Self {
            person: Person::wojtek(),
            person_opt: Some(Person::olivier()),
            person_res: Err("String".into()),
            persons: vec![Person::wojtek(), Person::olivier()],
        }
    }
}

impl SomeNestedStructure {
    pub fn test() -> Self {
        Self {
            inner: vec![
                SomeStructure {
                    person: Person::wojtek(),
                    person_opt: Some(Person::olivier()),
                    person_res: Err("String".into()),
                    persons: vec![Person::wojtek(), Person::olivier()],
                },
                SomeStructure {
                    person: Person::olivier(),
                    person_opt: None,
                    person_res: Ok(Person::wojtek()),
                    persons: vec![Person::wojtek(), Person::olivier()],
                },
            ],
        }
    }
}

impl Person {
    pub fn olivier() -> Person {
        Person {
            age: 24,
            name: "Olivier".into(),
            boss: None,
            parents: vec![
                Person {
                    age: 55,
                    name: "Anne".to_string(),
                    parents: vec![],
                    boss: None,
                },
                Person {
                    age: 56,
                    name: "Thierry".to_string(),
                    parents: vec![],
                    boss: None,
                },
            ],
        }
    }

    pub fn wojtek() -> Person {
        Person {
            age: 27,
            name: "Wojtek".into(),
            boss: None,
            parents: vec![
                Person {
                    age: 72,
                    name: "Miroslawa".to_string(),
                    boss: None,
                    parents: vec![
                        Person {
                            age: 93,
                            name: "Lidia".to_string(),
                            parents: vec![],
                            boss: None,
                        },
                        Person {
                            age: 93,
                            name: "Jerzy".to_string(),
                            parents: vec![],
                            boss: None,
                        },
                    ],
                },
                Person {
                    boss: None,
                    age: 72,
                    name: "Zenon".to_string(),
                    parents: vec![
                        Person {
                            boss: None,
                            age: 93,
                            name: "Helena".to_string(),
                            parents: vec![],
                        },
                        Person {
                            boss: None,
                            age: 93,
                            name: "Waclaw".to_string(),
                            parents: vec![],
                        },
                    ],
                },
            ],
        }
    }
}
// #[allow(dead_code)]
// impl Person {
//     pub fn name(self) -> String {
//         self.name
//     }
//     pub fn name_mut(&mut self) -> &mut String {
//         &mut self.name
//     }
//     pub fn set_name(mut self, name: String) -> Self {
//         self.name = name;
//         self
//     }
//     pub fn name_opt(self) -> Option<String> {
//         Some(self.name)
//     }
// }

pub struct Test(pub String);
pub struct Arg;
#[allow(unused_variables)]
#[allow(dead_code)]
impl Test {
    pub fn ref_(&self) -> &String {
        &self.0
    }

    pub fn mut_(&mut self) -> &mut String {
        &mut self.0
    }

    pub fn opt_(&self) -> Option<&String> {
        Some(&self.0)
    }
    // pub fn set_(&mut self, s: String) {
    //     self.0 = s;
    // }

    pub fn prop_(&self) -> String {
        self.0.clone()
    }

    pub fn own_(self) -> String {
        self.0
    }

    pub fn own_opt(self) -> Option<String> {
        Some(self.0)
    }

    pub fn ref_arg(&self, arg: i32) -> &String {
        &self.0
    }

    pub fn mut_arg(&mut self, arg: i32) -> &mut String {
        &mut self.0
    }

    // pub fn set_arg(&mut self, s: String, arg: i32) {
    //     self.0 = s;
    // }

    pub fn prop_arg(&self, arg: i32) -> String {
        self.0.clone()
    }

    pub fn own_arg(self, arg: i32) -> String {
        self.0
    }

    pub fn ref_complex(&self, arg: Arg) -> &String {
        &self.0
    }

    pub fn mut_complex(&mut self, arg: Arg) -> &mut String {
        &mut self.0
    }

    // pub fn set_complex(&mut self, s: String, arg: Arg) {
    //     self.0 = s;
    // }

    pub fn prop_complex(&self, arg: Arg) -> String {
        self.0.clone()
    }

    pub fn own_complex(self, arg: Arg) -> String {
        self.0
    }
}
