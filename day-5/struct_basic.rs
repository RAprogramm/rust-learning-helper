struct User {                 // объявляем структуру с именем User
    name: String,             // поле name — это строка
    age: u32,                 // поле age — это число без знака
}

fn main() {                   // начало программы
    let user = User {         // создаём переменную user типа User
        name: String::from("Валера"), // записываем имя
        age: 25,              // записываем возраст
    };

    println!("Имя: {}", user.name); // выводим имя пользователя
    println!("Возраст: {}", user.age); // выводим возраст
}
