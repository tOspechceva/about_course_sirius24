/* Нужно создать функционал для упрошенного описания экономических отношений между работодателем, работником и компанией. Нужно реализовать классы Director, Employee, Company, Promise.
У компании должен быть метод, позволяющей получить прибыль условных единиц (flot). У Компании всегда есть только один Director, количество Employee не ограниченно. Также у компании должен быть метод позволяющий начислить зарплату всем ее работникам, согласно Promise каждого сотрудника компании.
У всех работников компании есть контракт с компанией о размере его заработной плате – Promise, а также есть имена, фамилии и номера СНИЛС(Id)
Promise - содержит в себе СНИЛС(Id) работника, размер его зарабтной платы, и информацию о том, была ли начисленна плата сотруднику
set_profit - метод, устанваливающий прибыль всей компании за некоторый промежуток времен.
fulfill_promise – метод, начисялющий всем заработную плату согласно персональному Promise каждого работника, если это возможно. Возвращает булевое значение
Замечание: метод set_profit устанавливает прибыль для комании. Компания в свою очередь, пытается выплатить заработную плату всем сотрудникам компании методом fulfill_promise. Если баланса компании хватает на выплаты всем участникам компании, то fulfill_promise возвращает true, в противном случаее false */

type Id = String; // СНИЛС — строка (например, "123-456-789 00")

#[derive(Debug, Clone)]
pub struct Promise {
    pub employee_id: Id,
    pub salary: f64,
    pub is_fulfilled: bool,
}

impl Promise {
    pub fn new(employee_id: Id, salary: f64) -> Self {
        Promise {
            employee_id,
            salary,
            is_fulfilled: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Employee {
    pub first_name: String,
    pub last_name: String,
    pub snils: Id,
    pub promise: Promise,
}

impl Employee {
    pub fn new(first_name: String, last_name: String, snils: Id, salary: f64) -> Self {
        let promise = Promise::new(snils.clone(), salary);
        Employee {
            first_name,
            last_name,
            snils,
            promise,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Director {
    pub first_name: String,
    pub last_name: String,
    pub snils: Id,
    pub promise: Promise,
}

impl Director {
    pub fn new(first_name: String, last_name: String, snils: Id, salary: f64) -> Self {
        let promise = Promise::new(snils.clone(), salary);
        Director {
            first_name,
            last_name,
            snils,
            promise,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Company {
    pub name: String,
    pub director: Director,
    pub employees: Vec<Employee>,
    pub profit: f64,
}

impl Company {
    pub fn new(name: String, director: Director) -> Self {
        Company {
            name,
            director,
            employees: Vec::new(),
            profit: 0.0,
        }
    }

    pub fn set_profit(&mut self, profit: f64) {
        self.profit = profit;
    }

    pub fn fulfill_promise(&mut self) -> bool {
        // Считаем общую сумму зарплат
        let mut total_salary = self.director.promise.salary;
        for emp in &self.employees {
            total_salary += emp.promise.salary;
        }

        // Проверяем, хватает ли прибыли
        if self.profit >= total_salary {
            // Начисляем зарплату
            self.director.promise.is_fulfilled = true;
            for emp in &mut self.employees {
                emp.promise.is_fulfilled = true;
            }
            self.profit -= total_salary;
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_company_fulfill_promise_success() {
        let director = Director::new(
            "Иван".to_string(),
            "Петров".to_string(),
            "111-222-333 00".to_string(),
            1000.0,
        );
        let mut company = Company::new("ООО Рога и Копыта".to_string(), director);
        company.employees.push(Employee::new(
            "Анна".to_string(),
            "Сидорова".to_string(),
            "444-555-666 00".to_string(),
            500.0,
        ));

        company.set_profit(2000.0);
        assert_eq!(company.fulfill_promise(), true);
        assert_eq!(company.director.promise.is_fulfilled, true);
        assert_eq!(company.employees[0].promise.is_fulfilled, true);
        assert!((company.profit - 500.0).abs() < 1e-6); // 2000 - 1000 - 500 = 500
    }

    #[test]
    fn test_company_fulfill_promise_failure() {
        let director = Director::new(
            "Иван".to_string(),
            "Петров".to_string(),
            "111-222-333 00".to_string(),
            1000.0,
        );
        let mut company = Company::new("ООО Рога и Копыта".to_string(), director);
        company.employees.push(Employee::new(
            "Анна".to_string(),
            "Сидорова".to_string(),
            "444-555-666 00".to_string(),
            500.0,
        ));

        company.set_profit(1200.0); // Мало для 1500
        assert_eq!(company.fulfill_promise(), false);
        assert_eq!(company.director.promise.is_fulfilled, false);
        assert_eq!(company.employees[0].promise.is_fulfilled, false);
        assert!((company.profit - 1200.0).abs() < 1e-6); // Прибыль не изменилась
    }

    #[test]
    fn test_empty_company() {
        let director = Director::new(
            "Босс".to_string(),
            "Главный".to_string(),
            "000-000-000 00".to_string(),
            2000.0,
        );
        let mut company = Company::new("Solo Ltd".to_string(), director);

        company.set_profit(2000.0);
        assert_eq!(company.fulfill_promise(), true);
        assert_eq!(company.director.promise.is_fulfilled, true);
        assert!((company.profit - 0.0).abs() < 1e-6); // 2000 - 2000 = 0
    }
}