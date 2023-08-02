use card::Card;
use components::*;
use leptos::*;
use list::List;
use storage::load_and_create_list_signal;

mod card;
mod components;
mod list;
mod storage;

fn main() {
    mount_to_body(|cx| {
        move || {
            let (todo_list, set_todo_list) = load_and_create_list_signal(cx, "todo-todo-list");
            let (done_list, set_done_list) = load_and_create_list_signal(cx, "todo-done-list");

            view! { cx,
                <main class="flex flex-col justify-center items-center m-4 gap-6">
                    <ToDoCardList list={todo_list} set_list={set_todo_list} done_card_set_list={set_done_list} />
                    <DoneCardList list={done_list} />
                </main>
            }
        }
    })
}
