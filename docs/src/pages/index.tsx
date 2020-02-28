import React, {Component} from "react"

import Layout from "../components/Layout/Layout";
import Hero from "../components/Hero/Hero";
import UndrawSection from "../components/UndrawSection";
import UndrawCollecting from "react-undraw/dist/illustrations/UndrawCollecting";
import UndrawLighthouse from "react-undraw/dist/illustrations/UndrawLighthouse";


export default class IndexPage extends Component {

  render() {
    return (
      <Layout title="Home">
        <Hero />
        <UndrawSection color={'white'} undraw={UndrawCollecting}>
          <h1>The power of GraphQL</h1>
          <p>Shelf is built around the concept of GraphQL. You define your resources with GraphQL and that is also the way you query it.</p>
        </UndrawSection>
        <UndrawSection color={'lightgray'} undraw={UndrawLighthouse} right={true}>
          <h1>The Ultimate Developer Experience</h1>
          <p>Shelf is super easy to get started with. You just define your resources and everything works. Then you can use the GraphQL schema to generate a client with whatever tool you want for any language you want</p>
        </UndrawSection>
      </Layout>
    )
  }
}
