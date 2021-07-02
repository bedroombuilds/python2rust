trait Person {
    fn name(&self) -> String {
        String::from("Unnamed")
    }
}

trait Student: Person {
    fn university(&self) -> String;
}

trait Programmer {
    fn fav_language(&self) -> String;
}

trait CompSciStudent: Programmer + Student {
    fn git_username(&self) -> String;
}

struct StreetPerson(String);
struct CollegeStudent(String);
struct RustProgrammer(String);
struct ArtificialProgrammer(String);
struct SuperCompSciStudent(String);

impl Person for StreetPerson {}

impl Person for RustProgrammer {}
impl Programmer for RustProgrammer {
    fn fav_language(&self) -> String {
        String::from("Rust")
    }
}

impl Programmer for ArtificialProgrammer {
    fn fav_language(&self) -> String {
        String::from("Any")
    }
}

impl Person for CollegeStudent {}
impl Student for CollegeStudent {
    fn university(&self) -> String {
        String::from("Community College")
    }
}

impl Person for SuperCompSciStudent {
    fn name(&self) -> String {
        self.0.clone()
    }
}
impl Programmer for SuperCompSciStudent {
    fn fav_language(&self) -> String {
        String::from("Rust and Python")
    }
}
impl Student for SuperCompSciStudent {
    fn university(&self) -> String {
        String::from("Alma Mater")
    }
}
impl CompSciStudent for SuperCompSciStudent {
    fn git_username(&self) -> String {
        self.0.to_lowercase()
    }
}

fn comp_sci_student_greeting(student: &dyn CompSciStudent) -> String {
    format!(
        "My name is {} and I attend {}. My favorite language is {}. My Git username is {}",
        student.name(),
        student.university(),
        student.fav_language(),
        student.git_username()
    )
}

fn comp_sci<T: CompSciStudent>(student: &T) -> String {
    comp_sci_student_greeting(student)
}

fn comp_sci_vs_programmer<T, U>(student: &T, programmer: &U) -> String
where
    T: CompSciStudent,
    U: Programmer + Person,
{
    println!(
        "Programmer named {} likes {}",
        programmer.name(),
        programmer.fav_language()
    );
    comp_sci_student_greeting(student)
}

fn make_person(rnd: u8) -> Box<dyn Person> {
    match rnd {
        0..=3 => Box::new(StreetPerson("Bob".to_string())),
        4..=6 => Box::new(CollegeStudent("Bobby".to_string())),
        7..=9 => Box::new(RustProgrammer("Robert".to_string())),
        _ => Box::new(SuperCompSciStudent("Bert".to_string())),
    }
}

fn main() {
    let p = make_person(8);
    println!("{}", p.name());
    let css = SuperCompSciStudent("Bert".to_string());
    println!("{}", comp_sci_student_greeting(&css));
    println!("{}", comp_sci(&css));
    let prog = RustProgrammer("Robert".to_string());
    //let prog = ArtificialProgrammer("AI".to_string());
    println!("{}", comp_sci_vs_programmer(&css, &prog));
}
