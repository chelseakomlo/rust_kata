## Rust code kata for Tor Dev

### Purpose

To learn non-trivial Rust, fight with the borrow checker, and see some nice
qualities of Rust and how that applies to tor.

### The exercise

1. Fetch the latest consensus

2. Build a program that will count the hamming distance between relay names to
   see if anyone is spamming the network with poorly-named relays.

How you can do this:
  - Filter the consensus for r (router) lines.
  - Extract the router name from each line.
  - For each router name, compare it to all of the other router names using
    (Damerau-)Levenshtein distance (or optionally for toy cases, the Hamming
    distance). For names which are < 3 distance, this is considered a match.
  - Return a `Vec<String>` (or another type, see the "Bonuses" section), where
    the `String`s are the matching router names.

Tips:
  - To start, just download the consensus directly, so you can start by writing
    Rust to read/parse/calculate distance. Once this has been completed, check
    out [the `hyper` crate](https://hyper.rs/), an HTTP library, for fetching
    the consensus.  If you prefer more of a challenge, you might try using
    [`tokio`](https://tokio.rs/), a lower-level async networking library,
    directly.
  - For parsing the consensus, you can probably just use the methods on native
    types, since you only need one field.  If you prefer more of a challenge,
    perhaps check out [the `nom` crate](https://github.com/Geal/nom) for an idea
    of how an actual descriptor parser might be built.

### Bonus Points

Any of the following are grounds for bonus points:
  - Filter the top five relay names with the highest number of matches.
  - Reduce the amount of copies that your program does by operating on borrowed,
    rather than newly-allocated, types, e.g. `&str` and `&[T]` rather than
    `String` and `Vec<T>`.
  - Implement some other metric(s) for determining if one relay is likely a
    sybil of another.
  - Produce a text document of the likely sybils, and submit it (whether by
    `HTTP POST` or `ssh` or `rsync` or some other transport, your choice) to a
    server (preferably one belonging to either you or Tor :).
  - Sign the text document before submitting it.
  - Unittests and/or integration tests and/or benchmarks.
