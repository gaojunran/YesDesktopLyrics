use anyhow::anyhow;
use anyhow::Result;
use regex::Regex;
use serde_json::Value;

#[derive(Debug)]
pub struct PlayerInfo {
    pub(crate) song_id: u64,
    pub(crate) progress: f64
}

#[derive(Debug)]
pub struct SongLyrics {
    song_id: u64,
    lyrics: Vec<Lyric>
}

impl SongLyrics {
    pub fn current(&self, progress: f64) -> Option<&Lyric> {
        let mut current_lyric: Option<&Lyric> = None;
        for lyric in &self.lyrics {
            if lyric.time <= (progress + 0.75) {
                current_lyric = Some(lyric);
            } else {
                break;
            }
        }
        current_lyric
    }
}

#[derive(Debug)]
pub struct Lyric {
    time: f64,
    content: String
}

pub async fn fetch_player() -> Result<PlayerInfo> {
    let url = "http://127.0.0.1:27232/player";
    let response = reqwest::get(url).await?;

    if response.status().is_success() {
        let json: Value = response.json().await?;
        // 提取歌曲ID
        let song_id = json["currentTrack"]["id"]
            .as_u64()
            .ok_or_else(|| anyhow!("Failed to parse song ID"))?;

        // 提取播放进度
        let progress = json["progress"]
            .as_f64()
            .ok_or_else(|| anyhow!("Failed to parse progress"))?;

        Ok(PlayerInfo {
            song_id,
            progress
        })
    } else {
        Err(anyhow!("Failed to fetch player info: {}. Is YesPlayMusic running?", response.status()))
    }
}

pub async fn fetch_lyrics(id: u64) -> Result<SongLyrics> {
    let url = format!("http://127.0.0.1:10754/lyric?id={id}");
    let response = reqwest::get(url).await?;

    if response.status().is_success() {
        let json: Value = response.json().await?;

        let lyrics_str = json["lrc"]["lyric"].as_str()
            .ok_or(anyhow!("Lyrics not found"))?;

        let re = Regex::new(r"(?m)\[(\d{2}:\d{2}\.\d{2,3})\](.*?)$")?;
        let mut lyrics = Vec::new();

        for cap in re.captures_iter(lyrics_str) {
            let time_str = cap.get(1)
                .ok_or(anyhow!("Time not found"))?.as_str();
            let content = cap.get(2)
                .ok_or(anyhow!("Content not found"))?.as_str();

            // 将时间字符串转换为秒
            let parts: Vec<&str> = time_str.split(':').collect();
            let minutes: f64 = parts[0].parse()?;
            let seconds: f64 = parts[1].parse()?;
            let time = minutes * 60.0 + seconds;

            lyrics.push(Lyric {
                time,
                content: content.to_string(),
            });
        }
        Ok(SongLyrics { song_id: id, lyrics })
    } else {
        Err(anyhow!("Failed to fetch lyrics: {}. ", response.status()))
    }
}