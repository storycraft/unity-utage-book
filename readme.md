# unity-utage-book
Library for reading and writing unity Utage visual novel engine book scenario file

This project is not documented. See examples for usage.

## Examples

### extract
Extract book scenario file to json
* usage: `cargo run --example extract --features=serde < input.book > output.json`

### repack
Repack unpacked json file to book
* usage: `cargo run --example repack --features=serde < input.json > output.book`
