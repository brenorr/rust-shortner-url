FROM rust:1.70-alpine3.18

LABEL maintaner="Breno R. <br3eno14@gmail.com>"

WORKDIR /app

##Project dependencies
RUN apk update
RUN apk add --no-cache curl libcurl musl-dev


#Add project files to build
ADD ./src ./src
ADD ./Cargo.toml ./
ADD ./Cargo.lock ./

#Build command
RUN cargo build -r --bin shortner-url
RUN cp ./target/release/shortner-url ./shortner-url
RUN rm -rf ./src ./Cargo.toml ./Cargo.lock ./target

CMD ["./shortner-url"]

EXPOSE 8080
