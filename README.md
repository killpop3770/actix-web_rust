0) https://github.com/docker-library/postgres/issues/537


1) docker-compose down

    docker volume ls

    docker volume prune (delete old cash)


2) docker-compose up --force-recreate -d

    docker ps 

    psql -f database.sql -p 5432 -h 127.0.0.1 -U actix actix

    docker exec -it actix-web_rust_postgres_1 psql -U actix

    psql -h 127.0.0.1 -p 5432 -U actix
