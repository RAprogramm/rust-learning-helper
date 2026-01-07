fn main() {                          // начало программы
    let mut count = 0;               // создаём переменную count

    loop {                           // бесконечный цикл
        println!("count = {}", count); // выводим count
        count += 1;                  // увеличиваем count

        if count == 3 {              // если count стал равен 3
            break;                   // выходим из цикла
        }
    }                                // конец цикла
}
