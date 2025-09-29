/* Создайте класс Laptop, у которого есть:
Конструктор, принимающий 3 аргумента: бренд, модель и цену ноутбука. На основании этих аргументов нужно для экземпляра создать атрибуты brand, model, price и также атрибут laptop_name — строковое значение, следующего вида: « brand model>»
Метод laptop_name возврашающий аналогичное значение поля. */
#[derive(Debug, Clone)]
pub struct Laptop {
    brand: String,
    model: String,
    price: f64, // или можно использовать u32/i64, если цена в целых единицах
    laptop_name: String,
}

impl Laptop {
    // Конструктор — аналог __init__ из Python
    pub fn new(brand: String, model: String, price: f64) -> Self {
        let laptop_name = format!("{} {}", brand, model);
        Laptop {
            brand,
            model,
            price,
            laptop_name,
        }
    }

    // Метод, возвращающий имя ноутбука (дублирует поле, как требуется)
    pub fn laptop_name(&self) -> &str {
        &self.laptop_name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_laptop_creation() {
        let laptop = Laptop::new("Dell".to_string(), "XPS 13".to_string(), 1200.0);
        assert_eq!(laptop.laptop_name(), "Dell XPS 13");
        assert_eq!(laptop.brand, "Dell");
        assert_eq!(laptop.model, "XPS 13");
        assert_eq!(laptop.price, 1200.0);
    }

    #[test]
    fn test_laptop_with_special_chars() {
        let laptop = Laptop::new("Apple".to_string(), "MacBook Pro".to_string(), 2499.99);
        assert_eq!(laptop.laptop_name(), "Apple MacBook Pro");
    }
}