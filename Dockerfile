FROM rust:slim
LABEL authors="keno"

# Install Nginx
RUN apt-get update && apt-get install -y nginx curl

# Get HTTPS
RUN mkdir -p /etc/nginx/ssl
RUN curl -o /etc/nginx/ssl/cert.pem https://traefik.me/cert.pem
RUN curl -o /etc/nginx/ssl/key.pem https://traefik.me/privkey.pem

# Copy your Rust project into the Docker image
COPY . /usr/src/docky-mc-proxy

# Set the working directory
WORKDIR /usr/src/docky-mc-proxy

# Build your Rust project
RUN cargo build --release

# Setup alias
RUN ln -s /usr/src/docky-mc-proxy/target/release/docky-mc-proxy /usr/bin/dmp

# Expose the ports you want to use
EXPOSE 80 443

# Set the entrypoint to the startup script
ENTRYPOINT ["./start.sh"]

# Start an interactive bash shell by default
CMD ["bash"]