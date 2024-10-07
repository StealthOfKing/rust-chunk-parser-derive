# chunk-parser-derive

![GitHub Package Version](https://img.shields.io/badge/dynamic/toml?url=https%3A%2F%2Fraw.githubusercontent.com%2FStealthOfKing%2Frust-chunk-parser-derive%2Frefs%2Fheads%2Fmaster%2FCargo.toml&query=%24.package.version&prefix=v&label=Rust)
![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/StealthOfKing/rust-chunk-parser-derive/rust.yml)
![GitHub License](https://img.shields.io/github/license/StealthOfKing/rust-chunk-parser-derive)

`chunk-parser-derive` provides the `chunk_parser()` attribute macro for `chunk-parser`.

## Usage

The macro implements everything required to define a specialised chunk parser:

```rust
use chunk_parser::prelude::*;

// Decorate IFFParser with chunk parser API.
#[chunk_parser]
struct IFFParser;

// Define the chunk header layout.
struct IFFHeader { typeid: TypeId, length: u32 }

// Define the chunk header parsing logic.
impl<R: Read> HeaderParser<IFFHeader> for IFFParser<R> {
    fn header(&mut self) -> Result<IFFHeader>
        { Ok( IFFHeader { typeid: self.read()?, length: self.read_be()? } ) }
}
```

## Custom

To define a custom parser loop, pass `custom` to the attribute macro and define a `ChunkParser` implementation.
Optionally include the `depth` attribute to add `depth()`, `pop()` and `parse()`.

```rust
#[chunk_parser(custom,depth)]
struct IFFParserCustom {}

// Optionally define a header in order to use parser.header().
impl<R: Read> HeaderParser<IFFHeader> for IFFParserCustom<R> {
    fn header(&mut self) -> Result<IFFHeader>
        { Ok( IFFHeader { typeid: self.read()?, length: self.read_be::<u32>()? - 8 } ) }
}

// Define a custom chunk parser loop.
impl<R: std::io::Read + std::io::Seek> ChunkParser<R> for IFFParserCustom<R> {
    fn parse_loop<H>(&mut self, f: ParserFn<Self,H>, total_size: u64) -> Result<()> where Self: HeaderParser<H> {
        self.push();
        match loop {
            let header = self.header()?;
            let start = self.reader().stream_position()?;
            let size = f(self, &header)? + 8; // the parser function is responsible for parsing the size
            let end = start + size;
            let pos = self.reader().stream_position()?;
            if pos == total_size { break Ok(()) } // function consumed chunk
            else if pos != end { break Err(Error::ParseError) } // function made a mistake
        } {
            res => { self.pop(); res }
        }
    }
}

// Use the custom parser with adjusted length.
let mut iff = IFFParserCustom::cursor(DATA);
iff.parse(|parser, header| parser.skip(header.length as u64 + 8))
```
