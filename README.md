# Text [![Version][version-img]][version-url] [![Status][status-img]][status-url]

The package provides a text toolbox.

## [Documentation][doc]

## Example

```rust
extern crate font;
extern crate text;

use font::File;
use text::Layout;

let path = "SourceSerifPro-Regular.otf";
let font = File::open(path).unwrap().fonts.remove(0);

let mut layout = Layout::new(font);
let text = layout.draw("The quick brown fox jumps over the lazy dog.").unwrap();
```

## Contribution

Your contribution is highly appreciated. Do not hesitate to open an issue or a
pull request. Note that any contribution submitted for inclusion in the project
will be licensed according to the terms given in [LICENSE.md](LICENSE.md).

[doc]: https://bodoni.github.io/text
[status-img]: https://travis-ci.org/bodoni/text.svg?branch=master
[status-url]: https://travis-ci.org/bodoni/text
[version-img]: https://img.shields.io/crates/v/text.svg
[version-url]: https://crates.io/crates/text
