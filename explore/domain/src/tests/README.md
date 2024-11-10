### example usage of the when
```rust
pub trait Service {
    fn do_work(&self, a: i32, b: i32) -> i32;
    fn fetch_data(&self, query: String, limit: i32) -> String;
}

pub struct RealService {
    mock_do_work: Double<(i32, i32), i32>,
    mock_fetch_data: Double<(String, i32), String>,
}

impl RealService {
    pub fn new() -> Self {
        Self {
            mock_do_work: Double::new(),
            mock_fetch_data: Double::new(),
        }
    }

    pub fn mock_do_work(&self) -> &Double<(i32, i32), i32> {
        &self.mock_do_work
    }

    pub fn mock_fetch_data(&self) -> &Double<(String, i32), String> {
        &self.mock_fetch_data
    }
}

fn main() {
    let service = RealService::new();

    service
        .mock_do_work()
        .when((10, 20))
        .then_return(30);

    service
        .mock_fetch_data()
        .when(("select *".to_string(), 10))
        .then_return("data".to_string());

    let result = service.do_work(10, 20);
    println!("Result of do_work(10, 20): {}", result);  // Expected: 30

    let result2 = service.fetch_data("select *".to_string(), 10);
    println!("Result of fetch_data(\"select *\", 10): {}", result2);  // Expected: "data"

    // This will panic with an error message for unexpected argument
    let result3 = service.do_work(20, 30);
    println!("Result of do_work(20, 30): {}", result3);  // Should panic
}
```