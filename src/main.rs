use libs::dir_size;

mod list;
mod user_input;
mod libs {
    pub mod dir_size;
}
mod utils {
    pub mod message;
}

fn main() {
    dir_size::dothis();
    utils::message::message();
    while user_input::handle_user_input() {
        list::list_files();
    }
}
