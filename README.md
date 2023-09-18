# RSS feed reader in Rust

This is a simple RSS feed reader written in Rust. It is a learning project for me to get to know the language and its ecosystem.

## Features

Reads a list of RSS feeds and keywords from `feeds.txt` and `keywords.txt` respectively. Then it fetches the feeds and prints the entries that contain any of the keywords.

## Usage

### Run locally (requires Rust)

```bash
# install rust
curl https://sh.rustup.rs -sSf | sh
# build and run
cargo build
RUST_LOG=info cargo run
open "http://localhost:3030"
```

### Run inside a Docker container

```bash
docker pull jkarenko/rss-reader-rust-amd64:latest
docker run -p 3030:3030 jkarenko/rss-reader-rust-amd64:latest
```

### Run as a service in AWS ECS

#### Build and push image to ECR

```bash
# build image
docker build --platform linux/amd64 --rm -t rss-reader-rust .
# login to ECR
aws ecr get-login-password --region [aws-region] | docker login --username AWS --password-stdin [aws-account-id].dkr.ecr.[aws-region].amazonaws.com
# create repository
aws ecr create-repository --repository-name rss-reader-rust
# tag image
docker tag rss-reader-rust:latest [aws-account-id].dkr.ecr.[aws-region].amazonaws.com/rss-reader-rust:latest
# push image
docker push [aws-account-id].dkr.ecr.[aws-region].amazonaws.com/rss-reader-rust:latest
```

#### Create ECS cluster and deploy service

Go to AWS Console -> ECS -> something, something, profit
