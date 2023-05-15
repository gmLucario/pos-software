FROM rust:1.67-slim-buster as builder

RUN apt-get update
RUN apt-get -y install pkg-config
RUN apt-get -y install libssl-dev
 
RUN cargo install sqlx-cli --no-default-features --features native-tls,postgres

ARG DATABASE_URL
ENV DATABASE_URL $DATABASE_URL

WORKDIR /usr/pos-software
COPY migrations ./migrations/

CMD [ "/bin/bash" ]