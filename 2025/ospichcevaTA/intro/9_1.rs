/*  Создайте класс Person, у которого есть:
Конструктор, принимающий имя, фамилию и возраст. Их необходимо сохранить в поля экземпляра класса first_name, last_name, age.
Метод full_name, который возвращает строку в виде "<Фамилия> <Имя>".
Метод is_adult, который возвращает True, если человек достиг 18 лет и False в противном случае. */
#[derive(Debug, Clone)]
pub struct Person {
    first_name: String,
    last_name: String,
    age: u32,
}

impl Person {
    // Конструктор (аналог __init__ в Python)
    pub fn new(first_name: String, last_name: String, age: u32) -> Self {
        Person {
            first_name,
            last_name,
            age,
        }
    }

    // Метод full_name: возвращает "Фамилия Имя"
    pub fn full_name(&self) -> String {
        format!("{} {}", self.last_name, self.first_name)
    }

    // Метод is_adult: true, если возраст >= 18
    pub fn is_adult(&self) -> bool {
        self.age >= 18
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_person_full_name() {
        let p = Person::new("Иван".to_string(), "Петров".to_string(), 25);
        assert_eq!(p.full_name(), "Петров Иван");
    }

    #[test]
    fn test_person_is_adult() {
        let adult = Person::new("Анна".to_string(), "Сидорова".to_string(), 18);
        let child = Person::new("Маша".to_string(), "Иванова".to_string(), 17);

        assert!(adult.is_adult());
        assert!(!child.is_adult());
    }

    #[test]
    fn test_person_edge_cases() {
        let newborn = Person::new("".to_string(), "".to_string(), 0);
        assert_eq!(newborn.full_name(), " ");
        assert!(!newborn.is_adult());
    }
}