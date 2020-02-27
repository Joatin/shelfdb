import React, {Component} from "react";
import Container from "../Container";
import styles from './Section.module.scss';

export interface SectionProps {
  color: string
}

export default class Section extends Component<SectionProps> {

  render() {
    const {children, color} = this.props;
    return (
      <div className={styles.section} style={{backgroundColor: color}}>
        <Container>
          {children}
        </Container>
      </div>
    )
  }
}
