mod config;
mod noto_animated_emoji;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use crate::config::GLOBAL_CONFIG;

    use super::*;

    // #[test]
    // fn it_works() {
    //     let path = GLOBAL_CONFIG.noto_emoji_metadata_path;
    //
    //     path
    //     assert_q!(result, 4);
    //
    //
    // }
}
