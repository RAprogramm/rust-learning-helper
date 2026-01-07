struct Point {                // объявляем структуру Point
    x: i32,                   // координата x
    y: i32,                   // координата y
}

impl Point {                  // начинаем блок impl — реализация методов
    fn print(&self) {         // метод print, принимает ссылку на себя (&self)
        println!("({}, {})", self.x, self.y); // выводит координаты точки
    }
}

fn main() {                   // начало программы
    let p = Point { x: 3, y: 7 }; // создаём точку p
    p.print();                // вызываем метод print у точки p
}
