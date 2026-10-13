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
            if grapheme_is_emoji(grapheme) {
                Some((start_run + offset, grapheme.len()))
            } else {
                None
            }
        })
}

/// https://www.unicode.org/reports/tr51/#EBNF_and_Regex
fn grapheme_is_emoji(g: &str) -> bool {
    let mut chars = g.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    if first.is_ascii() && !matches!(first, '0'..='9' | '#' | '*') {
        return false;
    } else if g.contains('\u{FE0E}') {
        return false;
    } else if g.contains('\u{FE0F}') {
        return g.chars().any(|c| c.is_emoji_char_or_emoji_component());
    } else if g.contains('\u{20E3}') {
        return matches!(first, '0'..='9' | '#' | '*');
    } else if matches!(
        first.emoji_status(),
        EmojiPresentation
            | EmojiPresentationAndEmojiComponent
            | EmojiPresentationAndModifierAndEmojiComponent
            | EmojiPresentationAndModifierBase
    ) {
        return true;
    } else if g.contains('\u{200D}') && g.chars().any(|c| c.is_emoji_char()) {
        return true;
    } else {
        return false;
    }
}
