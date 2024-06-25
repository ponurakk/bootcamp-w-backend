#[ic_cdk::query]
fn greet(name: String, age: i32) -> String {
    format!("Hello, {}. {} years old!", name, age)
}
