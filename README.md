An expermintal (possibly optimal) lambda calculus machine
---------------------------------------------------------

The goal is simple, make pure lambda reduction fast by treating terms as flat
data rather than heap-allocated trees. No pointers, no string-based variables,
and no intermediate allocations during evaluation.

References are compact 32-bit integers: 2 bits for node kind, 30 bits for arena
offset.

```rust
pub struct Variable {
  pub index: u32,       // 4 bytes
}

pub struct Lambda {
  pub body: TermId,     // 4 bytes
}

pub struct Apply {
  pub function: TermId, // 4 bytes
  pub argument: TermId, // 4 bytes
}
```

Every node is either 4 or 8 bytes. A single 64-byte cache line holds 8 `Apply`
or 16 `Lambda` nodes. Identical nodes are deduplicated via canonical
hash-consing on insertion.

Spine Evaluation
----------------

Normal-order reduction uses a pre-allocated stack (`Vec<TermId>`) to unwind
the left spine in a tight loop. Arguments are popped and substituted in place
without allocating intermediate application trees.

Usage
-----

```sh
cargo build --release
```

Run and decode Church encodings:

```sh
$ lambdac factorial.lam --decode=church
6

$ lambdac arithmetic.lam --decode=church
7
```

Trace beta reductions:

```hs
$ lambdac identity.lam --trace
(λx. λz. x z) (λx. x) y -> (λx. (λz. z) x) y
(λx. (λz. z) x) y -> (λx. x) y
(λx. x) y -> y
y
```

Execution statistics:

```text
$ lambdac factorial.lam --stats
term
  nodes: 98
  depth: 21

eval
  steps:    652
  reduces:  652
  allocs:   691
  time:     380.2µs
```
