# День 1 — Переменные (day-1)

Этот каталог содержит материалы по теме "Переменные в Rust":
- `variables.rs` — демонстрационный пример с подробными русскоязычными комментариями.
- `variables_exercises.rs` — интерактивная версия с заданиями, подсказками и unit-тестами; в комментариях подробно объяснены используемые символы и конструкции.
- `variables_short.rs` — краткая версия-шпаргалка: компактный пример с пояснениями по ключевым символам.
- `strings.rs`, `tasks.md` — существующие файлы в каталоге.

Ниже — инструкция с командами терминала (Linux / macOS). Команды показаны пошагово и с пояснениями, чтобы вы могли:
- проверить содержимое каталога,
- скопировать или переместить файлы в проект,
- создать локальный cargo-проект для тестирования,
- выполнить тесты и пример,
- сделать git-коммит и отправить ветку в удалённый репозиторий.

Важно: команды выполняйте в корне репозитория (или указывайте пути явно). Если вы работаете на Windows, используйте Git Bash или WSL для тех же команд.

1) Просмотр файлов в каталоге day-1
- Показать список файлов (подробно):
  - ls -la day-1
  - Пример: 
    - ls -la day-1
      - выводит все файлы и их права/размер/время
- Показать структуру (если установлен tree):
  - tree -a day-1
- Просмотреть начало файла:
  - head -n 40 day-1/variables.rs
- Просмотреть весь файл:
  - less day-1/variables.rs
  - или
  - cat day-1/variables.rs

2) Копирование / перемещение файлов (Linux)
- Копировать файл (оставить исходник на месте):
  - cp path/to/source path/to/destination
  - Примеры:
    - cp day-1/variables_exercises.rs /home/you/projects/rust-variables-playground/src/main.rs
    - cp day-1/variables_short.rs /home/you/projects/rust-variables-playground/src/variables_short.rs
- Переместить (удаляет исходный файл, переносит его):
  - mv path/to/source path/to/destination
  - Пример:
    - mv day-1/variables.rs /home/you/projects/rust-variables-playground/src/variables_demo.rs
- Создать директорию (если нужно):
  - mkdir -p /path/to/dir

3) Быстро создать новый cargo-проект для запуска примеров
- Создать новый бинарный проект:
  - cargo new rust-variables-playground --bin
- Перейти в проект:
  - cd rust-variables-playground
- Заменить main.rs содержимым одного из наших файлов:
  - cp /path/to/repo/day-1/variables_exercises.rs src/main.rs
  - Или если хотите сохранить несколько файлов, скопируйте их в src/ и используйте модульную структуру.
- Запустить пример (main):
  - cargo run
- Запустить тесты:
  - cargo test
- Запустить один тест по имени (пример):
  - cargo test t_mutability
- Чтобы увидеть вывод println! во время тестов, добавьте флаг:
  - cargo test -- --nocapture

4) Примеры полных команд (пошагово)
- Создать проект и запустить упражнения:
  - cargo new rust-variables-playground --bin
  - cp day-1/variables_exercises.rs rust-variables-playground/src/main.rs
  - cd rust-variables-playground
  - cargo run            # запуск main (если main использует примеры)
  - cargo test           # запуск unit-тестов
- Если хотите оставить оригинальный main и просто добавить файл как модуль:
  - cp day-1/variables_exercises.rs rust-variables-playground/src/variables_exercises.rs
  - Отредактируйте src/main.rs, чтобы подключить модуль (mod variables_exercises;) и использовать его.

5) Git — как добавить изменения в репозиторий и открыть ветку
- Создать новую ветку:
  - git checkout -b day-1-variables-update
- Добавить файлы в индекс:
  - git add day-1/variables.rs day-1/variables_exercises.rs day-1/variables_short.rs day-1/README.md
  - (Если добавляли в другой проект, укажите соответствующие пути.)
- Сделать коммит:
  - git commit -m "day-1: улучшены материалы по переменным — упражнения, краткая версия и README"
- Отправить ветку в удалённый репозиторий:
  - git push origin day-1-variables-update
- Открыть Pull Request через веб-интерфейс GitHub (перейдите на страницу репозит��рия — GitHub предложит создать PR из недавно запушенной ветки).

6) Создание и применение unified diff/patch
- Создать патч с изменениями (при локальных изменениях):
  - git diff > day-1-changes.patch
- Применить патч:
  - git apply day-1-changes.patch
- Если патч создан между коммитами:
  - git format-patch <commit>..HEAD

7) Полезные команды для проверки содержимого и разрешений
- Проверить текущую директорию:
  - pwd
- Проверить права доступа:
  - stat day-1/variables.rs
  - ls -l day-1/variables.rs
- Если требуется sudo (обычно не нужно для файлов в вашем домашнем каталоге):
  - sudo cp source dest

8) Советы и примечания
- Всегда проверяйте содержимое файла после копирования:
  - head -n 20 path/to/destination
- Если вы работаете в GUI (VS Code, IntelliJ), после копирования файлов обновите окно проекта.
- Для запуска тестов используйте ту же версию Rust, что указана в проекте (rustup override set <toolchain> если нужно).
- Если хотите, я могу:
  - сгенерировать готовый patch (day-1-changes.patch) с точными изменениями — пришлю его сюда,
  - или подготовить пошаговый список команд, которые вы скопируете и выполните (скрипт .sh).

