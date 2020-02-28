import React, {Component} from "react";
import styles from './MobileMenu.module.scss';
import {faHome} from "@fortawesome/free-solid-svg-icons";
import {FontAwesomeIcon} from "@fortawesome/react-fontawesome";
import {graphql, Link, StaticQuery} from "gatsby";

export default class MobileMenu extends Component {

  render() {
    return (
      <>
        <div className={styles.mobileMenuPlaceHolder} />
        <StaticQuery
          query={graphql`
          query MobileMenuQuery {
            site {
              siteMetadata {
                menuLinks {
                  name
                  link
                  icon
                }
              }
            }
          }
        `}
          render={data => (
            <div className={styles.mobileMenu}>
              <Link to={'/'} className={styles.item}>
                <FontAwesomeIcon size={'lg'} icon={faHome} color={'#1D4350'} />
                <span>Home</span>
              </Link>
              { data.site.siteMetadata.menuLinks.map(({name, link, icon}) => (
                <Link key={name} to={link} className={styles.item}>
                  <FontAwesomeIcon size={'lg'} icon={icon} color={'#1D4350'} />
                  <span>{name}</span>
                </Link>
              )) }
            </div>
          )}
        />

      </>
    )
  }
}
