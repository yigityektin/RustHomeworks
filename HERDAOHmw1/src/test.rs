#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_student() {
        let mut students: Vec<Student> = Vec::new();
        let name = "Yigit Yektin\n".to_string();
        let age = "19\n".to_string();
        let grade = "80\n".to_string();

        let input = format!("{}{}{}", name, age, grade);
        let mut cursor = std::io::Cursor::new(input);

        add_student(&mut students, &mut cursor);

        assert_eq!(students.len(), 1);
        assert_eq!(students[0].name, "Yigit Yektin");
        assert_eq!(students[0].age, "19");
        assert_eq!(students[0].grade, "80");
    }

    #[test]
    fn test_get_average_grade() {
        let students = vec![
            Student {
                name: "Mustafa".to_string(),
                age: 15,
                grade: 100,
            },
            Student {
                name: "Kemal".to_string(),
                age: 12,
                grade: 100,
            },
            Student {
                name: "Fahrettin".to_string(),
                age: 48,
                grade: 99,
            },
        ];

        let avg = get_average_grade(&students);
        assert_eq!(avg, 80.0);
    }
}