# Настройка базы данных и миграций

## 📋 Требования к базе данных

### PostgreSQL параметры подключения

Настройте параметры подключения в файле `src/config/local.yml`:

```yaml
database:
  url: "postgres://username:password@host:port/database_name?connect_timeout=5"

web:
  addr: "127.0.0.1:8090"
```

### Текущая конфигурация
Из файла `src/config/local.yml`:
- **Пользователь**: `postgres`
- **Пароль**: `1221`
- **Хост**: `localhost` (порт по умолчанию 5432)
- **База данных**: `items`
- **Timeout**: `2 секунды`
- **Сервер**: `127.0.0.1:8090`

## 🚀 Автоматическая настройка

Приложение **автоматически**:
1. **Создает базу данных** если она не существует
2. **Запускает миграции** при старте приложения
3. **Создает все необходимые таблицы и индексы**

## 📁 Структура миграций

```
migrations/
└── 20240101000001_create_items_table.sql
```

### Миграция создает:

1. **Таблицу `items`**:
   ```sql
   CREATE TABLE items (
       id SERIAL PRIMARY KEY,
       name VARCHAR(255) NOT NULL,
       description TEXT NOT NULL,
       created_at TIMESTAMPTZ DEFAULT NOW(),
       updated_at TIMESTAMPTZ DEFAULT NOW()
   );
   ```

2. **Индекс для поиска**:
   ```sql
   CREATE INDEX idx_items_name ON items(name);
   ```

3. **Автоматическое обновление `updated_at`**:
   - Функция `update_updated_at_column()`
   - Триггер `update_items_updated_at`

## ⚙️ Ручная настройка PostgreSQL

### 1. Установка PostgreSQL

**macOS (Homebrew):**
```bash
brew install postgresql
brew services start postgresql
```

**Ubuntu/Debian:**
```bash
sudo apt update
sudo apt install postgresql postgresql-contrib
sudo systemctl start postgresql
sudo systemctl enable postgresql
```

**Docker:**
```bash
docker run --name postgres-db \
  -e POSTGRES_USER=postgres \
  -e POSTGRES_PASSWORD=1221 \
  -e POSTGRES_DB=items \
  -p 5432:5432 \
  -d postgres:15
```

### 2. Создание пользователя и базы данных

```sql
-- Подключение к PostgreSQL
sudo -u postgres psql

-- Создание пользователя (если нужно)
CREATE USER postgres WITH PASSWORD '1221';

-- Создание базы данных
CREATE DATABASE items OWNER postgres;

-- Предоставление привилегий
GRANT ALL PRIVILEGES ON DATABASE items TO postgres;

-- Выход
\q
```

### 3. Проверка подключения

```bash
psql "postgres://postgres:1221@localhost/items"
```

## 🔧 Настройка для разных окружений

### Development (local.yml)
```yaml
database:
  url: "postgres://postgres:1221@localhost/items_dev?connect_timeout=5"
web:
  addr: "127.0.0.1:8090"
```

### Testing (test.yml)
```yaml
database:
  url: "postgres://postgres:1221@localhost/items_test?connect_timeout=5"
web:
  addr: "127.0.0.1:8091"
```

### Production (prod.yml)
```yaml
database:
  url: "postgres://username:password@db_host:5432/items_prod?sslmode=require&connect_timeout=10"
web:
  addr: "0.0.0.0:8080"
```

## 🛠 Команды для работы с миграциями

### Создание новой миграции
```bash
# Создайте файл в директории migrations/
touch migrations/$(date +%Y%m%d%H%M%S)_your_migration_name.sql
```

### Проверка статуса миграций
Приложение автоматически отслеживает выполненные миграции через таблицу `_sqlx_migrations`.

### Откат миграций
Для отката создайте новую миграцию с командами `DROP` или `ALTER TABLE`.

## 📊 Мониторинг базы данных

### Просмотр таблиц
```sql
-- Подключение к базе
psql "postgres://postgres:1221@localhost/items"

-- Список таблиц
\dt

-- Структура таблицы items
\d items

-- Просмотр данных
SELECT * FROM items LIMIT 10;

-- Статистика
SELECT COUNT(*) FROM items;
```

### Производительность
```sql
-- Размер таблицы
SELECT pg_size_pretty(pg_total_relation_size('items'));

-- Использование индексов
SELECT schemaname, tablename, indexname, idx_scan 
FROM pg_stat_user_indexes 
WHERE tablename = 'items';
```

## 🔒 Безопасность базы данных

### Рекомендации для продакшена:

1. **Сильный пароль**: Используйте сложный пароль для пользователя БД
2. **SSL соединение**: Добавьте `sslmode=require` в URL
3. **Ограничение доступа**: Настройте `pg_hba.conf` для ограничения подключений
4. **Отдельный пользователь**: Создайте специального пользователя с ограниченными правами
5. **Резервное копирование**: Настройте автоматическое резервное копирование

### Создание пользователя для продакшена:
```sql
-- Создание роли только для приложения
CREATE ROLE app_user WITH LOGIN PASSWORD 'strong_password_here';

-- Предоставление минимальных необходимых прав
GRANT CONNECT ON DATABASE items_prod TO app_user;
GRANT USAGE ON SCHEMA public TO app_user;
GRANT SELECT, INSERT, UPDATE, DELETE ON ALL TABLES IN SCHEMA public TO app_user;
GRANT USAGE, SELECT ON ALL SEQUENCES IN SCHEMA public TO app_user;
```

## 🚨 Troubleshooting

### Частые проблемы:

1. **Ошибка подключения**:
   ```
   Error: Connection refused (os error 61)
   ```
   - Убедитесь, что PostgreSQL запущен
   - Проверьте параметры подключения в `local.yml`

2. **Ошибка аутентификации**:
   ```
   Error: password authentication failed
   ```
   - Проверьте пароль в конфигурации
   - Убедитесь, что пользователь существует

3. **База данных не найдена**:
   ```
   Error: database "items" does not exist
   ```
   - Приложение автоматически создаст БД при первом запуске
   - Убедитесь, что у пользователя есть права на создание БД

4. **Ошибка миграций**:
   ```
   Error: migration error
   ```
   - Проверьте логи для детальной информации
   - Убедитесь, что SQL в миграции корректен

## 📝 Логи

Приложение логирует все операции с базой данных:

```bash
# Запуск с подробными логами
RUST_LOG=debug cargo run

# Логи только базы данных
RUST_LOG=sqlx=debug cargo run
```

## 🔄 Восстановление данных

### Экспорт данных:
```bash
pg_dump "postgres://postgres:1221@localhost/items" > backup.sql
```

### Импорт данных:
```bash
psql "postgres://postgres:1221@localhost/items" < backup.sql
