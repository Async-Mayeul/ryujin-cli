### Docker-compose configuration : 
Inside the docker compose for apache2, we map the port 443 from the host with the port 443 inside the container, 
same with 8080 and 80, but it's not recommended to use only http. Next we copy the certificate file and key file for TLS 1.3 
from the host to the container, we also copy the content of the website to be served by Apache and the logs directory.
### Modification made on the Image Apache :
The original apache2 image from docker hub was modified to include an secure configuration, that follow the CIS benchmarks
for Apache2. You can modify this configuration if you want but this configuration is normally secure.
Modification made are a new non-root user to manage this container, ServerTokens, ServerSignature and Etag are disable to prevent
malicious user to retrieve informations from the server. We also have added an custom configuration of ssl (httpd-ssl.conf),
that use only TLS 1.3 and recommended ciphers algorithms.