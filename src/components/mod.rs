use wasm_bindgen::JsCast;

mod done_list;
mod todo_list;

pub use done_list::*;
pub use todo_list::*;

fn get_element_by_id<T: JsCast>(element_id: impl AsRef<str>) -> T {
    let window = web_sys::window().expect("global window does not exists");
    let document = window.document().expect("expecting a document on window");
    let element = document.get_element_by_id(element_id.as_ref()).unwrap();
    element.dyn_into::<T>().unwrap()
}
