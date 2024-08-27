<h1 align="center">Bit box</h1>
<p align="center">
  <img src="docs/bitbox-logo.png" width="200" alt="logo"/>
</p>

## Description

Bitbox is a SaaS aiming to handle and automate few tasks of the end project cycle for student projects : The 360 Evaluation.
<br>It can be used by teachers to manage the evaluation of students by their peers, by the teachers themselves and by the students to evaluate their peers.
It permits the teachers to register classes, students, projects and their groups. Then, when the time comes, everyone receives an email to evaluate the projects they are assigned to.
If they don't do it, they receive reminders till the deadline. Same for the professors who need to register groups' marks.
<br>When the deadline is reached, the teachers can see the results and the calculated marks of the students peers evaluations and the project.
<br><br>This repository is composed of 2 parts :
- The `API` part, which is the backend of the software, written in `Rust` with the `Actix` framework in project root.
- The `Frontend` part, which is the frontend of the software, written in `NextJS` and accessible through `web` folder.

## Made with

[![rust][rust]][rust-url]
[![actix][actix]][actix-url]
[![diesel][diesel]][diesel-url]
[![postgres][postgres]][postgres-url]
[![docker][docker]][docker-url]
[![swagger][swagger]][swagger-url]

[![nodejs][nodejs]][nodejs-url]
[![npm][npm]][npm-url]
[![nextjs][nextjs]][nextjs-url]
[![typescript][typescript]][typescript-url]

## License

[![MIT License][license-shield]](LICENSE)

## Table of Contents

