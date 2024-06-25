use std::cell::RefCell;

thread_local! {
    pub static POSTS: RefCell<Vec<String>> = RefCell::default();
}

#[ic_cdk::query]
fn greet(name: String, age: i32) -> String {
    format!("Hello, {}. {} years old!", name, age)
}

#[ic_cdk::update]
fn add_post(entry: String) {
    POSTS.with(|v| v.borrow_mut().push(entry));
}

#[ic_cdk::query]
fn get_posts() -> Vec<String> {
    POSTS.with_borrow(|v| v.to_owned())
}

#[ic_cdk::update]
fn remove_post(index: usize) {
    POSTS.with_borrow_mut(|v| {
        let vec = v;
        if index < vec.len() {
            vec.remove(index);
        }
    });
}
