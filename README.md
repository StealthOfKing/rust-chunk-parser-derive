# chunk-parser-derive

`chunk-parser-derive` provides the `chunk_parser()` attribute macro for `chunk-parser`.

## Usage

The macro implements everything required to define a specialised chunk parser:

```rust
use chunk_parser::prelude::*;
use chunk_parser::chunk_parser;

#[chunk_parser]
struct IFFParser;
impl<R> Parser for IFFParser<R> where R: std::io::Read + std::io::Seek {
    type Header = (TypeId, i32);
    fn parse_header(&mut self) -> Result<Self::Header> {
        Ok((self.read()?, self.read_be()?))
    }
}
```
