function unicode(str) {
    return Array.prototype.map.call(str, (_, i) => str.codePointAt(i))
        .filter(i => !(i >= 0xdc00 && i <= 0xdfff) && i !== 0xfe0f)
        .map(i => `\\u{${i.toString(16)}}`)
        .join('')
}

const tones = [
    '\\u{1f3fb}',
    '\\u{1f3fc}',
    '\\u{1f3fd}',
    '\\u{1f3fe}',
    '\\u{1f3ff}'
]

const emojis = []

const rawEmojis = await fetch('https://raw.githubusercontent.com/github/gemoji/master/db/emoji.json')
    .then(resp => resp.json())

rawEmojis.forEach(e => {
    e.aliases.forEach(a => {
        const encoded = unicode(e.emoji)
        emojis.push(`  "${a}" => "${encoded}"`)
        if (e.skin_tones) {
            tones.forEach((t, i) => emojis.push(`  "${a}_tone${i + 1}" => "${encoded}${t}"`))
        }
    })
})


Deno.writeTextFileSync('./src/map.rs',
    `use phf::{phf_map};
pub static EMOJI_MAP: phf::Map<&'static str, &'static str> = phf_map! {
${emojis.join(',\n')}
};
`)
