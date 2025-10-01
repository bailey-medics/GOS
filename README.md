# GOS

A play with GitEHR openEHR in server form - using rust only

## Time trial

```bash
brew install hyperfine
brew install postgresql@16
brew services start postgresql@16
PGURL="postgres://user:pass@localhost:5432/postgres" N=10000 ./file_db_time_trial.sh
createuser -s postgres || true
psql -U postgres -c "ALTER USER postgres WITH PASSWORD 'postgres';" || true
```
