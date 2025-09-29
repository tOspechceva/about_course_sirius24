/*
Напишите функцию my_sort, которая принимает двумерный массив n x n, а возвращает элементы массива, расположенные от самых крайних элементов до среднего элемента, таким образом, чтобы происходило перемещение по часовой стрелке.
Тоесть функция должна преобразовывать array = [ [1,2,3], [4,5,6], [7,8,9] ]
my_sort(array) #=> [1,2,3,6,9,8,7,4,5]
Еще один пример работы функции:
array = [ [1,2,3], [8,9,4], [7,6,5] ] my_sort(array) #=> [1,2,3,4,5,6,7,8,9]
*/
use super::common_functions;
use std::io;

pub fn task() -> io::Result<()> {
    println!("Введите размер квадратной матрицы n:");
    let n: u32 = common_functions::input_int("n = ")?;

    if n == 0 {
        println!("Матрица пуста.");
        return Ok(());
    }

    let n = n as usize;
    let mut matrix = Vec::new();

    println!("Введите элементы матрицы построчно:");
    for i in 0..n {
        let mut row = Vec::new();
        for j in 0..n {
            let val: i32 = common_functions::input_int(&format!("matrix[{}][{}] = ", i, j))?;
            row.push(val);
        }
        matrix.push(row);
    }

    let result = my_sort(matrix);
    println!("Результат обхода по спирали: {:?}", result);

    Ok(())
}

pub fn my_sort(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    if matrix.is_empty() {
        return Vec::new();
    }

    let n = matrix.len();
    let mut result = Vec::with_capacity(n * n);

    let mut top = 0;
    let mut bottom = n - 1;
    let mut left = 0;
    let mut right = n - 1;

    while top <= bottom && left <= right {
        // Вправо по верхней строке
        for col in left..=right {
            result.push(matrix[top][col]);
        }
        top += 1;

        // Вниз по правому столбцу
        for row in top..=bottom {
            result.push(matrix[row][right]);
        }
        if right == 0 { break; }
        right -= 1;

        // Влево по нижней строке (если строка осталась)
        if top <= bottom {
            for col in (left..=right).rev() {
                result.push(matrix[bottom][col]);
            }
            if bottom == 0 { break; }
            bottom -= 1;
        }

        // Вверх по левому столбцу (если столбец остался)
        if left <= right {
            for row in (top..=bottom).rev() {
                result.push(matrix[row][left]);
            }
            left += 1;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_sort() {
        let arr1 = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ];
        assert_eq!(my_sort(arr1), vec![1, 2, 3, 6, 9, 8, 7, 4, 5]);

        let arr2 = vec![
            vec![1, 2, 3],
            vec![8, 9, 4],
            vec![7, 6, 5],
        ];
        assert_eq!(my_sort(arr2), vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);

        assert_eq!(my_sort(vec![vec![42]]), vec![42]);
    }
}