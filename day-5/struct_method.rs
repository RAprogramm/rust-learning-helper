struct Rectangle {                 // объявляем структуру Rectangle (прямоугольник)
    width: u32,                    // ширина
    height: u32,                   // высота
}

impl Rectangle {                   // реализация методов для Rectangle
    fn area(&self) -> u32 {        // метод area возвращает площадь прямоугольника
        self.width * self.height   // умножаем ширину на высоту
    }
}

fn main() {                        // начало программы
    let rect = Rectangle {         // создаём прямоугольник
        width: 10,                 // ширина 10
        height: 5,                 // высота 5
    };

    let a = rect.area();           // вызываем метод area и сохраняем результат
    println!("Площадь: {}", a);    // выводим площадь
}
