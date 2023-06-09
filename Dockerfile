FROM rust:latest
WORKDIR /app
COPY . /app
RUN apt install libpq-dev
RUN cargo install diesel_cli --no-default-features --features postgres 
# RUN diesel setup
# RUN diesel migration redo --all
# RUN cargo build 
EXPOSE 8080
CMD ["sh", "./diesel-setup.sh"]
