# Quests Tracker

## How to Set Up

1. **Set Up Diesel**:

```sh
$ cargo install diesel_cli --no-default-features --features postgres
```

2. **Set Up PostgreSQL on Podman or Docker**:

```sh
$ podman compose up -d
or
$ docker compose up -d
```

3. **Run Migration Generate**

```sh
$ diesel migration generate init
```

4. **Run Migrations**:

```sh
$ diesel migration run
```

4. **Run the Server**:

```sh
$ cargo run
```

### 🙌 Credit
This project is part of the course `Rust Mastery Saga` by [Rayato159](https://github.com/Rayato159) on Udemy.

🔗 [Course Link](https://www.udemy.com/course/rust-mastery-saga-backend-rust/)
