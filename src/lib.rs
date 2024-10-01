//! `chunk_parser` attribute macro.

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_attribute]
pub fn chunk_parser(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput); // parse syntax tree
    let name = input.ident; // extract struct name

    // generate expanded struct definition
    return quote! {

        // the internal reader forms the backbone of the parser
        pub struct #name<R> where R: std::io::Read + std::io::Seek {
            reader: R,
            depth: usize
        }

        // general ctor
        impl<R> #name<R> where R: std::io::Read + std::io::Seek {
            #[inline] pub fn new(reader: R) -> Self { Self { reader, depth: 0 } }
        }

        // buffer ctor
        impl #name<std::io::Cursor<&[u8]>> {
            #[inline] pub fn buf(buffer: &'static [u8]) -> Self {
                Self::new(std::io::Cursor::new(buffer))
            }
        }

        // file ctor
        impl #name<std::io::BufReader<std::fs::File>> {
            #[inline] pub fn file(file_path: &str) -> chunk_parser::Result<Self> {
                let file = std::fs::File::open(file_path)?;
                Ok( Self::new(std::io::BufReader::new(file)) )
            }
        }

        // implement parser interface
        impl<R> ParserInner for #name<R> where R: std::io::Read + std::io::Seek {
            type Reader = R;
            #[inline] fn reader(&mut self) -> &mut Self::Reader { &mut self.reader }
            #[inline] fn depth(&self) -> usize { self.depth }
            #[inline] fn set_depth(&mut self, depth: usize) { self.depth = depth; }
        }

    }.into()
}