1. [Prerequisites](#prerequisites)
2. [Installation](#installation)
3. [Run](#run)
4. [Usage](#usage)
5. [Development](#dev)
7. [Tests](#tests)
8. [Contributing](#contributing)

<h2><a name="prerequisites"></a>1. Prerequisites</h2>

Before trying to deploy `Dockdock Go API`, you will need few things to make it work.

<h3><u>Docker</u></h3>

The service being a docker image, you indeed need `Docker` to pull it.
You can follow the instructions **[here](https://docs.docker.com/engine/install/)** to install Docker on the system where this service will be deployed.

<summary><h3><u>PostgreSQL Database</u></h3></summary>

`Bitbox` is using a `PostgreSQL` database to store all the data of the software. If you want to use your own, please create a database named <u>**bitbox**</u>

The `docker-compose` file template contains a database service, but you can install it from your own, on bare-metal, with a docker image or a kubernetes pod.
<u>The choice is yours</u>.

<h2><a name="installation"></a>2. Installation</h2>

To install it, you only need to clone the repository :
```sh
git clone https://github.com/brandy223/bitbox-online-notation.git
````

<h2><a name="run"></a>3. Run</h2>

<h3><u>With Docker</u></h3>

IF you want to run the service with docker, you can use the docker-compose file provided in the repository.

First, you'll need to adapt the `docker-compose.yml`:

<details>
<summary><h4>Docker Compose Template</h4></summary>

```yml
version: '3.8'

services:
  postgres:
    image: postgres:latest
    container_name: bitbox-db
    environment:
      POSTGRES_USER: user
      POSTGRES_PASSWORD: password
      POSTGRES_DB: bitbox
    ports:
      - "5433:5432"
    volumes:
      - postgres_data:/var/lib/postgresql/data

  bitbox-api:
    depends_on:
      - postgres
    image: bixbox-back:1.0.0
    container_name: bixbox-api
    ports:
      - "8080:8080"
    environment:
      DATABASE_URL: postgresql://user:password@localhost:5433/notation
      # Tokens
      JWT_SECRET: secret
      JWT_EXPIRES_IN: 24
      JWT_PASSWORD_RESET_EXPIRES_IN: 10
      # SMTP
      SMTP_HOST: smtp.gmail.com
      SMTP_PORT: port
      SMTP_USERNAME: username
      SMTP_PASSWORD: Thisispassword
      # API Admin
      ADMIN_EMAIL: contact@gmail.com
      DEFAULT_ADMIN_PASSWORD: admin
      ADMIN_PASSWORD: Thisisadminpassword
      # React app URL
      WEB_URL: http://localhost:3000
    entrypoint: ["/usr/local/bin/wait-for-it.sh", "localhost:5433", "--", "sh", "-c", "diesel migration run && main"]

  bitbox-front:
    image: bixbox-front:1.0.0
    container_name: bixbox-front
    network_mode: "bridge"
    ports:
      - "3000:3000"
    environment:
      NEXT_PUBLIC_API_URL: http://localhost:8080/api

volumes:
  postgres_data:
```
</details>

<details>
<summary><h4>Lexicon</h4></summary>

| Environment variables    | Description  |
| ------------ | ------------ |
| `DATABASE_URL` | URL to connect to database containing database address and port with the needed credentials |
| `JWT_SECRET` | Secret key to sign JWT tokens |
| `JWT_EXPIRES_IN` | Time in hours before a JWT token expires |
| `JWT_PASSWORD_RESET_EXPIRES_IN` | Time in minutes before a password reset token expires |
| `SMTP_HOST` | SMTP server host |
| `SMTP_PORT` | SMTP server port |
| `SMTP_USERNAME` | SMTP server username |
| `SMTP_PASSWORD` | SMTP server password |
| `ADMIN_EMAIL` | Email of the admin user |
| `DEFAULT_ADMIN_PASSWORD` | Default password for the admin user |
| `ADMIN_PASSWORD` | Password for the admin user |
| `WEB_URL` | URL of the React app |
</details>

Then, to build the 2 docker images, you can run :
```shell
# At the project root
docker build --rm=true --tag=bixbox-back:1.0.0 -f Dockerfile . 
docker build --rm=true --tag=bixbox-front:1.0.0 -f web/Dockerfile .
```

Finally, to run the service, you can run :
```shell
# At the project root
docker compose up -d # "-d" for detached, to avoid attaching the container to the current shell instance
```

<h3><u>Without Docker</u></h3>

If you want to run the service without docker, you can run the following commands :
```shell
# At the project root
cargo run --package api --bin main
```

Then, to run the frontend, you can run :
```shell
# In the web folfder
npm install # To install the dependencies
npm run build # To build the project
npm run start # To start the built project
```

<h2><a name="usage"></a>4. Usage</h2>

The `Swagger documentation` is accessible at : `http://{bitbox-api_host_ip}:{bitbox-api_host_port}/swagger-ui/#/`.

> [!NOTE]
> The `Swagger` documentation needs authentication to make requests. You can copy the cookie from the browser and paste it in the `Swagger` interface to be able to make requests.

<h2><a name="dev"></a>5. Development</h2>

### Prerequisites

If you want to bring modifications to this project, you'll first have some prerequisites to complete before accessing the code :
- ### [PosgreSQL Database](#prerequisites)
- ### Packages
Few packages are needed to build and run the project, so make sure you have these installed : `pkg-config`, `libssl-dev`, `libpq-dev`, `openssl`, `curl`
<u>Example :</u>
```sh
sudo apt update & sudo apt install -y pkg-config libssl-dev libpq-dev openssl curl & sudo apt clean
```
- ### [Cargo](https://www.rust-lang.org/tools/install)
To build and run **rust** projects, you will neeed cargo installed. To install it, you can run this command :
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
- ### [Diesel CLI](https://diesel.rs/guides/getting-started.html)
To manipulate the database and generate migrations or apply modified ones, you will need `Diesel CLI`. Because we use only `postgres` here, you can simply install it with :
```sh
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/diesel-rs/diesel/releases/download/v2.2.0/diesel_cli-installer.sh | sh
```
- ### [NodeJS](https://nodejs.org/en/download/)

### Installation

<u>After cloning the repo, before trying to run it, make these 2 steps :</u>
- Make a `.env` file at the project root to store the environment variables you'll need which can be retrieve with `docker-compose.template.yaml` <u>**and**</u> the `Dockerfile` *(Don't forget the 3 environment variables in there or it won't work)*.
- Make sure to run the `Diesel` migrations to ensure that your database is compliant with the current `Diesel` schema. To do so :
  ```sh
  cd infrastructure
  diesel run migration
  # or if you want to reset it by the same time
  diesel database reset # which will reset and run all the migrations too
  ```

### Run

Finally, to run the API, enter this command :
```sh
cargo run --package api --bin main
``` 
Then, to run the frontend, you can run :
```sh
# In the web folfder
npm install # To install the dependencies
npm run dev # To start the development server
```

<h2><a name="tests"></a>6. Tests</h2>

`Bitbox API` implements `Unit Testing` on all database & dependencies functions.
You can run the test with the following command :
```sh
# We assume here that you already have the development configured environment
# You need to be a the root of the repository
cd application
cargo test
```

This will trigger the 53 current tests and test mostly all the database functions contained in `application` folder more or less individually.

`Integration` and `End-to-End` testing are yet to be implemented.

<h2><a name="contributing"></a>7. Contributing</h2>

Pull requests are welcome. For major changes, please open an issue first
to discuss what you would like to change.

<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->
[license-shield]: https://img.shields.io/github/license/Ileriayo/markdown-badges?style=for-the-badge
[rust]: https://img.shields.io/badge/rust-black?style=for-the-badge&logo=rust&badgeColor=010101&logoColor=white
[rust-url]: https://www.rust-lang.org/
[actix]: https://img.shields.io/badge/actix_web-033f3b?style=for-the-badge&logo=actix&badgeColor=010101&logoColor=white
[actix-url]: https://actix.rs/
[diesel]: https://img.shields.io/badge/diesel-orm-red?style=for-the-badge&labelColor=9ba09d&badgeColor=010101&logoColor=white
[diesel-url]: https://diesel.rs/
[postgres]: https://img.shields.io/badge/postgres-4169E1?style=for-the-badge&badgeColor=010101&logoColor=white&logo=postgresql
[postgres-url]: https://www.postgresql.org/
[docker]: https://img.shields.io/badge/docker-2496ED?style=for-the-badge&badgeColor=010101&logoColor=white&logo=docker
[docker-url]: https://www.docker.com/
[swagger]: https://img.shields.io/badge/swagger-85EA2D?style=for-the-badge&badgeColor=010101&logoColor=black&logo=swagger
[swagger-url]: https://swagger.io/
[nextjs]: https://img.shields.io/badge/nextjs-000000?style=for-the-badge&badgeColor=010101&logoColor=white&logo=next.js
[nextjs-url]: https://nextjs.org/
[nodejs]: https://img.shields.io/badge/nodejs-339933?style=for-the-badge&badgeColor=010101&logoColor=white&logo=node.js
[nodejs-url]: https://nodejs.org/en/download/
[npm]: https://img.shields.io/badge/npm-CB3837?style=for-the-badge&badgeColor=010101&logoColor=white&logo=npm
[npm-url]: https://www.npmjs.com/
[typescript]: https://img.shields.io/badge/typescript-3178C6?style=for-the-badge&badgeColor=010101&logoColor=white&logo=typescript
[typescript-url]: https://www.typescriptlang.org/

<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->
[license-shield]: https://img.shields.io/github/license/Ileriayo/markdown-badges?style=for-the-badge