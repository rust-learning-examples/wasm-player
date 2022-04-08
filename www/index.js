// import * as wasm from "hello-wasm-pack";
// wasm.greet();

import HlsPlayer from './libs/HlsPlayer'

const container = document.querySelector('video')
const player = new HlsPlayer(container, {})
player.parseM3u8Url('https://bitdash-a.akamaihd.net/content/sintel/hls/playlist.m3u8').then(res => {
    console.log('player', player);
    const masterPlaylistInfo = player.getMasterPlaylistInfo();
    console.log(masterPlaylistInfo);
    const mediaPlaylistInfo = player.getMediaPlaylistInfo();
    console.log(mediaPlaylistInfo);

    player.play();
})

