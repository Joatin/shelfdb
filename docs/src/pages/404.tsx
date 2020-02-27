import React, {Component} from "react"
import Layout from "../components/Layout/Layout";

export default class NotFoundPage extends Component {

  render() {
    return (
      <Layout title="404: Not found">
        <h1>NOT FOUND</h1>
        <p>You just hit a route that doesn&#39;t exist... the sadness.</p>
      </Layout>
    )
  }
}
