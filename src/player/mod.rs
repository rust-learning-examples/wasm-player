use m3u8_rs::Playlist;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug)]
pub struct Player {
  m3u8_url: String,
  playlist: Option<Playlist>,
}

#[wasm_bindgen]
impl Player {
  pub async fn new(m3u8_url: String) -> Result<Player, JsValue> {
    let mut player = Player {
      m3u8_url,
      playlist: None,
    };
    let res = match reqwest::Client::new().get(&player.m3u8_url).send().await {
      Ok(res) => res,
      Err(_) => return Err("网络错误".into()),
    };
    let m3u8_text = match res.text().await {
        Ok(text) => text,
        Err(_) => return Err("解析错误".into())
    };

    match m3u8_rs::parse_playlist_res(m3u8_text.as_bytes()) {
        Ok(Playlist::MasterPlaylist(pl)) => {
          player.playlist = Some(Playlist::MasterPlaylist(pl));
          Ok(player)
        },
        Ok(Playlist::MediaPlaylist(pl)) => {
          player.playlist = Some(Playlist::MediaPlaylist(pl));
          Ok(player)
        },
        Err(e) => Err(format!("{:?}", e).into()),
    }
  }
  pub fn get_playlist_info(&self) -> JsValue {
    format!("{:#?}", self.playlist).into()
  }
}