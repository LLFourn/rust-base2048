# rust-base2048 &emsp; [![crates_badge]][crates_url] [![actions_badge]][actions_url] [![docs_badge]][docs_url]


[actions_badge]: https://github.com/llfourn/base2048/workflows/Rust/badge.svg
[actions_url]: https://github.com/llfourn/base2048/actions?query=workflow%3ARust
[crates_badge]: https://img.shields.io/crates/v/base2048.svg
[crates_url]: https://crates.io/crates/base2048
[docs_badge]: https://docs.rs/base2048/badge.svg
[docs_url]: https://docs.rs/base2048

base2048 encoding for packing data into tweets!

This is an experimental module for encoding chunks of 11 bits into a single character [as
counted by twitter](https://developer.twitter.com/en/docs/basics/counting-characters).
This allows you to encode 385 (280 * 11  / 8) bytes in tweet.
The api will probably remain the same but the encoding may very well change! YMV.

## Use

``` toml
[dependencies]
base2048 = "0.1.0"

```

## Example

```rust

// this will never fit in a tweet
let bytes = hex_literal::hex!(
"0100000001574981a3fb74e6632493fcab62947b07a6c228c2b9d840893ff1e7c4f143723c010000006a47304402201f2fc511e390f5dcecf5f0fcb627faff9c0acec671bf372c49e30b43cab048ff02200a10eefea2f2c7b1c5a1603b73dc4d3175b9a416db0acfedf9bf443c0be219c90121031132f6c2139c199a18bfe1fb7f7eb5d1daaf8d4d2e03bf11e833a13e62268fb5ffffffff01eda54e020000000017a914582e495bd15671cc7344ff54104a4d3e6468fff08700000000");

let encoded = base2048::encode(&bytes[..]);

// but you can fit two of these!
assert_eq!(encoded, "ƅØØƒڛಇࢠهནɆʤႻஞҥ࿖ၼಔѽଌଡǁহആႵဧəߡƍØžઘཏѭȅƻՑӜဿѫఖငଙцشܦܧٸԢಸߘഺԘদݳǶԄқႱކȞǴܣޤԛବεԔڮлͱಲயਡனಘসພຟȄݑໝൡяѝϕॸฤѠকലݷܧɺహბఉมۻӮԵЬލ৲ژрȆਠرჺჺმкਬٷܩØØǛਣޞοҪၸઽɣКȜ࿃ǿӧӐഒࢠႳɝØØØ");

assert_eq!(base2048::decode(&encoded), Some(bytes.to_vec()));
```


## Previous Work

This was inspired by the javascript module [base2048](https://github.com/qntm/base2048) but it is not compatible at all.
Quite a bit of effort has been put into this module to make sure all the characters used are visible to people running the default font in ubuntu (please open an issue if you come across characters you can't see).
Also it only uses letter characters where as the javascript one uses punctuation marks etc which make the encoding look weird (more than usual)
