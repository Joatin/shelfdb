import React, {Component} from "react";
import styles from './Header.module.scss';
import {Link} from "gatsby";
import GithubCorner from "react-github-corner";

export interface HeaderProps {
  title: string
}

export default class Header extends Component<HeaderProps> {

  render() {
    const {title} = this.props;

    return (
      <div className={styles.header}>
        <Link to={'/'} className={styles.itemMain}>
          <span>{title}</span>
        </Link>
        <Link to={'/'} className={styles.item}>
          <span>Docs</span>
        </Link>
        <Link to={'/'} className={styles.item}>
          <span>Blog</span>
        </Link>

        <GithubCorner
          href={'https://github.com/Joatin/shelfdb'}
          size={90}
          direction="right"
        />
      </div>
    )
  }
}
