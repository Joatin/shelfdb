import React, {Component} from "react";
import {graphql, StaticQuery} from "gatsby";
import SEO from "../SEO/SEO";
import Header from "../Header/Header";

import 'normalize.css';
import Footer from "../Footer";
import MobileMenu from "../MobileMenu";
import { library } from '@fortawesome/fontawesome-svg-core';
import {fab} from "@fortawesome/free-brands-svg-icons";
import {fas} from "@fortawesome/free-solid-svg-icons";
import styles from './Layout.module.scss';

library.add(fab, fas);

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
          <div className={styles.container}>
            <SEO {...this.props} title={title || data.site.siteMetadata.title}/>
            <Header title={data.site.siteMetadata.title} />
            {children}
            <div className={styles.spacer}/>
            <Footer/>
            <MobileMenu />
          </div>
        )}
      />
    )
  }
}
