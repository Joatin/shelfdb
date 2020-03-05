import React, {Component} from "react";
import { MDXProvider } from "@mdx-js/react"
import { MDXRenderer } from "gatsby-plugin-mdx"
import {graphql, Link, StaticQuery} from "gatsby";
import DocMenuSection from "../DocMenuSection";
import SEO from "../SEO/SEO";
import Header from "../Header/Header";
import Footer from "../Footer";
import MobileMenu from "../MobileMenu";
const shortcodes = { Link };
import styles from './DocsLayout.module.scss';
import DocContainer from "../DocContainer/DocContainer";
import {library} from "@fortawesome/fontawesome-svg-core";
import {fab} from "@fortawesome/free-brands-svg-icons";
import {fas} from "@fortawesome/free-solid-svg-icons";
import Container from "../Container/Container";

library.add(fab, fas);

const menu = [
  {
    title: "Welcome",
    to: "/docs/welcome"
  }
];

export default class DocsLayout extends Component<any> {

  render() {
    const {data: { mdx, site: { siteMetadata } }} = this.props;
    return (
      <div className={styles.container}>
        <SEO {...this.props} title={siteMetadata.title}/>
        <Header title={siteMetadata.title} />
        <div className={styles.contentContainer}>
          <div className={styles.mdContainer}>
            <DocMenuSection menu={menu} />
            <Container>
              <MDXProvider components={shortcodes}>
                <MDXRenderer>{mdx.body}</MDXRenderer>
              </MDXProvider>
            </Container>
            <div className={styles.docPadding}/>
          </div>
          <Footer/>
        </div>
        <MobileMenu />
      </div>
    )
  }
}

export const pageQuery = graphql`
  query BlogPostQuery($id: String) {
    site {
      siteMetadata {
        title
      }
    }
    mdx(id: { eq: $id }) {
      id
      body
      frontmatter {
        title
      }
    }
  }
`
