# Ryujin-CLI

![Ryujin logo](./docs/img/logo-ryujin.png "Ryujin logo"){width=150 height=150}

Ryujin-cli is a Rust-based CLI application designed to simplify and secure Docker deployments. Acting as a Docker-compose generator, it offers a curated catalog of preconfigured services aimed at enhancing security. With its step-by-step configuration guidance, Ryujin-cli ensures users can customize their Docker-compose files efficiently while learning about best security practices. By making deployment easier, more efficient, and more secure, Ryujin not only facilitates Docker usage but also sensitizes users to the importance of proper configuration for optimal efficiency and security.

Our services come preconfigured to the max, especially on the security front, to give you the best experience right out of the box. Plus, some Docker images are customized before they're added to our catalog.

All the modifications we make to these images are thoroughly documented in the README provided with each docker-compose file. This way, you can easily see what changes were made and why. It ensures full transparency and allows you to replicate or tweak our configurations to suit your specific needs.


## Table of content

[[_TOC_]]


## Getting started
### Installation
you need to install Rust, Docker and Git before install our tool and use it:
#### Git
Update package list
```sh
sudo apt-get update
```
Download git
```sh
sudo apt-get install git
```
#### Docker
Download the installation script
```sh
 curl -fsSL https://get.docker.com -o get-docker.sh
```
Run the script
```sh
sudo sh get-docker.sh
```
Start Docker
```sh
sudo systemctl start docker
```
#### Rust
Install Rust :
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
After installation, you need to reload your shell and run the following command to check that rust is correctly installed :
```sh
rustc --version
```

#### Linux and MacOs
To install our application, simply enter this command :
```sh
curl -o linux_install_script.sh -L https://gitlab.com/ryujingroup/ryujin-cli/-/raw/main/build/linux_install_script.sh; sudo bash linux_install_script.sh
```
After installation, reload your shell.
#### Windows
> :warning: **Warning** 
> Unfortunately, the installation script for Windows is not yet available. Please refer to the following installation procedure.

> :bulb: **Requirements** 
> To install our tool on windows, you must first install :
> - Docker
> - Rustup
> - git

Then download the project from our gitlab page :
```sh
git clone https://gitlab.com/ryujingroup/ryujin-cli.git
```
And then compile the application :
```sh
cargo build --release
```
Move ryujin-cli at the base of dir
```sh
move ./target/debug/ryujin-cli .
```
Add ryujin-cli executable to the PATH
```sh
set PATH=%PATH%;C:\your\path\here\ryujin-cli
```

### Running commands
Check available services
```sh
ryujin-cli catalog
```
add the services you want to a new selection
```sh
ryujin-cli select -n -s service1,service2
```
generate the docker compose with the services you've added to the selection
```sh
ryujin-cli compose -o ~/my_folder
```

After these commands, your docker compose is generated and you can find it in the specified folder, named **docker-compose.yml**.
```sh
my_folder/
├── docker-compose.yml
├── readme-compose.md
```

## Ryujin-cli usage
### The catalog command : Displays the list of services you can add to your docker-compose.
#### Synopsis
```sh
ryujin-cli catalog [OPTIONS]
```
#### Description
This command displays the services available to add to your docker compose. 
Various options are available to refine the search or obtain more information.
#### Options
```sh
-l, --long   Display detailed information about the services.
```
```sh
-t, --tags <tags>   Filter services by tags.
```
```sh
-n, --name <name>   Filter services by name.
```
```sh
-h, --help   Print help
```
#### Examples
Display the names of available services :
```sh
ryujin-cli catalog
```
To view detailed information about services :
```sh
ryujin-cli catalog -l
```
Filter by database services :
```sh
ryujin-cli catalog -t db
```
Filter by web server services and print detailed information :
```sh
ryujin-cli catalog -l -t websrv
```
Filter by the name of the service and print detailed information :
```sh
ryujin-cli catalog -l -n apache
```
### The service command : Displays detailed information about a specific service.
#### Synopsis 
```sh
ryujin-cli service <service_name>
```
#### Description
Displays detailed information about a specific service.
#### Arguments 
```sh
<service_name>  The name of the service to display.
```
#### Examples
Get information about a service :
```sh
ryujin-cli service apache
```
### The select command : The select command allows you to save a service selection and update it before using the compose command.
#### Synopsis
```sh
ryujin-cli select [OPTIONS]
```
#### Description
The select command allows you to save a service selection and update it before using the compose command.

