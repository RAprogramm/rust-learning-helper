fn main() {                                  // начало программы
    let mut words = Vec::new();              // создаём пустой вектор строк

    words.push(String::from("Привет"));      // добавляем строку
    words.push(String::from("мир"));         // добавляем строку

    for w in &words {                        // перебираем элементы по ссылке
        println!("{}", w);                   // выводим строку
    }
}
