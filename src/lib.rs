mod utils;

use wasm_bindgen::prelude::*;
use m3u8_rs::Playlist;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub async fn greet() -> Result<JsValue, JsValue> {
    // https://d2zihajmogu5jn.cloudfront.net/bipbop-advanced/bipbop_16x9_variant.m3u8
    // https://bitdash-a.akamaihd.net/content/sintel/hls/playlist.m3u8
    // https://test-streams.mux.dev/x36xhzz/x36xhzz.m3u8
    let res = match reqwest::Client::new().get("https://bitdash-a.akamaihd.net/content/sintel/hls/playlist.m3u8").send().await {
        Ok(res) => res,
        Err(_) => return Err("网络错误".into()),
    };
    let m3u8_text = match res.text().await {
        Ok(text) => text,
        Err(_) => return Err("解析错误".into())
    };

    match m3u8_rs::parse_playlist_res(m3u8_text.as_bytes()) {
        Ok(Playlist::MasterPlaylist(pl)) => Ok(format!("{:#?}", pl).into()),
        Ok(Playlist::MediaPlaylist(pl)) => Ok(format!("{:#?}", pl).into()),
        Err(e) => Err(format!("{:?}", e).into()),
    }
}
