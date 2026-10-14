use unicode_properties::{EmojiStatus, UnicodeEmoji};
use unicode_segmentation::UnicodeSegmentation;
use EmojiStatus::{
    EmojiPresentation, EmojiPresentationAndEmojiComponent,
    EmojiPresentationAndModifierAndEmojiComponent, EmojiPresentationAndModifierBase,
};

pub fn gather_emojis_from_run<'a>(
    line: &'a str,
    start_run: usize,
    end_run: usize,
) -> impl Iterator<Item = (usize, usize)> + 'a {
    line[start_run..end_run]
        .grapheme_indices(true)
        .filter_map(move |(offset, grapheme)| {
            if is_color_emoji(grapheme) {
                Some((start_run + offset, grapheme.len()))
            } else {
                None
            }
        })
}

/// <https://www.unicode.org/reports/tr51/#EBNF_and_Regex>
fn is_color_emoji(g: &str) -> bool {
    let Some(first) = g.chars().next() else {
        return false;
    };
    if first.is_ascii() && !matches!(first, '0'..='9' | '#' | '*') {
        false
    } else if g.contains('\u{FE0E}') {
        false
    } else if g.contains('\u{FE0F}') {
        g.chars().any(|c| c.is_emoji_char_or_emoji_component())
    } else if g.contains('\u{20E3}') {
        matches!(first, '0'..='9' | '#' | '*')
    } else if matches!(
        first.emoji_status(),
        EmojiPresentation
            | EmojiPresentationAndEmojiComponent
            | EmojiPresentationAndModifierAndEmojiComponent
            | EmojiPresentationAndModifierBase
    ) {
        true
    } else {
        g.contains('\u{200D}') && g.chars().any(|c| c.is_emoji_char())
    }
}
