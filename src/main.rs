use autogit::*;

fn main() {
    display_app_intro("white");
    loop {
        let (_, index) = get_user_selection(&CHOICES.to_vec(), "Option");

        match index {
            0 => display_repositories(),
            1 => create_repository(),
            _ => break,
        }
    }
}
