# Line clipping

<!-- cargo-rdme start -->

A rust crate to implement several line clipping algorithms. See the
[documentation](https://docs.rs/line_clipping) for more information. The choice of algorithms is
based on the following article which contains a good summary of the options:

Matthes D, Drakopoulos V. [Line Clipping in 2D: Overview, Techniques and
Algorithms](https://pmc.ncbi.nlm.nih.gov/articles/PMC9605407/). J Imaging. 2022 Oct
17;8(10):286. doi: 10.3390/jimaging8100286. PMID: 36286380; PMCID: PMC9605407.

Supports:

- [x] Cohen-Sutherland

TODO

- [ ] Cyrus-Beck
- [ ] Liang-Barsky
- [ ] Nicholl-Lee-Nicholl
- [ ] More comprehensive testing

## Installation

```shell
cargo add line-clipping
```

## Usage

```rust
use line_clipping::cohen_sutherland::clip_line;
use line_clipping::{LineSegment, Point, Window};

let line = clip_line(
    LineSegment::new(Point::new(0.0, 0.0), Point::new(10.0, 10.0)),
    Window::new(1.0, 9.0, 1.0, 9.0),
);
```

## License

Copyright (c) 2024 Josh McKinney

This project is licensed under either of

- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)

at your option.

## Contribution

Contributions are welcome! Please open an issue or submit a pull request.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in
the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without
any additional terms or conditions.

<!-- cargo-rdme end -->
