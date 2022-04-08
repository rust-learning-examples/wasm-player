mod utils;
mod player;

use wasm_bindgen::prelude::*;
use player::Player;
// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub async fn greet() -> () {
    alert("Hello world");
}


#[wasm_bindgen]
#[derive(Debug)]
pub struct WasmPlayer {
  player: Option<Player>,
//   on_data_closure: js_sys::Function
}

#[wasm_bindgen]
impl WasmPlayer {
    // JS Function: https://rustwasm.github.io/wasm-bindgen/reference/receiving-js-closures-in-rust.html
    pub async fn new(m3u8_url: String) -> Result<WasmPlayer, JsValue> {
        let mut wasm_player = WasmPlayer {
            player: None,
            // on_data_closure
        };
        let player = Player::new(m3u8_url.clone());
        wasm_player.player = Some(Player::fetch_playlists(player, m3u8_url).await?);

        Ok(wasm_player)
    }
    pub fn get_master_playlist_info(&self) -> Result<JsValue, JsValue> {
        if let Some(player) = &self.player {
            Ok(player.get_master_playlist_info())
        } else {
            Err("player not found".into())
        }
    }
    pub fn get_media_playlist_info(&self) -> Result<JsValue, JsValue> {
        if let Some(player) = &self.player {
            Ok(player.get_media_playlist_info())
        } else {
            Err("player not found".into())
        }
    }
    pub fn play(&mut self) {
        if let Some(player) = &self.player {
            player.play().unwrap();
        }
    }
}
