# Leafy

A fast implementation of the Leaf esoteric programming language in Rust and a
self-interpreter.

[Leaf](https://github.com/CRogers/leaf) is a programming language, created by
Callum Rogers for the PLT Games December 2012 competition [“Into the Tarpit”](https://web.archive.org/web/20140910193725/http://www.pltgames.com/competition/2012/12).
It's oriented around a single binary tree and provides a small set of operations
for manipulating it. It's like Brainfuck in its simplicity, but instead of using
data on a linear tape, it uses data-less nodes in a binary tree with movable
roots. I think that Leaf is a hidden gem, that had since been forgotten, and I
hope that this project can revitalize it.

I've proven its Turing-completeness by devising a [correspondence](brainfuck.md)
between Leaf and Brainfuck and demonstrated its power by writing a
self-interpreter, [leaf.leaf](programs/leaf.leaf).

Leafy implements the central tree data structure as a vector of nodes,
referenced by indices, with a built-in free list. Since nodes are frequently
created and freed, this gives efficient allocation with good locality. The
technique was inspired by arenas for graphs.

To learn Leaf, read the original [tutorial](https://crogers.github.io/leaf/tutorial.html)
or try programs interactively in the [playground](https://crogers.github.io/leaf/default.htm).
Additionally, I've written a list of [idioms](idioms.md), that may be useful
when writing programs in Leaf. Note, however, that some of the programs here
will not run correctly in the original playground, because Leaf has some
implementation and semantics [bugs](differences.md). As such, I'd recommend
using [my fork of Leaf](https://github.com/thaliaarchi/leaf) and its
[playground](https://thaliaarchi.github.io/leaf/playground.html) or Leafy.
