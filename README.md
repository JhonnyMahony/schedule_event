# Yew Rust Додаток з Tailwind CSS

Цей проєкт — це веб-додаток та телеграм бот, створений за допомогою [Yew](https://yew.rs/), фреймворку на Rust для створення фронтенд веб-додатків, та [Tailwind CSS](https://tailwindcss.com/), CSS фреймворку з утилітарним підходом для стилізації. Бот був створений за допомогою С# (Telegram.bot).

## Особливості

- **Yew Фреймворк**: Використання мови Rust для фронтенду, що забезпечує типобезпечність та продуктивність.
- **Tailwind CSS**: Гнучкий CSS фреймворк з утилітарним підходом для швидкого створення адаптивних інтерфейсів.

## Встановлення

Щоб почати роботу, переконайтеся, що у вас встановлено:

- [Rust](https://www.rust-lang.org/tools/install)
- [Trunk](https://trunkrs.dev/#install) для збору додатка
- [Tailwind CSS](https://tailwindcss.com/docs/installation) для стилізації

### 1. Клонування репозиторію:

```bash
git clone https://github.com/JhonnyMahony/schedule_event
cd schedule_event/mini_app
```
### 2. Встановлення залежностей mini_app:

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk
npm install -D tailwindcss
npx tailwindcss init
```

### 3. Встановлення залежностей mini_app:

```bash
dotnet add package telegram.bot 
dotnet add package system.data.sqlite
```

### 4. Збірка та запуск додатка mini_app:

```bash
trunk serve
```

### 5. Збірка та запуск додатка TelegramBot:

```bash
dotnet run
```


