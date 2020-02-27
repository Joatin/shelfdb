import React, {Component} from "react";
import {graphql, StaticQuery} from "gatsby";
import SEO from "../SEO/SEO";
import Header from "../Header/Header";

import 'normalize.css';
import Footer from "../Footer";

interface LayoutProps {
  lang?: string,
  meta?: { name?: string, property?: string, content: string }[],
  description?: string,
  title?: string
}

export default class Layout extends Component<LayoutProps> {
  render() {
    const {children, title} = this.props;

    return(
      <StaticQuery
        query={graphql`
          query SiteTitleQuery {
            site {
              siteMetadata {
                title
              }
            }
          }
        `}
        render={data => (
          <div>
            <SEO {...this.props} title={title || data.site.siteMetadata.title}/>
            <Header title={data.site.siteMetadata.title} />
            {children}
            <Footer/>
          </div>
        )}
      />
    )
  }
}
