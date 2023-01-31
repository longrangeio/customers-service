FROM rust:1.67

RUN mkdir /app
COPY . /app

WORKDIR /app

RUN cargo install --path .

CMD ["customers-service"]