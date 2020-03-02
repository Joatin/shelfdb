<h1 align="center">
  Shelf
</h1>
<h3 align="center">
  The GraphQL database. Makes storing data easy!
</h3>

[![Test](https://github.com/Joatin/shelfdb/workflows/Test/badge.svg)](https://github.com/Joatin/shelf/actions) [![Docker Pulls](https://img.shields.io/docker/pulls/joatin/shelfdb)](https://hub.docker.com/r/joatin/shelfdb) [![MicroBadger Size](https://img.shields.io/microbadger/image-size/joatin/shelfdb/nightly)](https://hub.docker.com/r/joatin/shelfdb)

#### DISCLAIMER: This project is currently in *ALPHA*

## About
Shelf is a database written in rust and that uses GraphQL for it's api and migrations. Graphql makes it easy to 
describe the data resources you want to fetch. Shelf expands that power to simple let ypu describe your data and then 
have the persistence automatically generated for you

However what makes this database even more powerful is the ecosystem surrounding GraphQL. You dont need any specific 
client to access this database. All you have to do is take the GraphQL Schema and use you favourite tool to generate the 
client for you. You can use JavaScript, TypeScript, Java, C#, Rust, Go and probably most other languages that are out there!

The GraphQL api is compatible with both the Apollo standard and Facebooks Relay specification. Most list resources are 
paginated using cursor based pagination

## Installation

#### Helm
The simplest way to add Shelfdb to you cluster is by using our helm script. Assuming you have helm installed 
([How to install](https://helm.sh/docs/intro/quickstart/)), you can simple run the following command and you will be all set

```shell script
helm install --repo https://shelfdb.netlify.com shelfdb
```

#### Docker Compose

You can also run Shelfdb locally using the following docker compose file. Really nice for some local experimentation

```yaml
version: '3'
services:
  shelfdb:
    image: "joatin/shelfdb:latest"
    ports:
      - "5600:5600"
```

## Roadmap
 - [x] Disk File Store
 - [x] GraphQL Resource Specifications
 - [ ] Working GraphQL API
 - [ ] GraphQL Migration Support
 - [ ] S3 File Store
 - [ ] GraphQL Subscriptions
 - [ ] Clustering

## License
See [LICENSE](LICENSE)
