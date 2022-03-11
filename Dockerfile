FROM rust:latest

ARG DEBIAN_FRONTEND=noninteractive

ENV TZ=Asia/Novosibirsk

RUN apt-get update

RUN apt-get install wget -y

RUN apt-get install lsb-release -y

RUN sh -c 'echo "deb http://apt.postgresql.org/pub/repos/apt $(lsb_release -cs)-pgdg main" > /etc/apt/sources.list.d/pgdg.list'

RUN wget --quiet -O - https://www.postgresql.org/media/keys/ACCC4CF8.asc | apt-key add -

RUN apt-get install postgresql-client-13 -y

RUN apt-get install postgresql-13 -y

RUN apt-get install postgresql-contrib-13 -y

RUN apt-get install libpq-dev -y

RUN apt-get install postgresql-server-dev-13 -y

RUN cargo install ats-monitoring

CMD service postgresql start && su postgres bash -c "psql -c \"CREATE USER stepanov WITH CREATEDB PASSWORD 'postgres';\"" && ats-monitoring