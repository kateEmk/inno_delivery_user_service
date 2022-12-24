FROM rust:latest

WORKDIR /app
COPY ./ ./
RUN cargo build --release

#CMD ./entrypoint.sh
ENTRYPOINT ["sh", "app/entrypoint.sh"]