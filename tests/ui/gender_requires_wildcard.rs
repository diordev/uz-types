use uz_types::Gender;

fn main() {
    let gender = Gender::Male;
    let _ = match gender {
        Gender::Male => "erkak",
        Gender::Female => "ayol",
    };
}
