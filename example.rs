// Press `shift + g` to move to the end of the file
trait AcademicEntity {
    fn name(&self) -> &str;
    fn print_details(&self);
}
// press `19j` to move 19 lines down
impl AcademicEntity for Student {
    fn name(&self) -> &str {
        &self.name
    }
    fn print_details(&self) {
        // Press `j` to move to next line then `w` to move to next word, and continue pressing `w` to move to next words
        println!("Student Name: {}", self.name);
        println!("Grades: {:?}", self.grades);
        //Move to cursor to letter `A` in Average and press `d2w` to delete `Average Grade`
        println!("Average Grade: {:.2}", self.average_grade());
    }
}
struct Student {
    name: String,
    grades: Vec<u32>,
}

impl Student {
    // Move to next line and press `ci(` to change parameters
    fn new(name: &str, grades: Vec<u32>) -> Self {
        Self {
            name: String::from(name),
            grades,
        }
    }
    // Press `2j` to move the cursor 2 line down then `ci{` to change the block
    fn average_grade(&self) -> f64 {
        let sum: u32 = self.grades.iter().sum();
        let count = self.grades.len() as f64;
        sum as f64 / count
    }
}
// Press `j` move to next line and press `f{` to move to `{` then press `%` to move to the matching `}`
fn main() {
    let bob = Student::new("Bob", vec![78, 80, 85, 90]);
    let charlie = Student::new("Charlie", vec![95, 95, 119]);

    alice.print_details();
    println!();
    bob.print_details();
    println!();
    charlie.print_details();
}
// Press `gg` to move to the top of the file
