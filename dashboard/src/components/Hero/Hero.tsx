import React, {Component} from "react";
import Typist from 'react-typist';
import 'react-typist/dist/Typist.css';

import styles from './Hero.module.scss';
import Container from "../Container/Container";
import {Link} from "gatsby";

export default class Hero extends Component {
  render() {
    return (
      <div className={styles.hero}>
        <Container>
          <div className={styles.container}>
            <div className={styles.titleSection}>
              <h1 className={styles.heroTile}>Shelf</h1>
              <h2 className={styles.heroSubTile}>The GraphQL Database!</h2>
              <Link to={'/docs/installation'} className={styles.heroButton}><span>Install now</span></Link>
            </div>
            <div className={styles.demoSection}>
              <div className={styles.demo}>
                <Typist startDelay={500} className={styles.demoText}>
                  <span><span style={{color: 'rgb(241, 143, 1)'}}># car_schema.graphql</span></span>
                  <br/><br/>
                  <span><span style={{color: 'rgb(42, 126, 211)'}}>type</span> <span style={{color: 'yellow'}}>Car</span> <span style={{color: 'rgb(241, 143, 1)'}}>@collection</span> <span>{'{'}</span></span>
                  <br/>
                  <span>&nbsp;&nbsp;&nbsp;&nbsp;<span style={{color: 'rgb(42, 126, 211)'}}>id:</span> <span style={{color: 'rgb(41, 185, 115)'}}>Uuid!</span></span>
                  <br />
                  <span>&nbsp;&nbsp;&nbsp;&nbsp;<span style={{color: 'rgb(42, 126, 211)'}}>createdAt:</span> <span style={{color: 'rgb(41, 185, 115)'}}>DateTimeUtc!</span></span>
                  <br/>
                  <span>&nbsp;&nbsp;&nbsp;&nbsp;<span style={{color: 'rgb(42, 126, 211)'}}>brand:</span> <span style={{color: 'rgb(41, 185, 115)'}}>String!</span></span>
                  <br/>
                  <span>&nbsp;&nbsp;&nbsp;&nbsp;<span style={{color: 'rgb(42, 126, 211)'}}>model:</span> <span style={{color: 'rgb(41, 185, 115)'}}>String!</span></span>
                  <br/>
                  <span>&nbsp;&nbsp;&nbsp;&nbsp;<span style={{color: 'rgb(42, 126, 211)'}}>productionYear:</span> <span style={{color: 'rgb(41, 185, 115)'}}>String!</span></span>
                  <br/>
                  <span>&nbsp;&nbsp;&nbsp;&nbsp;<span style={{color: 'rgb(42, 126, 211)'}}>batterySize:</span> <span style={{color: 'rgb(41, 185, 115)'}}>String!</span></span>
                  <br/>
                  <span><span>{'}'}</span></span>
                </Typist>
              </div>
            </div>
          </div>
        </Container>
      </div>
    )
  }
}
