# INNO-DELIVERY SERVICE (pt.1):

### Run with Swagger:
`docker run -p 80:8080 swaggerapi/swagger-editor`

### Run with cargo-watch:
`cargo watch -q -c -w src/ -x run`


## Used dependencies:

- Web-framework: **Actix-web**
- ORM: **Diesel** 
- Work with data: **serde**, **serde-json**
- Database: **PostgreSQL**
- Loggers: **env-logger**