FROM nginxinc/nginx-unprivileged
USER root

COPY ./ryujin.conf /etc/nginx/conf.d/ryujin.conf
RUN mkdir /etc/nginx/ssl && mkdir /etc/nginx/ssl/private && mkdir /etc/nginx/ssl/certs
RUN touch /etc/nginx/ssl/private/nginx.key && touch /etc/nginx/ssl/certs/nginx.crt
RUN chown nginx:nginx /etc/nginx/ssl/private/nginx.key /etc/nginx/ssl/certs/nginx.crt && \
  chmod 400 /etc/nginx/ssl/private/nginx.key && \
  chmod 644 /etc/nginx/ssl/certs/nginx.crt

USER nginx
