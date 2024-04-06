# Differences between Leafy and the reference implementation

The original, reference implementation has several bugs and semantics issues,
which I fix in Leafy, leaf.leaf, and my [fork of Leaf](https://github.com/thaliaarchi/leaf).
This documents the semantics of my implementations compared to the original.

- `}` should not pop the topmost root, but instead set success to false. The
  original pops it anyway, which makes `^` and `?` fail.
- `-` should not delete the node when at a root, but instead set success to
  false. The original deletes the root anyway and errors when it was the topmost
  root.
- `?` should properly break out of nested loops. The original did not pop the
  loop stack, which makes many of my programs not work as expected in the
  reference interpreter.
- `?` should function as a conditional exit, when it occurs outside of a loop.
  The original repeats the program from the start, usually making it
  non-terminating.
