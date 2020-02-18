<h1 align="center">
  Shelf
</h1>
<p align="center">
  The GraphQL database. Makes storing data easy!
</p>

![Build](https://github.com/Joatin/shelf/workflows/Build/badge.svg)

## About
Shelf is a database written in rust and that uses GraphQL for it's api and migrations. Graphql makes it easy to 
describe the data resources you want to fetch. Shelf expands that power to simple let ypu describe your data and then 
have the persistence automatically generated for you

However what makes this database even more powerful is the ecosystem surrounding GraphQL. You dont need any specific 
client to access this database. All you have to do is take the GraphQL Schema and use you favourite tool to generate the 
client for you. You can JavaScript, TypeScript, Java, C#, Rust, Go and probably most other languages that are out there!

The GraphQL api is compatible with both the Apollo standard and Facebooks Relay specification. Most list resources are 
paginated using cursor based pagination


## Installation
Shelf is currently under heavy development. Once it reaches alpha it will be deployed through docker hub. As of now you 
can clone this repo and build the source locally through ```cargo run```

## License
See [LICENSE](LICENSE)