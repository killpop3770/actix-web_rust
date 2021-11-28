1) docker-compose down

    docker volume ls

    docker volume prune (delete old cash)


2) docker-compose up --force-recreate -d

    docker ps 

    docker exec -it actix-web_rust_postgres_1 psql -U actix

    psql -h 127.0.0.1 -p 5432 -U actix
