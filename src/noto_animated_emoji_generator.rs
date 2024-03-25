use std::{fs::File, io::BufReader, path::PathBuf};

use serde::Deserialize;

#[derive(Debug, PartialEq, Eq, Deserialize, Clone)]
#[cfg_attr(test, derive(serde::Serialize))]
struct Emoji {
    base: Vec<u32>,
    alternates: Vec<Vec<u32>>,
    emoticons: Vec<String>,
    shortcodes: Vec<String>,
    animated: bool,
    directional: bool,
}

impl Emoji {
    fn as_hex(&self) -> String {
        self.base
            .iter()
            .map(|base| format!("{:x}", base))
            .collect::<Vec<_>>()
            .join("_")
    }

    fn lottie_download_url(&self) -> String {
        format!(
            "https://fonts.gstatic.com/s/e/notoemoji/latest/{}/lottie.json",
            self.as_hex()
        )
    }
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
#[cfg_attr(test, derive(serde::Serialize))]
struct EmojiGroup {
    group: String,
    emoji: Vec<Emoji>,
}

fn deserialize_noto_animated_emojis_metadata(
    path: &PathBuf,
) -> Result<Vec<EmojiGroup>, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let emoji_groups: Vec<EmojiGroup> = serde_json::from_reader(reader)?;

    Ok(emoji_groups)
}

#[cfg(test)]
mod tests {
    use std::{fs, str::FromStr};

    use super::*;
    use crate::config::GLOBAL_CONFIG;
    use serde_json::json;

    #[test]
    fn it_deserializes_metadata() {
        let path = &GLOBAL_CONFIG.noto_emoji_metadata_path;

        let result = deserialize_noto_animated_emojis_metadata(&path);
        insta::assert_yaml_snapshot!(result.unwrap());
    }

    #[test]
    fn it_returns_correct_lottie_download_url() {
        let emoji: Emoji = serde_json::from_value(json!({
          "base": [
            127987,
            65039
          ],
          "alternates": [],
          "emoticons": [],
          "shortcodes": [
            ":white-flag:"
          ],
          "animated": true,
          "directional": false
        }))
        .unwrap();

        insta::assert_snapshot!(emoji.lottie_download_url());
    }

    #[tokio::test]
    async fn it_downloads_lottie_files() {
        let path = &GLOBAL_CONFIG.noto_emoji_metadata_path;

        let metadata = deserialize_noto_animated_emojis_metadata(&path).unwrap();

        let emojis: Vec<_> = metadata
            .iter()
            .flat_map(|group| group.emoji.clone())
            .filter(|emoji| emoji.animated)
            .collect();

        println!("Found {} animated emojis", emojis.len());

        let client = reqwest::Client::builder()
            .proxy(reqwest::Proxy::all("http://127.0.0.1:3128").unwrap())
            .build()
            .unwrap();

        for emoji in emojis {
            let emoji_path = PathBuf::from_str(
                format!("static/noto_animated_emoji/lottie/{}", emoji.as_hex()).as_str(),
            )
            .unwrap();

            let emoji_file_path = emoji_path.join("lottie.json");

            if emoji_file_path.exists() {
                continue;
            }

            let req = client.get(emoji.lottie_download_url()).build().unwrap();
            let resp = client.execute(req).await.expect("request failed");

            if !emoji_path.exists() {
                fs::create_dir(&emoji_path).unwrap();
            }

            let mut out = File::create(emoji_file_path).expect("failed to create file");
            std::io::copy(&mut resp.text().await.unwrap().as_bytes(), &mut out)
                .expect("failed to copy content");
            println!("Saved {}", emoji.lottie_download_url());
        }

        println!("Downloaded all emojis.");
    }

    #[test]
    fn it_generates_rust_file() {
        let path = &GLOBAL_CONFIG.noto_emoji_metadata_path;

        let metadata = deserialize_noto_animated_emojis_metadata(&path).unwrap();

        let emojis: Vec<_> = metadata
            .iter()
            .flat_map(|group| group.emoji.clone())
            .filter(|emoji| emoji.animated)
            .collect();

        let mut script_content: String = format!(
            "/// List of valid Noto Animated Emojis in hexadecimal with _ as character divider.\npub const NOTO_ANIMATED_EMOJIS : [&str; {}] = [",
            emojis.len()
        );

        for emoji in emojis {
            script_content += format!("\n\t\"{}\",", &emoji.as_hex()).as_str();
        }

        script_content += "\n];";

        insta::assert_snapshot!(script_content);

        let mut out = File::create("./src/noto_animated_emoji.rs").unwrap();
        std::io::copy(&mut script_content.as_bytes(), &mut out).unwrap();
    }
}
