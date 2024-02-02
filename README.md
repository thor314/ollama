# ollama
My notes using ollama. 

## Installation and running
```fish
# install 
curl https://ollama.ai/install.sh | sh

# run ollama
ollama run llama2
# or create a model from a model file
# ollama create mario -f ./Modelfile
# ollama run mario

# ask something
cargo run $PROMPT
# 
```

### Cargo

* Install the rust toolchain in order to have cargo installed by following
  [this](https://www.rust-lang.org/tools/install) guide.
* run `cargo install ollama`

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
