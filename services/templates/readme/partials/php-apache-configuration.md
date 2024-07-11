### Docker-Compose Configuration

The Docker-Compose configuration is used to build and run a PHP-Apache container. This configuration provides a development environment with necessary tools and user configurations.

### Modifications Made to the PHP Image

We start with the official PHP-Apache image, adding necessary development tools and user configurations to create a flexible development environment. This includes installing Git and Docker tools, and setting up a non-root user with appropriate permissions.

Here is the Dockerfile:

```dockerfile
FROM --platform=$BUILDPLATFORM php:8.0.9-apache as builder

CMD ["apache2-foreground"]

FROM builder as dev-envs

RUN <<EOF
apt-get update
apt-get install -y --no-install-recommends git
EOF

RUN <<EOF
useradd -s /bin/bash -m vscode
groupadd docker
usermod -aG docker vscode
EOF

# Install Docker tools (cli, buildx, compose)
COPY --from=gloursdocker/docker / /

CMD ["apache2-foreground"]
```

Here is the docker compose:

```dockerfile
version: '3.8'
services:
  web:
    build:
      image: /php-apache:latest
      target: builder
    ports: 
      - '80:80'
    volumes:
      - {{ APP }}:/var/www/html/
```

This Docker-Compose configuration sets up a PHP-Apache web server in a development environment, facilitating the deployment and management of PHP applications with ease.
