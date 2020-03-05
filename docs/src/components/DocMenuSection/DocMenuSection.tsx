import React, {Component} from "react";
import styles from './DocMenuSection.module.scss';
import {Link} from "gatsby";

export interface DocMenuSectionProps {
  menu: Array<{
    title: string,
    to: string
  }>
}

export default class DocMenuSection extends Component<DocMenuSectionProps> {
  render() {
    const { menu } = this.props;
    return (
      <div className={styles.container}>
        <ul>
          {
            menu.map(i => (
              <li>
                <Link to={i.to}>
                  <span>{i.title}</span>
                </Link>
              </li>
            ))
          }
        </ul>
      </div>
    )
  }
}
