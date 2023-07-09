# Differences between Leafy and the reference implementation

The original, reference implementation has several bugs, that I fix in Leafy and
leaf.leaf. This documents how they differ and the semantics I used.

- `}` will not pop the topmost root and instead sets success to false. The
  original pops it anyway, which makes `^` and `?` fail.
- `-` will not delete the node when at a root and instead sets success to false.
  The original deletes the root anyway and errors when it was the topmost root.
- `?` properly breaks out of nested loops. The original did not pop the loop
  stack, which makes many of my programs not work as expected in the reference
  interpreter.
- `?` conditionally exits the program, when it occurs outside of a loop. The
  original repeats the program from the start, usually making it
  non-terminating.
