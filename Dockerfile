FROM dokken/ubuntu-22.04

RUN apt-get update -y
RUN apt-get upgrade -y
RUN apt-get install cmake -y
RUN apt-get install build-essential -y
RUN apt-get install cargo -y

RUN mkdir /app
COPY . /app

WORKDIR /app

RUN cargo build

CMD ["cargo", "run", "--color=always", "--bin", "app"]