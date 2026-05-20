use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let shared = create(); 
    assert_eq!(shared.borrow().is_none(), true);

    let a = shared.clone();
    let b = shared.clone();

    let value = a.borrow_mut();
    
    b.borrow_mut()
        .replace(String::from("new value"));

    println!("{:?}", value);
}
fn create() -> Rc<RefCell<Option<String>>> { 
    Rc::new(RefCell::new(None))
}
fn create() -> Rc<Option<RefCell<String>>>
fn create() -> RefCell<Rc<Option<String>>>
fn create() -> Option<Rc<RefCell<String>>>