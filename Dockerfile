FROM rust:latest
WORKDIR /app
COPY . /app
RUN apt install libpq-dev
RUN cargo build 
EXPOSE 8080
CMD ["cargo", "run"]
