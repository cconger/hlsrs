# HLS-rs

Implementing a fast, stream editable HLS parser/mutator.

Current version depends on m3u8-rs as we use them as the baseline for benchmarking.

This library should be minimal and fast, to be usable as a WebAssembly library.

### Overview

This library is trying to be future looking for how one might use an HLS parser.  There are a few major
things that I expect a consumer would want to do.

1. Modify Tags
2. Add Tags
3. Verify Playlist is Parsable
4. Verify Playlist matches Spec
5. Use high level Playlist objects for loading content

The primary goal (why I wrote this) was to do the first.

However I see there being value separating the errors for parsability from spec conformance and verifying
them both.  Also I was disappointed that the current best offering is a parser combinator based approach
which while ergonomic to write is slow.

This library attempts to allow you to interact with the datastructures at two tiers.  The first is at a "Tag Level".  This is where we have done a first pass and turned each of the lines into a known (if known) HLS tag with the proper types.  This does the first pass of checking for parsability.

The second step is to then try to create an data structure that represents the data as you might ergonomically use it.  This step of turning one into the ohter is also a time when we can verify and enforce compatibility with the spec.

Finally this high level abstract object can be used for driving playback or doing analysis.


### TODOS:
- [ ] Helpers around parsing Attribute Lists
- [ ] Handle all Media Playlist tags
- [ ] Fix Error types to a ParseError that coalesces sub errors.
- [ ] Remove `unwrap()` from `parse_tag`
- [ ] Make this library just about parsing HLS and providing the streaming modification and split out the WASM wrapper
- [ ] Make m3u8-rs a dep only for bench target (if possible?)
- [ ] More Benchmarks and tests.

### HLS Spec
This implementation is targetting [Draft 11 of the HLS2 spec](https://datatracker.ietf.org/doc/html/draft-pantos-hls-rfc8216bis) that is attempting to replace [RPC 2816](https://datatracker.ietf.org/doc/html/rfc8216).

### Tests

```
cargo test
```

### WASM
Project is designed to be embedded as a wasm module.

```
wasm-pack build --release
```

Or For Cloudflare:
```
worker-build --release
```

Short term, the idea is that the webassembly code would import this package, and so you can make all the
modifications you want in WASM space, and then serialize the final playlist back to the host env.  But it
would still be nice to afford some streaming capability.


The long term goal is to do something to the effect of:
```javascript
const originalPlaylist = "...";

// Mutate at the tag level
console.log(hlsrs.Mutate(
  originalPlaylist, 
  "#segment#uri",
  (uri) => (uri + "?foo=bar"),
));
```

### Bench
To compare with m3u8-rs
```
cargo bench
```

Will attempt to parse a 2MB included hls manifest in both frameworks.

TODO: Include alternative benchmarks beyond just simple parsing.  Add stringification.
