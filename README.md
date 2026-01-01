[license]: https://github.com/threadexio/tokio-anysocket/blob/master/LICENSE
[crates-io]: https://crates.io/crates/tokio-anysocket
[docs-rs]: https://docs.rs/tokio-anysocket/latest/tokio-anysocket

[license-badge]: https://img.shields.io/github/license/threadexio/tokio-anysocket?style=flat-square
[version-badge]: https://img.shields.io/crates/v/tokio-anysocket?style=flat-square
[docs-badge]: https://img.shields.io/docsrs/tokio-anysocket?style=flat-square

[tokio]: https://crates.io/crates/tokio

<div class="rustdoc-hidden">

<div align="center">

  <h1>
    tokio-anysocket
  </h1>
  <br>
  <br>

  <p>
    Abstracted API over tokio's TCP and Unix streams.
  </p>

  [![version-badge]][crates-io]
  [![docs-badge]][docs-rs]
  [![license-badge]][crates-io]

  <br>
  <br>

</div>

</div>

A simple crate that abstracts over [tokio]'s TCP and Unix stream sockets
allowing an application to choose at runtime which socket it uses.

This crate aims to be a drop-in replacement for [tokio]'s imports. Any deviation
from [tokio]'s API is considered a bug. (not including differences in the import
paths themselves)
