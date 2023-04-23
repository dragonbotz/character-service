# How to setup the database?
Basically the database volume (dbz-character-database-volume) should be added to your docker volumes settings.

If there is an error like "Unbale to find postgresql.conf", just run the docker compose, docker exec into the database container, `rm -r /var/lib/postgresql/data/*` and run `initdb -D /var/lib/postgresql/data`, then `docker compose down` and `up`, docker exec again inside the database container and run `psql -f init_database.sql`
