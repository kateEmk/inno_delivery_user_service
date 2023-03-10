FROM rust:1.66

WORKDIR /app
COPY ./ ./
RUN cargo install diesel_cli --no-default-features --features postgres
RUN cargo build --release

# sudo chown -R $USER /home/user/Documents/InnoDelivery/inno_delivery_user_service/data/postgres

RUN diesel migration run
EXPOSE 8080
ENTRYPOINT ["target/debug/innoDelivery_userService"]
