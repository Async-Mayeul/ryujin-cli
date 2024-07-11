FROM httpd:2.4

COPY ./conf/httpd.conf /usr/local/apache2/conf/httpd.conf 
COPY ./conf/httpd-ssl.conf /usr/local/apache2/conf/extra/httpd-ssl.conf

RUN groupadd apache && useradd -g apache apache -p password
RUN chown -R apache:apache /usr/local/apache2
USER apache
RUN touch /usr/local/apache2/logs/access.log && touch /usr/local/apache2/error/error.log
RUN find / -perm /6000 -type f -exec chmod a-s {} \; || true
