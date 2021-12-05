use rand::Rng;
use std::cmp::Ordering;
use std::io::stdin;

const ERROR_MESSAGE: &str = "Не удалось прочитать информацию из stdio :(";

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..21);
    print_headline();
    println!("Секретное число — {}", secret_number);

    loop {
        println!("А теперь попробуйте ввести свое предположение");
        
        let mut guess = String::new();
        stdin()
            .read_line(&mut guess)
            .expect(ERROR_MESSAGE);
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Ты загадал(-а): {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Неверно, загаданное число больше"),
            Ordering::Greater => println!("Неверно, загаданное число меньше"),
            Ordering::Equal => {
                println!("Поздравляю, вы выиграли!!!");
                break;
            }, 
        }
    }
}

fn print_headline() {
    println!("ИГРА «УГАДАЙ ЧИСЛО»!");
    println!("Я загадал тебе число от 1 до 20. Попробуй угадать)");
}
