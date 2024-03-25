use std::{fs::File, io::BufReader, path::PathBuf};

use serde::Deserialize;

#[derive(Debug, PartialEq, Eq, Deserialize)]
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
}
