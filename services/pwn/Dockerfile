FROM ubuntu:latest
ARG $USER
RUN apt-get update && apt-get install -y gdb \
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
