struct Student {
    name: String,
    scores: Vec<f64>,
}

trait GradeReport {
    fn average(&self) -> f64;
    fn highest(&self) -> f64;
    fn lowest(&self) -> f64;
    fn letter_grade(&self) -> char;
}

impl GradeReport for Student {
    fn average(&self) -> f64 {
        self.scores.iter().sum::<f64>() / self.scores.len() as f64
    }

    fn highest(&self) -> f64 {
        self.scores.iter().cloned().fold(f64::MIN, f64::max)
    }

    fn lowest(&self) -> f64 {
        self.scores.iter().cloned().fold(f64::MAX, f64::min)
    }

    fn letter_grade(&self) -> char {
        let avg = self.average();

        if avg >= 70.0 {
            'A'
        } else if avg >= 60.0 {
            'B'
        } else if avg >= 50.0 {
            'C'
        } else if avg >= 45.0 {
            'D'
        } else if avg >= 40.0 {
            'E'
        } else {
            'F'
        }
    }
}

pub fn run() {
    let students = vec![
        Student {
            name: String::from("Victor"),
            scores: vec![75.0, 82.0, 69.0],
        },
        Student {
            name: String::from("Grace"),
            scores: vec![58.0, 61.0, 55.0],
        },
        Student {
            name: String::from("John"),
            scores: vec![35.0, 42.0, 39.0],
        },
    ];

    println!("Student Grade Report");
    println!("====================");

    for student in students {
        println!("Name: {}", student.name);
        println!("Average: {:.2}", student.average());
        println!("Highest: {:.2}", student.highest());
        println!("Lowest: {:.2}", student.lowest());
        println!("Letter Grade: {}", student.letter_grade());
        println!();
    }
}
