//! `chunk_parser` attribute macro.

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields, Field};

/// Generate required and optional implementations for ChunkParser.
///
/// # Attributes
///
/// The `custom` attribute prevents the macro from generating a default impl for
/// `ChunkParser` so that its behaviour can be customised.
///
/// The `depth` attribute adds a `u8` field and implements `push()`, `pop()` and
/// `depth()`. This attribute is automatically added to the default parser.
///
/// The `path` attribute adds a `PathBuf` field that is initialised with the
/// resource's location, and implements `path()` to retrieve the value.
#[proc_macro_attribute]
pub fn chunk_parser(args: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput); // parse syntax tree

    // struct and impl components
    let name = input.ident; // extract struct name
    let mut fields = quote! {}; // struct fields
    let mut defaults = quote! {}; // default initialisers
    let mut impls = quote! {}; // implementations
    // ctor extras
    let mut cursor_ctor_extra = quote! {};
    let mut file_ctor_extra = quote! {};

    // optional feature config
    let mut has_depth = false;
    let mut is_custom = false;
    let mut needs_depth = false;

    // generate existing fields
    if let Data::Struct(data) = input.data {
        if let Fields::Named(existing) = data.fields {
            for field in existing.named {
                let Field { ident, ty, .. } = field;
                fields.extend(quote! { #ident: #ty, });
                defaults.extend(quote! { #ident: #ty::default(), });
                if ident.unwrap().to_string() == "depth" {
                    has_depth = true;
                }
            }
        }
    }

    // append inner parser fields
    fields.extend(quote! { reader: R, });

    // process extra attribute features
    let parser = syn::meta::parser(|meta| {
        if meta.path.is_ident("custom") {
            is_custom = true;
        } else if !has_depth && meta.path.is_ident("depth") {
            needs_depth = true;
        } else if meta.path.is_ident("path") {
            // path attribute feature
            fields.extend(quote! { path: std::path::PathBuf, });
            defaults.extend(quote! { path: std::any::type_name::<R>().into(), });
            cursor_ctor_extra.extend(quote! { new.path = ":memory:".into(); });
            file_ctor_extra.extend(quote! { new.path = file_path.into(); });
            impls.extend(quote! {
                impl<R> ParserPath for #name<R> {
                    #[inline] fn path(&self) -> &std::path::PathBuf
                        { &self.path }
                }
            });
        } else {
            return Err(meta.error("unsupported attribute"))
        }
        Ok(())
    });
    parse_macro_input!(args with parser);

    // depth attribute feature
    if needs_depth || !is_custom {
        fields.extend(quote! { depth: u8, });
        defaults.extend(quote! { depth: 0, });
        impls.extend(quote! {
            impl<R> ParserDepth for #name<R> {
                #[inline] fn inner_depth(&mut self) -> &mut u8
                    { &mut self.depth }
            }
        });
    }

    // default parser loop attribute feature
    if !is_custom {
        impls.extend(quote! {
            impl<R: std::io::Read + std::io::Seek> ChunkParser<R> for #name<R> {}
        });
    }

    // generate expanded struct definition
    return quote! {

        // the inner reader forms the backbone of the parser
        pub struct #name<R> {
            #fields
        }

        // general ctor
        impl<R: std::io::Read> #name<R> {
            #[inline] pub fn new(reader: R) -> Self
                { Self { reader, #defaults } }
        }

        // buffer ctor
        impl #name<std::io::Cursor<&[u8]>> {
            #[inline] pub fn cursor(buffer: &'static [u8]) -> Self {
                let mut new = Self::new(std::io::Cursor::new(buffer));
                #cursor_ctor_extra
                new
            }
        }

        // file ctor
        impl #name<std::io::BufReader<std::fs::File>> {
            #[inline] pub fn file(file_path: &str) -> chunk_parser::Result<Self> {
                let file = std::fs::File::open(file_path)?;
                let mut new = Self::new(std::io::BufReader::new(file));
                #file_ctor_extra
                Ok( new )
            }
        }

        // implement parser interface
        impl<R> ParserReader<R> for #name<R> {
            #[inline] fn reader(&mut self) -> &mut R
                { &mut self.reader }
        }
        impl<R: std::io::Seek> ParserSeek<R> for #name<R> {}
        impl<R: std::io::Read> ParserRead<R> for #name<R> {}

        #impls

    }.into()
}
