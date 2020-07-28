# rust-base2048 &emsp; [![crates_badge]][crates_url] [![actions_badge]][actions_url] [![docs_badge]][docs_url]


[actions_badge]: https://github.com/LLFourn/rust-base2048/workflows/Rust/badge.svg
[actions_url]: https://github.com/LLFourn/rust-base2048/actions
[crates_badge]: https://img.shields.io/crates/v/base2048.svg
[crates_url]: https://crates.io/crates/base2048
[docs_badge]: https://docs.rs/base2048/badge.svg
[docs_url]: https://docs.rs/base2048

base2048 encoding for packing data into tweets!

This is an experimental module for encoding chunks of 11 bits into a single character [as
counted by twitter](https://developer.twitter.com/en/docs/basics/counting-characters).
This allows you to encode 385 (280 * 11  / 8) bytes in tweet.
The api will probably remain the same but the encoding may very well change! YMV.

I've put a bit of effort into making sure all the characters used are visible to people running with standard font packages on most platforms. **please open an issue with info about your device/OS if a character is not showing properly (e.g. you see a box looking thing).**
See [base2048.txt](./base2048.txt) for the ordered list of characters.

## Use

``` toml
[dependencies]
base2048 = "0.1"
```

## Example

```rust
// these 189 will never fit in a tweet encoded as hex
let bytes = hex_literal::hex!("0100000001574981a3fb74e6632493fcab62947b07a6c228c2b9d840893ff1e7c4f143723c010000006a47304402201f2fc511e390f5dcecf5f0fcb627faff9c0acec671bf372c49e30b43cab048ff02200a10eefea2f2c7b1c5a1603b73dc4d3175b9a416db0acfedf9bf443c0be219c90121031132f6c2139c199a18bfe1fb7f7eb5d1daaf8d4d2e03bf11e833a13e62268fb5ffffffff01eda54e020000000017a914582e495bd15671cc7344ff54104a4d3e6468fff08700000000");

// but with base2048 you can fit it - twice!
let encoded = base2048::encode(&bytes[..]);
assert_eq!(encoded, "ŐØØŝقగސץແȑɯၻଥѩཛသఢшચબƌইಖၵဂȤމŘØŉਠະиǐƆԛҧဆжணལણБדڿۀ؎ӭಋހഐӣॵܡǁӏџၦݑǩƿڼݯӦસͰӟٹІɼಆମভଫదআภมǏۍโകКШΟछෂЫरഇܥۀɅఌგஏලڥҹӿϷݘঐؾЋǑবא٭٭ნЅৎ؋ۂØØƦযݩΆѮယઌȮϥǧཌǊҲқಠސၯȨØØØ");
assert_eq!(base2048::decode(&encoded), Some(bytes.to_vec()));
```
## Previous Work

This was inspired by the javascript module [base2048](https://github.com/qntm/base2048) but they are not compatible.
The main difference is the character list is more curated here to display properly on each platform.
