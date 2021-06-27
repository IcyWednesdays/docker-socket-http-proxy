# docker-socket-http-api

A local HTTP server for proxying requests to the Docker socket. Useful in scenarios where direct connectivity to Unix sockets is not possible (e.g. frontend clients).

## Running locally

To start the HTTP server, clone this repository and run:

```bash
$ cargo run

Server listening on http://127.0.0.1:8080
```

### Via Docker

You can also build and run the server in Docker:

```bash
$ docker-compose up api
```

The server will pass through any requests directly to the Docker daemon. You can check out the list of supported endpoints [in the Docker documentation](https://docs.docker.com/engine/api/v1.24/#3-endpoints).

For example, to query all images on your system:

```bash
$ curl -s http://localhost:8080/images/json
```

## To-do

- [ ] Public Docker hub image w/ CI
