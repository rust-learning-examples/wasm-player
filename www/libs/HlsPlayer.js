import { WasmPlayer } from 'wasm-player'
class HlsPlayer {
  constructor(container, options) {
    options = Object.assign({m3u8Url: ''}, options)
    this.container = container
    this.options = options
    this.WasmPlayer = WasmPlayer
    this.core = null

    if (options.m3u8Url) {
      WasmPlayer.new(options.m3u8Url).then(core => {
        this.core = core
      })
    }
  }
  async parseM3u8Url(m3u8Url) {
    this.options.m3u8Url = m3u8Url
    this.core = await WasmPlayer.new(m3u8Url)
  }
  getPlaylistInfo() {
    if (this.core) {
      return this.core.get_playlist_info()
    }
  }
  play() {
    if (this.core) {
      this.core.play()
    }
  }
}

export default HlsPlayer