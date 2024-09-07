# Yew Rust Додаток з Tailwind CSS

Цей проєкт — це веб-додаток, створений за допомогою [Yew](https://yew.rs/), фреймворку на Rust для створення фронтенд веб-додатків, та [Tailwind CSS](https://tailwindcss.com/), CSS фреймворку з утилітарним підходом для стилізації.

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
git clone https://github.com/yourusername/your-repo.git
cd your-repo
```
### 2. Встановлення залежностей:

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk
npm install -D tailwindcss
npx tailwindcss init
```

### 3. Збірка та запуск додатка:

```bash
trunk serve
```
