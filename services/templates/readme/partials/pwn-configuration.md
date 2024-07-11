### Docker-Compose Configuration

The Docker-Compose configuration is designed to build and run a custom Ubuntu-based container tailored for debugging and reverse engineering tasks. This setup includes essential tools such as GDB, Python, Pwntools, Pwndbg, and Radare2. It also sets up a user with a specific home directory and permissions.

### Modifications Made to the Ubuntu Image

We start with the latest Ubuntu image and install a variety of debugging and development tools. Additionally, we clone and set up Pwndbg and Radare2 for enhanced debugging capabilities. A non-root user is created, and permissions are set appropriately to ensure a secure and user-friendly environment.

Here is the Dockerfile:

```dockerfile
FROM ubuntu:latest
ARG USER
RUN apt-get update && apt-get install -y gdb \
    vim \
    python3 \
    python3-pip \
    git \
    python3-dev \
    libssl-dev \
    libffi-dev \
    build-essential \
    python3-pwntools \
    curl \
    wget
RUN git clone https://github.com/pwndbg/pwndbg && cd pwndbg && ./setup.sh
RUN git clone https://github.com/radareorg/radare2 && cd radare2 && sys/install.sh
RUN useradd $USER --home /home/$USER/ --create-home --shell /bin/bash
RUN cp /root/.gdbinit /home/$USER/.gdbinit
RUN chown -R $USER:$USER /pwndbg/
RUN chown -R $USER:$USER /home/$USER/.gdbinit
USER $USER
```

Here is the docker compose:

```dockerfile
version: '3.8'
services:
  pwnbox:
    build: 
      image: /pwn:latest
      args:
        USER: {{ PWN_USER }}
    volumes:
      - {{ PWNFILES }}:/home/{{ PWN_USER }}/pwnfiles
    tty: true
```

This Docker-Compose configuration sets up a comprehensive environment for debugging and reverse engineering, providing the necessary tools and a flexible user setup for efficient development and testing.
