FROM node:lts AS development

WORKDIR /app

#COPY $PACKAGE_JSON /app/package.json 
#COPY $PACKAGE_LOCK_JSON /app/package-lock.json 

RUN npm ci

#COPY $NGINX_CONF /app/.nginx/nginx.conf
#COPY $SRC_PATH /app
#COPY $PUBLIC_PATH /app

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
