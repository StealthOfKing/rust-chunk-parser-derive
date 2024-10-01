# chunk-parser-derive

![GitHub Package Version](https://img.shields.io/badge/dynamic/toml?url=https%3A%2F%2Fraw.githubusercontent.com%2FStealthOfKing%2Frust-chunk-parser-derive%2Frefs%2Fheads%2Fmaster%2FCargo.toml&query=%24.package.version&prefix=v&label=Rust)
![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/StealthOfKing/rust-chunk-parser-derive/rust.yml)
![GitHub License](https://img.shields.io/github/license/StealthOfKing/rust-chunk-parser-derive)

`chunk-parser-derive` provides the `chunk_parser()` attribute macro for `chunk-parser`.

## Usage

The macro implements everything required to define a specialised chunk parser:

```rust
use chunk_parser::prelude::*;

#[chunk_parser]
struct IFFParser;
impl<R> Parser for IFFParser<R> where R: std::io::Read + std::io::Seek {
    type Header = (TypeId, i32);
    fn parse_header(&mut self) -> Result<Self::Header> {
        Ok((self.read()?, self.read_be()?))
    }
}
```
