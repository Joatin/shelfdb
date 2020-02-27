import React, {Component} from "react"

import Image from "../components/image"
import Layout from "../components/Layout/Layout";
import Hero from "../components/Hero/Hero";
import Section from "../components/Section/Section";


export default class IndexPage extends Component {

  render() {
    return (
      <Layout title="Home">
        <Hero />
        <Section color={'red'}>
          Hello World
        </Section>
        <Section color={'green'}>
          Hello World
        </Section>
        <Section color={'red'}>
          Hello World
        </Section>
      </Layout>
    )
  }
}
