# Welcome
Welcome to Dragon Bot Z's Character service. This service is responsible for storing, retrieving and managing the Character Database.

# How to interact with the service
The service enables interactions via HTTP, you can interact with it by calling the routes corresponding to your needs.

# How to build
To build the service and its database, run the following command:
```bash
$ source build.sh
```

# How to run
Once you've build the service, you can run it by using this command:
```bash
$ docker compose up
```
You may encounter Database errors, please refer to the next section to fix them.

# How to setup the database?
Basically the database volume (dbz-character-database-volume) should be added to your docker volumes settings.

If there is an error like "Unbale to find postgresql.conf", just run:
```bash
$ docker compose up
```
Then run 
```bash
$ docker ps
```
And, retrieve the id of the container named `dbz-character-service-db` and run:
```bash
$ docker exec -it <container id> bash

# Inside the container
$ rm -r /var/lib/postgresql/data/*
$ initdb -D /var/lib/postgresql/data
$ exit
```
Finally, shut down `docker compose` and run it again, then:
```bash
$ docker ps
$ docker exec -it <container id> bash

# Inside the container 
$ psql -f init_database.sql
```
