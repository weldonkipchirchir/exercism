// #[cfg(feature = "grapheme")]
// use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    let output = input.chars().rev().collect();
    output
}

/*
What unicode-segmentation does

It implements the Unicode Text Segmentation Algorithm (UAX #29) and lets you work with:

Grapheme clusters (user-perceived characters)

Words

Sentences

For reversing strings correctly, you want graphemes.

"uüu" → "uüu" ✅
"👍🏽🔥" → "🔥👍🏽" ✅
"🇰🇪🇺🇸" → "🇺🇸🇰🇪" ✅
When should you use it?

Use unicode-segmentation when correctness matters for users:

✔ UI text
✔ Human languages
✔ Emojis
✔ Accented characters
✔ Bioinformatics display strings
*/

// pub fn reverseGrapheme(input: &str) -> String{
//         input.graphemes(true).rev().collect()
// }