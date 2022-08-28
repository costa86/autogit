use autogit::*;
mod gist;
mod repository;

fn main() {
    display_app_intro("white");
    loop {
        let (_, index) = get_user_selection(&CHOICES.to_vec(), "Option");

        match index {
            0 => repository::display_repositories(),
            1 => repository::create_repository(),
            2 => gist::display_gists(),
            _ => break,
        }
    }
}
