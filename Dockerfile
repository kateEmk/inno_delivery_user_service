FROM rust:1.66

WORKDIR /app
COPY ./ ./
RUN cargo install diesel_cli --no-default-features --features postgres
RUN cargo build --release

#CMD ["/bin/diesel", "diesel"]
#
RUN diesel migration run
EXPOSE 8080
ENTRYPOINT ["target/debug/innoDelivery_userService"]
