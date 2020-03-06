import React, {Component} from "react";
import styles from './Header.module.scss';
import {graphql, Link, StaticQuery} from "gatsby";
import {FontAwesomeIcon} from "@fortawesome/react-fontawesome";
import { faGithub } from '@fortawesome/free-brands-svg-icons'

const headerQuery = graphql`
  query HeaderQuery {
    site {
      siteMetadata {
        repository
        menuLinks {
          name
          link
        }
      }
    }
  }
`;

export interface HeaderProps {
  title: string
}

export default class Header extends Component<HeaderProps> {

  render() {
    const {title} = this.props;

    return (
      <>
        <div className={styles.headerPlaceHolder} />
        <StaticQuery
          query={headerQuery}
          render={data => (
            <div className={styles.header}>
              <Link to={'/'} className={styles.itemMain}>
                <span>{title}</span>
              </Link>
              { data.site.siteMetadata.menuLinks.map(({ name, link }) => (
                <Link key={name} to={link} className={styles.item}>
                  <span>{name}</span>
                </Link>
              )) }
              <div className={styles.spacer}/>
              <a
                href={data.site.siteMetadata.repository}
                className={styles.github}
              ><FontAwesomeIcon size={'2x'} icon={faGithub} color={'white'} /></a>
            </div>
          )}
        />
      </>
    )
  }
}
