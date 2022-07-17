// Simple
mod get;
mod set;

mod review;
mod traverse;

// Complex
mod lens;
mod prism;

// Impl
mod impls;

pub use get::*;
pub use lens::*;
pub use prism::*;
pub use review::*;
pub use set::*;
pub use traverse::*;

#[cfg(test)]
mod tests {
    use crate::data::{Person, Test};

    use super::*;

    #[test]
    fn at() {
        let mut olivier = Person {
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
        };

        let parents_lens = (Person::parents, Person::parents_mut, Person::parents_opt);
        let name_lens = (Person::name, Person::name_mut, Person::name_opt);

        // let mothers_name = parents_lens.then(Vec::<Person>::at(0)).then(name_lens);

        // assert_eq!(mothers_name.preview(&olivier).unwrap(), "Anne");
    }
}
