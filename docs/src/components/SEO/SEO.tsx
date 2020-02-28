import React, {Component} from "react";
import {graphql, StaticQuery} from "gatsby";
import Helmet from "react-helmet"

export interface SEOProps {
  lang?: string,
  meta?: { name?: string, property?: string, content: string }[],
  description?: string,
  title: string
}

export default class SEO extends Component<SEOProps> {

  render() {
    const {title, lang = 'en', meta = [], description = ''} = this.props;

    return (
      <StaticQuery
        query={graphql`
          query SiteSEOQuery {
            site {
              siteMetadata {
                title
                description
                author
              }
            }
          }
        `}
        render={({site}) => {
          const metaDescription = description || site.siteMetadata.description;
          return (
            <Helmet
              htmlAttributes={{
                lang,
              }}
              title={title}
              titleTemplate={`%s | ${site.siteMetadata.title}`}
              meta={[
                {
                  name: `viewport`,
                  content: 'width=device-width, initial-scale=1.0, viewport-fit=cover',
                },
                {
                  name: `description`,
                  content: metaDescription,
                },
                {
                  property: `og:title`,
                  content: title,
                },
                {
                  property: `og:description`,
                  content: metaDescription,
                },
                {
                  property: `og:type`,
                  content: `website`,
                },
                {
                  name: `twitter:card`,
                  content: `summary`,
                },
                {
                  name: `twitter:creator`,
                  content: site.siteMetadata.author,
                },
                {
                  name: `twitter:title`,
                  content: title,
                },
                {
                  name: `twitter:description`,
                  content: metaDescription,
                },
              ].concat(meta as any[])}
            />
          )
        }}
      />
    )
  }
}
