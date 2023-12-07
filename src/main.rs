mod list;
mod user_input;

fn main() {
    while user_input::handle_user_input() {
        list::list_files();
    }
}
