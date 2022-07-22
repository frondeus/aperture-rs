#[derive(Clone, Debug)]
pub struct Person {
    pub age: u32,
    pub name: String,
    pub parents: Vec<Person>,
}

impl Person {
    pub fn name(&self) -> &String {
        &self.name
    }
    pub fn name_mut(&mut self) -> &mut String {
        &mut self.name
    }
    pub fn set_name(mut self, name: String) -> Self {
        self.name = name;
        self
    }
    pub fn name_opt(&self) -> Option<&String> {
        Some(&self.name)
    }
    pub fn mother(&self) -> &Person {
        &self.parents[0]
    }
    pub fn mother_mut(&mut self) -> &mut Person {
        &mut self.parents[0]
    }
    pub fn mother_opt(&self) -> Option<&Person> {
        self.parents.get(0)
    }
    pub fn parents(&self) -> &Vec<Person> {
        &self.parents
    }
    pub fn parents_mut(&mut self) -> &mut Vec<Person> {
        &mut self.parents
    }
    pub fn parents_opt(&self) -> Option<&Vec<Person>> {
        Some(&self.parents)
    }
}

pub struct Test(pub String);
pub struct Arg;
#[allow(unused_variables)]
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
