pub mod lenses;

#[derive(Clone, Debug, PartialEq)]
pub struct Person {
    pub age: u32,
    pub name: String,
    pub parents: Vec<Person>,
}

impl Person {
    pub fn olivier() -> Person {
        Person {
            age: 24,
            name: "Olivier".into(),
            parents: vec![
                Person {
                    age: 55,
                    name: "Anne".to_string(),
                    parents: vec![],
                },
                Person {
                    age: 56,
                    name: "Thierry".to_string(),
                    parents: vec![],
                },
            ],
        }
    }

    pub fn wojtek() -> Person {
        Person {
            age: 27,
            name: "Wojtek".into(),
            parents: vec![
                Person {
                    age: 72,
                    name: "Miroslawa".to_string(),
                    parents: vec![
                        Person {
                            age: 93,
                            name: "Lidia".to_string(),
                            parents: vec![],
                        },
                        Person {
                            age: 93,
                            name: "Jerzy".to_string(),
                            parents: vec![],
                        },
                    ],
                },
                Person {
                    age: 72,
                    name: "Zenon".to_string(),
                    parents: vec![
                        Person {
                            age: 93,
                            name: "Helena".to_string(),
                            parents: vec![],
                        },
                        Person {
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
#[allow(dead_code)]
impl Person {
    pub fn name(self) -> String {
        self.name
    }
    pub fn name_mut(&mut self) -> &mut String {
        &mut self.name
    }
    pub fn set_name(mut self, name: String) -> Self {
        self.name = name;
        self
    }
    pub fn name_opt(self) -> Option<String> {
        Some(self.name)
    }
}

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
