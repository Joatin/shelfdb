import React, {Component} from "react";
import Container from "../Container";
import styles from './Footer.module.scss';


export default class Footer extends Component {

  render() {
    return (
      <div className={styles.footer}>
        <Container>
          <div className={styles.container}>
            <div className={styles.links}>
              <div>
                link
                link
                link
              </div>
              <div>
                link
                link
                link
              </div>
              <div>
                link
                link
                link
              </div>
            </div>
            <div className={styles.copyrightRow}>
              <span>Copyright blah blah</span>
            </div>
          </div>
        </Container>
      </div>
    )
  }
}
