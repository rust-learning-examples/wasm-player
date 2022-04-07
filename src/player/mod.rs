use m3u8_rs::Playlist;
use wasm_bindgen::prelude::*;
use url::Url;
use std::pin::Pin;
use std::future::Future;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = console)]
    fn debug(s: &str);
}


#[derive(Debug)]
pub struct Player {
  m3u8_url: String,
  playlist: Option<Playlist>,
}

impl Player {
  pub fn new(m3u8_url: String) -> Player {
    Player {
      m3u8_url: m3u8_url.clone(),
      playlist: None,
    }
  }
  pub fn fetch_playlists(mut player: Player, m3u8_url: String) -> Pin<Box<dyn Future<Output = Result<Player, JsValue>>>> {
    Box::pin(async move {
      let res = match reqwest::Client::new().get(&m3u8_url).send().await {
        Ok(res) => res,
        Err(_) => return Err("网络错误".into()),
      };
      let m3u8_text = match res.text().await {
          Ok(text) => text,
          Err(_) => return Err("解析错误".into())
      };

      match m3u8_rs::parse_playlist_res(m3u8_text.as_bytes()) {
          Ok(Playlist::MasterPlaylist(mut pl)) => {
            debug("get MasterPlaylist");
            if let Some(newest_variant) = pl.get_newest_variant() {
              let base_url = Url::parse(&m3u8_url).unwrap();
              let url = Url::options().base_url(Some(&base_url)).parse(&newest_variant.uri).unwrap();
              debug(&format!("{:?}", url));
              return Player::fetch_playlists(player, url.as_str().to_owned()).await;
            } else {
              return Err("MasterPlaylist中无播放信息".into())
            }
          },
          Ok(Playlist::MediaPlaylist(pl)) => {
            debug("get MediaPlaylist");
            player.playlist = Some(Playlist::MediaPlaylist(pl));
            Ok(player)
          },
          Err(e) => Err(format!("{:?}", e).into()),
      }
    })
  }

  pub fn get_playlist_info(&self) -> JsValue {
    format!("{:#?}", self.playlist).into()
  }

  pub fn play(&self) -> Result<(), String> {
    debug("play todo...");
    Ok(())
  }
}