/* Нужно описать структуры для платформы электронного обучения. В ней есть администраторы системы, преподаватели курсов и студенты, а также группы учебные и домашние задания. */

use std::collections::HashMap;

// Общий идентификатор для всех сущностей
type Id = u32;

// Общая информация о пользователе
#[derive(Debug, Clone)]
pub struct User {
    pub id: Id,
    pub name: String,
    pub email: String,
}

// Роли пользователей
#[derive(Debug, Clone)]
pub struct Admin {
    pub user: User,
}

#[derive(Debug, Clone)]
pub struct Teacher {
    pub user: User,
    pub courses: Vec<Id>, // список ID курсов, которые ведёт преподаватель
}

#[derive(Debug, Clone)]
pub struct Student {
    pub user: User,
    pub group_id: Option<Id>, // студент может быть в группе или без неё
}

// Учебная группа
#[derive(Debug, Clone)]
pub struct StudyGroup {
    pub id: Id,
    pub name: String,
    pub student_ids: Vec<Id>, // список ID студентов в группе
    pub course_id: Id,        // группа привязана к курсу
}

// Домашнее задание
#[derive(Debug, Clone)]
pub struct Homework {
    pub id: Id,
    pub title: String,
    pub description: String,
    pub course_id: Id,
    pub due_date: String, // можно использовать chrono::NaiveDate, но для простоты — String
}

// Курс
#[derive(Debug, Clone)]
pub struct Course {
    pub id: Id,
    pub title: String,
    pub description: String,
    pub teacher_id: Id,
    pub homework_ids: Vec<Id>,
    pub group_ids: Vec<Id>,
}

// Конструкторы для удобства

impl Admin {
    pub fn new(id: Id, name: String, email: String) -> Self {
        Admin {
            user: User { id, name, email },
        }
    }
}

impl Teacher {
    pub fn new(id: Id, name: String, email: String) -> Self {
        Teacher {
            user: User { id, name, email },
            courses: Vec::new(),
        }
    }
}

impl Student {
    pub fn new(id: Id, name: String, email: String) -> Self {
        Student {
            user: User { id, name, email },
            group_id: None,
        }
    }
}

impl StudyGroup {
    pub fn new(id: Id, name: String, course_id: Id) -> Self {
        StudyGroup {
            id,
            name,
            student_ids: Vec::new(),
            course_id,
        }
    }
}

impl Homework {
    pub fn new(id: Id, title: String, description: String, course_id: Id, due_date: String) -> Self {
        Homework {
            id,
            title,
            description,
            course_id,
            due_date,
        }
    }
}

impl Course {
    pub fn new(id: Id, title: String, description: String, teacher_id: Id) -> Self {
        Course {
            id,
            title,
            description,
            teacher_id,
            homework_ids: Vec::new(),
            group_ids: Vec::new(),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_platform_entities() {
        let admin = Admin::new(1, "Админ Иван".to_string(), "admin@example.com".to_string());
        let teacher = Teacher::new(2, "Преподаватель Анна".to_string(), "teacher@example.com".to_string());
        let student = Student::new(3, "Студент Пётр".to_string(), "student@example.com".to_string());

        let mut course = Course::new(101, "Rust для начинающих".to_string(), "Курс по основам Rust".to_string(), teacher.user.id);
        course.homework_ids.push(201);
        course.group_ids.push(301);

        let mut group = StudyGroup::new(301, "Группа Rust-1".to_string(), course.id);
        group.student_ids.push(student.user.id);

        let homework = Homework::new(
            201,
            "Первое задание".to_string(),
            "Написать структуру Point".to_string(),
            course.id,
            "2025-06-01".to_string(),
        );

        assert_eq!(admin.user.name, "Админ Иван");
        assert_eq!(teacher.user.email, "teacher@example.com");
        assert_eq!(student.group_id, None);
        assert_eq!(course.teacher_id, 2);
        assert_eq!(group.course_id, 101);
        assert_eq!(homework.title, "Первое задание");
    }
}