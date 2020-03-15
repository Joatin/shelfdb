import React, {Component} from "react";
import Section, {SectionProps} from "../Section/Section";
import UndrawQuestion from 'react-undraw/dist/illustrations/UndrawQuestions';
import styles from './UndrawSection.module.scss';

export interface UndrawSectionProps extends SectionProps {
  undraw: string;
  right?: boolean;
}

export default class UndrawSection extends Component<UndrawSectionProps> {

  render() {
    const {children, undraw = UndrawQuestion, right = false} = this.props;

    const Undraw = undraw;

    return (
      <Section {...this.props}>
        <div className={styles.container}>
          <div style={{order: right ? 3 : 1}}>
            <Undraw primaryColor={'lightgray'} />
          </div>
          <div style={{order: 2}}>
            {children}
          </div>
        </div>
      </Section>
    )
  }
}
