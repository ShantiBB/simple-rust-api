# Simple Rust API

Высокопроизводительный REST API, написанный на Rust с использованием фреймворка Actix-web. Проект создан для продакшена с лучшими практиками безопасности и производительности.

## 🚀 Особенности

- **Actix-web** - быстрый и безопасный веб-фреймворк
- **PostgreSQL** - надежная база данных с поддержкой SQLx
- **Модульная архитектура** - четкое разделение на слои
- **Middleware для продакшена**:
  - CORS поддержка
  - Логирование запросов  
  - Сжатие ответов
  - Заголовки безопасности
  - Обработка ошибок
- **Конфигурация через YAML**
- **Структурированное логирование**

## 📁 Структура проекта

```
src/
├── main.rs              # Точка входа приложения
├── config/              # Конфигурация приложения
│   ├── mod.rs
│   ├── config.rs        # Структуры конфигурации
│   └── local.yml        # Файл конфигурации
├── models/              # Модели данных
│   ├── mod.rs
│   └── item.rs          # Модель Item
├── repository/          # Слой доступа к данным
│   ├── mod.rs
│   ├── postgres.rs      # Подключение к PostgreSQL
│   └── item.rs          # Репозиторий для Item
├── routes/              # HTTP маршруты
│   ├── mod.rs           # Конфигурация маршрутов
│   └── item_routes.rs   # Маршруты для Item
├── middleware/          # Пользовательские middleware
│   ├── mod.rs
│   ├── cors.rs          # CORS middleware
│   └── error_handler.rs # Обработка ошибок
└── handler/             # (Legacy) Старые обработчики
    ├── mod.rs
    └── item.rs
```

## 🛠 Технологии

- **Rust 2024 Edition**
- **Actix-web 4.4** - веб-фреймворк
- **SQLx 0.8** - асинхронный SQL toolkit
- **PostgreSQL** - база данных
- **Serde** - сериализация/десериализация JSON
- **Anyhow** - обработка ошибок
- **Env_logger** - логирование

## 📋 API Endpoints

### Items API

Все endpoints доступны по префиксу `/api/v1/items`

| Method | Path | Description |
|--------|------|-------------|
| GET | `/api/v1/items` | Получить все элементы |
| POST | `/api/v1/items` | Создать новый элемент |
| GET | `/api/v1/items/{id}` | Получить элемент по ID |
| PUT | `/api/v1/items/{id}` | Обновить элемент по ID |
| DELETE | `/api/v1/items/{id}` | Удалить элемент по ID |

### Примеры запросов

**Создать элемент:**
```bash
curl -X POST http://localhost:8080/api/v1/items \
  -H "Content-Type: application/json" \
  -d '{"name": "Test Item", "description": "Test Description"}'
```

**Получить все элементы:**
```bash
curl http://localhost:8080/api/v1/items
```

**Получить элемент по ID:**
```bash
curl http://localhost:8080/api/v1/items/1
```

## ⚙️ Конфигурация

Конфигурация осуществляется через файл `src/config/local.yml`:

```yaml
database:
  url: "postgresql://username:password@localhost/database_name"

web:
  addr: "127.0.0.1:8080"
```

## 🗄️ База данных

Создайте таблицу в PostgreSQL:

```sql
CREATE TABLE items (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    description TEXT NOT NULL
);
```

## 🚀 Запуск

1. **Убедитесь, что PostgreSQL запущен**

2. **Настройте конфигурацию** в `src/config/local.yml`

3. **Запустите приложение:**
   ```bash
   cargo run
   ```

4. **Для продакшена:**
   ```bash
   cargo build --release
   ./target/release/simple-rust-api
   ```

## 📊 Производительность

Проект оптимизирован для продакшена:

- **4 worker процесса** для обработки запросов
- **Сжатие ответов** для уменьшения трафика
- **Connection pooling** для базы данных
- **Эффективная обработка ошибок**
- **Структурированное логирование**

## 🔒 Безопасность

Встроенные меры безопасности:

- **CORS** настройка
- **Security headers**:
  - X-Frame-Options: DENY
  - X-Content-Type-Options: nosniff
  - X-XSS-Protection: 1; mode=block
  - Referrer-Policy: strict-origin-when-cross-origin

## 🧪 Разработка

**Проверка кода:**
```bash
cargo check
```

**Форматирование:**
```bash
cargo fmt
```

**Линтинг:**
```bash
cargo clippy
```

## 🔄 Миграция с Axum

Проект был успешно мигрирован с Axum на Actix-web с сохранением всей функциональности и улучшением архитектуры для продакшена.

## 📝 Логирование

Уровни логирования настраиваются через переменную окружения:

```bash
RUST_LOG=info cargo run        # INFO уровень
RUST_LOG=debug cargo run       # DEBUG уровень
RUST_LOG=error cargo run       # Только ошибки
```

## 🤝 Вклад в проект

1. Форкните репозиторий
2. Создайте feature ветку
3. Сделайте коммит изменений
4. Отправьте Pull Request

## 📄 Лицензия

MIT License
