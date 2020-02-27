import React, {Component} from "react";
import styles from './Container.module.scss'

export default class Container extends Component {

  render() {
    const {children} = this.props;

    return (
      <div className={styles.container}>
        {children}
      </div>
    )
  }
}
