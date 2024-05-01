trait AcademicEntity {
    fn name(&self) -> &str;
    fn print_details(&self);
}

impl AcademicEntity for Student {
    fn name(&self) -> &str {
        &self.name
    }
    fn print_details(&self) {
        println!("Student Name: {}", self.name);
        println!("Grades: {:?}", self.grades);
        println!("Average Grade: {:.2}", self.average_grade());
    }
}
struct Student {
    name: String,
    grades: Vec<u32>,
}

impl Student {
    fn new(name: &str, grades: Vec<u32>) -> Self {
        Self {
            name: String::from(name),
            grades,
        }
    }
    fn average_grade(&self) -> f64 {
        let sum: u32 = self.grades.iter().sum();
        let count = self.grades.len() as f64;
        sum as f64 / count
    }
}

fn main() {
    let alice = Student::new("Alice", vec![90, 85, 88]);
    let bob = Student::new("Bob", vec![78, 80, 85, 90]);
    let charlie = Student::new("Charlie", vec![95, 92, 98]);

    alice.print_details();
    println!();
    bob.print_details();
    println!();
    charlie.print_details();
}

