# Running Locally

1. Run `scripts/init_db.sh`
2. Create a docker image running PGAdmin on the `nl-network` docker network:
`sudo docker run -p 80:80     -e 'PGADMIN_DEFAULT_EMAIL=user@domain.com'     -e 'PGADMIN_DEFAULT_PASSWORD=SuperSecret'   --network=nl-network   -d dpage/pgadmin4`
3. Open pgadmin buy visiting localhost:80
4. Connect to the server using `nl-database` as the host and the appropriate credentials