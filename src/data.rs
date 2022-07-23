pub mod lenses {
    use super::Person;
    use crate::optics::{AffineFold, AffineTraversal, AsLens, Fold, Getter, Setter, Traversal};

    #[derive(Clone)]
    pub struct PersonName;

    impl Getter<AsLens, Person> for PersonName {
        type T = String;

        fn view(&self, source: Person) -> <Self as Getter<AsLens, Person>>::T {
            source.name
        }
    }
    impl AffineFold<AsLens, Person> for PersonName {
        type T = String;

        fn preview(&self, source: Person) -> Option<<Self as AffineFold<AsLens, Person>>::T> {
            Some(source.name)
        }
    }
    impl Fold<AsLens, Person> for PersonName {
        type D = std::option::IntoIter<String>;

        fn fold(&self, source: Person) -> Self::D {
            Some(source.name).into_iter()
        }
    }
    impl AffineTraversal<AsLens, Person> for PersonName {
        fn map_opt<T, F>(&self, source: Person, f: F) -> Option<T>
        where
            F: FnOnce(String) -> T,
        {
            Some(source.name).map(f)
        }
    }
    impl Traversal<AsLens, Person> for PersonName {
        fn traverse<F, T>(&self, source: Person, f: F) -> std::iter::Map<Self::D, F>
        where
            F: FnMut(String) -> T,
        {
            Some(source.name).into_iter().map(f)
        }
    }
    impl Setter<AsLens, Person> for PersonName {
        type T = String;
        type O = String;

        type D = Person;

        fn set<F>(&self, mut source: Person, mut f: F) -> Self::D
        where
            F: FnMut(Self::O) -> String,
        {
            source.name = f(source.name);
            source
        }
    }
    pub struct PersonMother;

    impl Getter<AsLens, Person> for PersonMother {
        type T = Person;

        fn view(&self, source: Person) -> <Self as Getter<AsLens, Person>>::T {
            source.parents.into_iter().next().unwrap()
        }
    }
    impl AffineFold<AsLens, Person> for PersonMother {
        type T = Person;

        fn preview(&self, source: Person) -> Option<<Self as AffineFold<AsLens, Person>>::T> {
            source.parents.into_iter().next()
        }
    }
    impl Fold<AsLens, Person> for PersonMother {
        type D = std::iter::Take<std::vec::IntoIter<Person>>;

        fn fold(&self, source: Person) -> Self::D {
            source.parents.into_iter().take(1)
        }
    }
    impl AffineTraversal<AsLens, Person> for PersonMother {
        fn map_opt<T, F>(&self, source: Person, f: F) -> Option<T>
        where
            F: FnOnce(Person) -> T,
        {
            source.parents.into_iter().take(1).next().map(f)
        }
    }
    impl Traversal<AsLens, Person> for PersonMother {
        fn traverse<F, T>(&self, source: Person, f: F) -> std::iter::Map<Self::D, F>
        where
            F: FnMut(Person) -> T,
        {
            source.parents.into_iter().take(1).map(f)
        }
    }
    impl Setter<AsLens, Person> for PersonMother {
        type T = Person;
        type O = Person;

        type D = Person;

        fn set<F>(&self, mut source: Person, f: F) -> Self::D
        where
            F: FnMut(Self::O) -> Person,
        {
            let mut iter = source.parents.into_iter();
            let new_mom = iter.next().map(f);
            source.parents = new_mom.into_iter().chain(iter).collect();
            source
        }
    }
    pub struct PersonParents;
    impl Getter<AsLens, Person> for PersonParents {
        type T = Vec<Person>;

        fn view(&self, source: Person) -> <Self as Getter<AsLens, Person>>::T {
            source.parents
        }
    }
    impl AffineFold<AsLens, Person> for PersonParents {
        type T = Vec<Person>;

        fn preview(&self, source: Person) -> Option<<Self as AffineFold<AsLens, Person>>::T> {
            Some(source.parents)
        }
    }
    impl Fold<AsLens, Person> for PersonParents {
        type D = std::option::IntoIter<Vec<Person>>;

        fn fold(&self, source: Person) -> Self::D {
            Some(source.parents).into_iter()
        }
    }
    impl AffineTraversal<AsLens, Person> for PersonParents {
        fn map_opt<T, F>(&self, source: Person, f: F) -> Option<T>
        where
            F: FnOnce(Vec<Person>) -> T,
        {
            Some(source.parents).map(f)
        }
    }
    impl Traversal<AsLens, Person> for PersonParents {
        fn traverse<F, T>(&self, source: Person, f: F) -> std::iter::Map<Self::D, F>
        where
            F: FnMut(Vec<Person>) -> T,
        {
            Some(source.parents).into_iter().map(f)
        }
    }
    impl Setter<AsLens, Person> for PersonParents {
        type T = Vec<Person>;
        type O = Vec<Person>;

        type D = Person;

        fn set<F>(&self, mut source: Person, mut f: F) -> Self::D
        where
            F: FnMut(Self::O) -> Vec<Person>,
        {
            source.parents = f(source.parents);
            source
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
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
