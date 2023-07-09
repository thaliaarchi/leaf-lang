# Leaf idioms

- Header comment: `(? … )`
- Block comment: `{(? … )}`
- Delete left child: `+<-`
- Delete right child: `*>-`
- Test left child: `{<(? …)`
- Test right child: `{>(? …)`
- Unconditionally break from loop:
  - `(? … {)`
  - `(… {?)`
- Unconditionally exit loop:
  - `(… {^)`
  - `(… (^)^)`
  - `(… (<)<)`
  - `(… (>)>)`
  - `(… (-)-)` (only in Leafy; see [differences](differences.md))
  - `(… (})})` (only in Leafy; see [differences](differences.md))
