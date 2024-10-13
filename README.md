# Bitcoin lightning nodes

This project is a learning Rust HTTP server API that serves data from Bitcoin
lightning nodes.

- Podman: Version 5.2.2

## Steps to run the app

Ensure Podman is installed:

```bash
podman version
```

Access the project directory:

```bash
cd bitcoin-lightning-nodes
```

Create data directory to MySQL:

```bash
mkdir data
```

Build the server image:

```bash
podman build -t server environments/server
```

Run the application:

```bash
podman kube play environments/development.yml
```

Access the application:

The server will be available at `http://localhost:8080/nodes`.
