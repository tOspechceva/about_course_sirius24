/* Создайте класс Point. У этого класса должны быть:
Метод set_coordinates, который принимает координаты точки на плоскости и сохраняет их в экземпляр класса в атрибуты x и y.
Метод get_distance, который обязательно принимает экземпляр класса Point и возвращает расстояние между двумя точками по теореме Пифагора. В случае, если в данный метод передается не экземпляр класса Point, необходимо вывести сообщение "Передана не точка". */

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    // Метод set_coordinates — создаёт новую точку с заданными координатами
    pub fn set_coordinates(x: f64, y: f64) -> Self {
        Point { x, y }
    }

    // Метод get_distance — принимает ссылку на другую точку и возвращает расстояние
    pub fn get_distance(&self, other: &Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }

   
}

pub fn safe_get_distance(p1: &Point, maybe_p2: Option<&Point>) -> Option<f64> {
    match maybe_p2 {
        Some(p2) => Some(p1.get_distance(p2)),
        None => {
            println!("Передана не точка");
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_distance() {
        let p1 = Point::set_coordinates(0.0, 0.0);
        let p2 = Point::set_coordinates(3.0, 4.0);
        let dist = p1.get_distance(&p2);
        assert!((dist - 5.0).abs() < 1e-9);
    }

    #[test]
    fn test_safe_distance_none() {
        let p1 = Point::set_coordinates(1.0, 1.0);
        let result = safe_get_distance(&p1, None);
        assert_eq!(result, None);
    }
}