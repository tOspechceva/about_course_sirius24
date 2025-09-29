mod tasks {
    pub mod common_functions;

    #[path = "1.rs"]
    pub mod task1;
    #[path = "1_1.rs"]
    pub mod task1_1;
    #[path = "2.rs"]
    pub mod task2;
    #[path = "2_1.rs"]
    pub mod task2_1;
    #[path = "3.rs"]
    pub mod task3;
    #[path = "3_1.rs"]
    pub mod task3_1;
    #[path = "4.rs"]
    pub mod task4;
    #[path = "4_1.rs"]
    pub mod task4_1;
    #[path = "5.rs"]
    pub mod task5;
    #[path = "5_1.rs"]
    pub mod task5_1;
    #[path = "5_2.rs"]
    pub mod task5_2;
}

fn main() {
    if let Err(e) = tasks::task5_2::task() {
        eprintln!("Ошибка при выполнении task_5_2: {}", e);
    }
    if let Err(e) = tasks::task1::task() {
        eprintln!("Ошибка при выполнении task_1: {}", e);
    }
    if let Err(e) = tasks::task1_1::task() {
        eprintln!("Ошибка при выполнении task_1_1: {}", e);
    }
    if let Err(e) = tasks::task2::task() {
        eprintln!("Ошибка при выполнении task_2: {}", e);
    }
    if let Err(e) = tasks::task2_1::task() {
        eprintln!("Ошибка при выполнении task_2_1: {}", e);
    }
    if let Err(e) = tasks::task3::task() {
        eprintln!("Ошибка при выполнении task_3: {}", e);
    }
    if let Err(e) = tasks::task3_1::task() {
        eprintln!("Ошибка при выполнении task_3_1: {}", e);
    }
    if let Err(e) = tasks::task4::task() {
        eprintln!("Ошибка при выполнении task_4: {}", e);
    }
    if let Err(e) = tasks::task4_1::task() {
        eprintln!("Ошибка при выполнении task_4_1: {}", e);
    }
    if let Err(e) = tasks::task5::task() {
        eprintln!("Ошибка при выполнении task_5: {}", e);
    }
    if let Err(e) = tasks::task5_1::task() {
        eprintln!("Ошибка при выполнении task_5_1: {}", e);
    }
}