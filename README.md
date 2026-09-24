An experimental (possibly optimal) lambda calculus machine
---------------------------------------------------------

In standard interpreters, pure lambda calculus is evaluated as graph reduction
over heap-allocated trees: every application is a heap node, every abstraction
is a pointer, and every beta-step allocates intermediate terms.

The cost is not in the lambda calculus; the cost is in the memory model. When
terms are pointer graphs, evaluation is dominated by cache misses and allocator
pressure.

This machine models reduction as flat array transformations over contiguous
arenas, treating the lambda calculus through Data-Oriented Design.

1. Pointers Are Distances

To a programmer, a pointer is an abstraction. To hardware, a pointer is a
commute: an unpredictable jump across RAM that stalls execution on an L3 or
main-memory miss.

Terms are packed into contiguous 32-bit indices inside typed arenas:

```rust
pub struct Variable { pub index: u32 }
pub struct Lambda   { pub body: TermId }
pub struct Apply    { pub function: TermId, pub argument: TermId }
```

Every node is 4 or 8 bytes. A single 64-byte cache line brings in up to 16
terms in one memory transaction.

Canonical hash-consing deduplicates identical subterms on insertion. Terms
form a Directed Acyclic Graph (DAG) rather than a tree, reducing memory
footprint and turning structural equality into an O(1) index check.

2. The Spine Is a Stack

Normal-order reduction searches for the leftmost, outermost redex. Instead of
recursive calls unwinding the syntax tree, the left spine is walked into a
pre-allocated evaluation stack.

Arguments are consumed in place without constructing transient intermediate
terms. Beta-substitution rewrites arena indices directly, avoiding heap churn
during tight reduction loops.

Pure lambda calculus has no primitive types; data is represented by Church,
Scott, or Boehm encodings.

Under Church encoding, representations can be structurally indistinguishable:

    λf. λx. x

This term is simultaneously `false` and the numeral `0`. The bit-pattern is
identical; the type depends on the evaluation context.

When decoding normal forms back into host values (`--decode`), `--prefer` sets
the inspection precedence:

```sh
$ lambdac examples/basic/zero.lam --decode=church --prefer=number
0

$ lambdac examples/basic/zero.lam --decode=church --prefer=bool
false
```

If the preferred rule fails to match, the decoder falls back to alternative
patterns.

4. Reduction Tracing and Statistics

The tracer instruments reduction steps across rules (α, β, η, δ) with zero
heap allocations per step:

```text
$ lambdac examples/basic/not.lam --decode=church --trace --prefer=bool
(λnot. λfalse. not false) (λb. λt. λf. b f t) (λt. λf. f)
│ β (not ↦ λb. λt. λf. b f t)
↓
(λfalse. (λb. λt. λf. b f t) false) (λt. λf. f)
│ β (false ↦ λt. λf. f)
↓
(λb. λt. λf. b f t) (λt. λf. f)
│ β (b ↦ λt. λf. f)
↓
λt. λf. (λx. λy. y) f t
│ β (t ↦ f)
↓
λt. λf. (λx. x) t
│ β (f ↦ t)
↓
λt. λf. t
true
```

Graph topology and reduction metrics are measured directly via `--stats`:

```text
$ lambdac examples/recursion/factorial.lam --stats
term
  nodes: 98
  depth: 21

eval
  steps:    652
  reduces:  652
  allocs:   691
  time:     380.2µs
```
