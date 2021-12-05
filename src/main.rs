use std::io::stdin;

fn main() {
    println!("Угадай число!");
    println!("Для того, чтобы продолжить, введите свое предположение :)");

    let mut guess = String::new();
    let error_message = "Не удалось прочитать ввод из командной строки";

    stdin()
        .read_line(&mut guess)
        .expect(&error_message);

    println!("You guessed: {}", guess);
}
