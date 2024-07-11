### Docker-Compose Configuration

The Docker-Compose configuration defines multiples variables that will be use on the dockerfile. Like package.json, package-lock.json, source directory, public directory and nginx configuration file.

### Modifications Made on the Node.js Image

We start with a Node.js LTS image and configure it for development and production environments. The Dockerfile utilizes multi-stage builds to optimize the build process and minimize image size. Here's a breakdown:

#### Dockerfile

```dockerfile
FROM node:lts AS development

ARG PACKAGE_JSON
ARG PACKAGE_LOCK_JSON
ARG NGINX_CONF
ARG SRC_PATH
ARG PUBLIC_PATH

WORKDIR /app

COPY $PACKAGE_JSON /app/package.json 
COPY $PACKAGE_LOCK_JSON /app/package-lock.json 

RUN npm ci

COPY $NGINX_CONF /app/.nginx/nginx.conf
COPY $SRC_PATH /app
COPY $PUBLIC_PATH /app

ENV CI=true
ENV PORT=3000

CMD ["npm", "start"]

FROM development AS build

RUN npm run build

FROM development AS dev-envs

RUN <<EOF
apt-get update
apt-get install -y --no-install-recommends git
EOF

RUN <<EOF
useradd -s /bin/bash -m vscode
groupadd docker
usermod -aG docker vscode
EOF

COPY --from=gloursdocker/docker / /

CMD ["npm", "start"]

FROM nginx:alpine

COPY --from=build /app/.nginx/nginx.conf /etc/nginx/conf.d/default.conf

WORKDIR /usr/share/nginx/html

RUN rm -rf ./*

COPY --from=build /app/build .

ENTRYPOINT ["nginx", "-g", "daemon off;"]
```

Here is the docker compose :

```dockerfile
version: '3.8'
services:
  frontend:
    build:
      image: /react-nginx:latest
      args:
        PACKAGE_JSON: {{ PACKAGE_JSON }}
        PACKAGE_LOCK_JSON: {{ PACKAGE_LOCK_JSON }}
        NGINX_CONF: {{ NGINX_CONF }}
        SRC_PATH: {{ SRC_PATH }}
        PUBLIC_PATH: {{ PUBLIC_PATH }}
    container_name: {{ REACT_CONTAINER_NAME }}
    ports:
      - "80:80"
```

This Docker-Compose configuration sets up a comprehensive environment for developing, building, and serving a Node.js frontend application with Nginx as a reverse proxy. It facilitates efficient development practices and ensures consistent deployment across different environments.
