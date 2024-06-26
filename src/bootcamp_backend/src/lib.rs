use std::cell::RefCell;

use ic_cdk::trap;

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
    POSTS.with_borrow_mut(|vec| {
        if index >= vec.len() {
            trap(&format!("Index ({}) out of bounds {}.", index, vec.len()));
        }

        vec.remove(index);
    });
}

#[ic_cdk::update]
fn clear() {
    // POSTS.with(|v| *v.borrow_mut() = Vec::new())
    POSTS.with_borrow_mut(|v| v.clear())
}

#[ic_cdk::update]
fn update_post(index: usize, entry: String) {
    POSTS.with_borrow_mut(|vec| {
        if index >= vec.len() {
            trap(&format!("Index ({}) out of bounds {}.", index, vec.len()));
        }

        if let Some(elem) = vec.get_mut(index) {
            *elem = entry;
        } else {
            trap(&format!("Index with id ({}) doesn't exist.", index));
        }
    })
}
