// import * as wasm from "hello-wasm-pack";
// wasm.greet();

import * as wasm from 'wasm-player'
import HlsPlayer from './libs/HlsPlayer'

const container = document.querySelector('video')
const player = new HlsPlayer(container, {})
player.parseM3u8Url('https://bitdash-a.akamaihd.net/content/sintel/hls/playlist.m3u8').then(res => {
    console.log('player', player);
    const playlistInfo = player.getPlaylistInfo();
    console.log(playlistInfo);
    player.play();
})

