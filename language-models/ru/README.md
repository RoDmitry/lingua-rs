## Russian language model for Lingua

This is the language model for the Russian language which is used by 
[*Lingua*](https://github.com/pemistahl/lingua-rs), 
the most accurate natural language detection library in the Rust ecosystem.

Models can be printed using `cat unigrams.json.br | brotli --decompress --stdout`

### Changelog

#### Version 1.1.0

- The language model files are now compressed with the Brotli algorithm which
  reduces the file size by 15 %, on average.
