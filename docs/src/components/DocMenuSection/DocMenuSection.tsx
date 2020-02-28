import React, {Component} from "react";
import styles from './DocMenuSection.module.scss';

export default class DocMenuSection extends Component {
  render() {
    return (
      <>
        <div className={styles.placeholder} />
        <div className={styles.container}>
          Menu
        </div>
      </>
    )
  }
}
