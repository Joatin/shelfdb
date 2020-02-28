import React, {Component} from "react";
import styles from './DocContainer.module.scss';

export default class DocContainer extends Component {
  render() {
    const { children } = this.props;
    return (
      <div className={styles.container}>
        {children}
      </div>
    )
  }
}
