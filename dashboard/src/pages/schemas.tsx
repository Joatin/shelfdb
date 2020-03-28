import React, {Component} from "react"

import Layout from "../components/Layout/Layout";
import client from "../client";
import {gql} from "apollo-boost";


export default class SchemasPage extends Component {

  componentDidMount(): void {
    client.query({
      query: gql`
        {
          rates(currency: "USD") {
            currency
          }
        }
      `
    })
    .then(result => console.log(result));
  }

  render() {
    return (
      <Layout title="Shelfdb">
        SCHEMAS
      </Layout>
    )
  }
}