#### Options
```sh
-n, --new   Create a new selection.
```
- Cannot be used with the option `-r`.
- Erase the previous selection.
```sh
-a, --add   Add some services to the current selection.
```
- The `-s` option needs to be provided for this option to be accepted.
- Cannot be used with the option `-r`.
```sh
-d, --delete   Delete the current selection.
```
- No other options can be used at the same time.
```sh
 -r, --remove   Remove some services from the current selection.
```
- The `-s` option needs to pe provided for this option to be accepted.
- Cannot be used with the options `-a` and `-n`.
```sh
-p, --print   Print the current selection.
```
- If used with other options, it shall print the selection after the actions performed by the other options.
```sh
-s, --services <services>   The service you want.
```
- The list services shall be specified separated with comas `(, )`
- Used with options: `-a`, `-r`, `-n`
```sh
-h, --help   Print help
```
#### Examples
Create a new selection (erase the previous) :
```sh
ryujin-cli select -n
```
Create a new selection with new specified service : 
```sh
ryujin-cli select -n -s service
```
Add new services in the current selection :
```sh
ryujin-cli select -a -s service1,service2,service3
```
Remove two services from the current selection and print the selection after removing :
```sh
ryujin-cli select -r -s service2,service3 -p 
```
Remove all the services from the current list :
```sh
ryujin-cli select -d
```
### The compose command : Start the process of creating a docker-compose. 
#### Synopsis 
```sh
ryujin-cli compose [OPTIONS...] --output-dir <output-dir>
```
#### Description 
Start the process of creating a docker-compose.

To create the docker-compose, you'll be asked different questions to configure it. 

After creation, you have have many explication about your new docker-compose configuration :
- How to use it. 
- Informations about the service.
- Security things.

You can also find how to use docker-compose in this wiki.

#### Options 
```sh
-s, --services <services>   List of services to add to the docker-compose.
```
- You need to choose one or more services to add to your docker-compose with this option.
```sh
-o, --output-dir <output-dir>   Path of the output directory where the docker-compose should be created.
```
- if a folder does not exist in the path, it will be created automatically. 
```sh
-h, --help   Print help
```
#### Examples 
Create docker-compose with one service :
```sh
ryujin-cli compose -s service -o ~/folder
```
Create docker-compose with two services :
```sh
ryujin-cli compose -s service1,service2 -o ~/folder
```
Create docker-compose with the services specified in the current selection :
```sh
ryujin-cli compose -o ~/folder
```
#### Questions examples for docker-compose configuration 
##### A docker-compose for a web server 

If a docker-compose file already exists in the specified path, you will get this question:
```
There is already a file named docker-compose.yml in /home/user/. Do you want to erase it? (y/n)
```
Now let's start with the configuration questions :
```
Enter the path where the certificate is stored. The certificate enable communication encryption and authenticate website's identity. The certificate will be copy from your host inside docker container in /etc/nginx/ssl/certs/.

>>> /etc/ssl/certs/server.crt

Enter the path where the private key is stored. The private key is used to decrypt message between the server and client that have been encrypted by the public key in the certificate. The private key will be copy from your host inside docker container in /etc/nginx/ssl/private/. The private key need to be private and not shared because it's an critical asset that permit to authenticate the web server.

>>> /etc/ssl/private/server.key

Enter the path where the content of your web site is stored. The source code of your website

>>> /var/www/html/
```

**Result of the docker-compose creation :**
```sh
version: '3'

services:
  nginx:
    build:
      image: /nginx-ryujin:latest
      args:
        CERTIFICATE_PATH: /etc/ssl/certs/server.crt
        PRIVATE_KEY_PATH: /etc/ssl/private/server.key
    volumes:
      /var/www/html/:/usr/share/nginx/html/
  
    ports:
        8080:80
        443:443

```
### How to use Docker Compose 
#### What is Docker Compose 
Docker compose is used to define and run multi-container application. Compose can control your entier stack.
You can manage services, networks, volumes with a single YAML file.

Compose works in all environments; production, staging, development, testing, as well as CI workflows. It also has commands for managing the whole lifecycle of your application:
- Start, stop, and rebuild services
- View the status of running services
- Stream the log output of running services
- Run a one-off command on a service
#### Run Docker Compose 
- In your project directory, start your application :
```sh
docker-compose up
```
- Down your application :
```sh
docker-compose down
```
- List local images :
```sh
docker image ls
```
- Inspect an image :
```sh
docker inspect <id>
```
- Run your application in background :
```sh
docker-compose up -d
```
#### Docker-Compose build
```sh
docker-compose build
```

If the docker doesn't build, try with sudo. 
## About us 

Welcome to our project page! We are Raphaël, Julien, and Mayeul, three passionate computer science students.

As part of our annual mission with Oteria Cyber School, we are collaborating to develop this secure docker compose tool. Each of us brings unique skills in development, organization, and creativity to ensure the success of this project.

Thank you for visiting, and see you soon!


