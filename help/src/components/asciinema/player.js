/* global asciinema */

import React from 'react'
import clsx from 'clsx'
import baseStyles from 'asciinema-player/resources/public/css/asciinema-player.css'
import styles from './player.css'

// Based on https://github.com/neon-bindings/website/commit/1c75ba295a006240c88b43c6c7a69739db70a6a2

export default class Asciinema extends React.Component {
  constructor(...args) {
    super(...args)
    this.bindRef = (ref) => {
      this.ref = ref
    }
  }

  componentDidMount() {
    asciinema.player.js.CreatePlayer(this.ref, this.props.src, this.props)
  }

  componentWillUnmount() {
    if (!this.ref) {
      return
    }

    asciinema.player.js.UnmountPlayer(this.ref)
    this.ref = null
  }

  render() {
    return <div ref={this.bindRef} />
  }
}

Asciinema.defaultProps = {
  theme: 'asciinema',
  idleTimeLimit: 2,
  fontSize: '15px',
}
