# DockyMcProxy 🚀

DockyMcProxy is a Rust-based project that simplifies the process of setting up a local HTTPS reverse proxy using `nginx`. It's a handy tool for developers who want to mimic a production environment locally. 🛠️

## Key Features 🔑

- Automatic HTTPS setup with Let's Encrypt. 🔒
- CLI for easy management. 🖥️
- Easy setup of subdomains and ports for local development. 🌐
- Built with Rust for performance and safety. 🦀
- Docker support for portability. 🐳
- Uses [traefik.me](https://traefik.me) for certificates and DNS. 🌍

## Usage 📖

The easiest way to use DockyMcProxy is to run it with Docker. You can also run it directly on your local machine using the Rust programming language. Here's how to use it with both methods.

### Running with Docker 🐳

In this example, `D_EXAMPLE=8080` sets up a subdomain `example.traefik.me` that redirects to `localhost:8080`.

```bash
docker run --rm --network=host --name "dockymcproxy" -e "D_EXAMPLE=8080" -d unit214/dockymcproxy
```

### CLI Usage 💻

When you are running DockyMcProxy on Docker, you can access the CLI by running the following command:

```bash
docker exec -it <container_id> dmp <command>
```

If you are running DockyMcProxy directly on your local machine, you can use the following command:

```bash
cargo run -- <command>
```

The application supports the following commands:

- `init`: Initializes the environment by creating new domains for each environment variable starting with `D_`.
- `list`: Lists all the existing subdomains and their corresponding ports.
- `add`: Adds a new subdomain and port. Use the `--force` or `-f` flag to overwrite an existing subdomain.
- `remove`: Removes an existing subdomain.

Here are some examples of how to use these commands:

```bash
# Add a new subdomain
cargo run -- add --subdomain example --port 8080
# or
docker exec dockymcproxy dmp add --subdomain example --port 8080

# Remove a subdomain
cargo run -- remove --subdomain example
# or
docker exec dockymcproxy dmp remove --subdomain example
```

## Development 🛠️

### Prerequisites 📋

- Rust programming language
- Cargo package manager
- Nginx web server

### Installing 📦

Clone the repository to your local machine:

```bash
git clone https://github.com/unit214/docky-mc-proxy.git
```

Navigate to the project directory:

```bash
cd docky-mc-proxy
```

Build the project:

```bash
cargo build
```

## Contribute 🤝

Your contributions are welcome! Please read [CONTRIBUTE.md](./CONTRIBUTE.md) for details on the process.

## License 📄

This project is licensed under the MIT License - see the [LICENSE](./LICENSE) file for details.