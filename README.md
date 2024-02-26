# focus
### Setup instructions
1. Create `.env`:
```
POSTGRES_HOST=127.0.0.1
POSTGRES_PORT=5432
POSTGRES_USERNAME=postgres
POSTGRES_PASSWORD=mysecretpassword
POSTGRES_DBNAME=focus
DATABASE_URL=postgresql://postgres:mysecretpassword@localhost:5432/focus

DISCORD_TOKEN=MTIxMTc5MjQyODYyMTYyNzQyMw.xxx.xxx
FOCUS_ROLE_ID=123123123123123123
GUILD_ID=123123123123123123
```
2. `docker-compose up`
3. `cargo run`
