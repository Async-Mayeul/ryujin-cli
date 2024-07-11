### Docker-Compose Configuration:

The Docker-Compose configuration is used to provide your own SSL certificate (private key and certificate). You also provide the sources for your website.

### Modifications Made to the Nginx Image:

We use the unprivileged Nginx Docker image to run a container without root rights. In the Dockerfile, we map a hardened configuration we created (ryujin.conf) inside the container. This hardened configuration follows best practices from the CIS benchmark to secure an HTTP/HTTPS server. We also apply the least privileges necessary to the private key and certificate files to prevent malicious access.

Here is the Dockerfile:

```dockerfile
FROM nginxinc/nginx-unprivileged
USER root

ARG CERTIFICATE_PATH
ARG PRIVATE_KEY_PATH

COPY ./ryujin.conf /etc/nginx/conf.d/ryujin.conf
COPY $CERTIFICATE_PATH /etc/nginx/ssl/certs/nginx.crt
COPY $PRIVATE_KEY_PATH /etc/nginx/ssl/private/nginx.key

RUN chown nginx:nginx /etc/nginx/ssl/private/nginx.key /etc/nginx/ssl/certs/nginx.crt && \
  chmod 400 /etc/nginx/ssl/private/nginx.key && \
  chmod 644 /etc/nginx/ssl/certs/nginx.crt

USER nginx
```

Here is the docker compose:

```dockerfile
version: '3.8'
services:
  nginx:
    build:
      image: /nginx-ryujin:latest
      args:
        CERTIFICATE_PATH: {{ CERTIFICATE_PATH }}
        PRIVATE_KEY_PATH: {{ PRIVATE_KEY_PATH }}
    volumes:
      - {{ NGINX_CONTENT }}:/usr/share/nginx/html/
    ports:
      - 8080:80
      - 443:443
```

This Docker-Compose configuration allows you to securely deploy your Nginx web server while providing a straightforward method to use a custom SSL certificate.
