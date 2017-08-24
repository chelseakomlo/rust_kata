## Rust code kata for Tor Dev

### Purpose

To lwarn non-trivial Rust, fight with the borrow checker, and see some nice
qualities of Rust and how that applies to tor.

### The exercise

1. Fetch the latest consensus

2. Build a program that will count the hamming distance between relay names to
   see if anyone is spamming the network with poorly-named relays.

How you can do this:
  - Filter the consensus for r (router) lines
  - Extract the router name from each line
  - For each router name, compare it to all of the other router names using
    hamming distance. For names which are < 3 distance, this is considered a
    hit.
  - Return a list of pairs, where the first element in the pair is the number
    of hits, and the second is the matching router names.

Bonus:
  - Filter the top 5 names with the highest number of hits
