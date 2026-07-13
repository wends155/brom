# simple_blog

Minimal brom example (`Post` + `Category`).

## Run the server (required for the browser)

`ERR_CONNECTION_REFUSED` on `http://localhost:3000/admin` means **this process is not running**. Start it and **leave the terminal open**.

From the repo root:

```bash
just simple-blog-run
```

Or:

```bash
cd examples/simple_blog
cargo run -p simple-blog
```

In Cursor/VS Code: **Terminal → Run Task… → `simple-blog: run server`**.

When it is up, open:

- Admin: http://127.0.0.1:3000/admin  
- Swagger UI: http://127.0.0.1:3000/docs  

## First-time admin login

Create a user (from `examples/simple_blog`):

```bash
cargo run -p simple-blog --bin seed_admin
```

Or: **Run Task… → `simple-blog: seed admin`**.

Use the printed email/password on the admin login page.

## Database file

By default the SQLite file is **`brom.db` in the current working directory**. Run the server from `examples/simple_blog/` (or set `DATABASE_URL`) so the DB is where you expect.
