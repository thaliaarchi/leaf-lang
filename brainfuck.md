# Brainfuck in Leaf

## Initialize finite cells

```leaf
+*>+*>+*>+*> 29,999 times +*>+*>+*>+*>
+(^)<{
```

## `>` Move right (bounded)

```leaf
}^><{
```

## `<` Move left (bounded)

```leaf
}^^<{
```

## `+` Increment cell (non-wrapping)

```leaf
(>)*    Move to the cell tail and add a leaf
(^)     Return to the cell head
```

```leaf
(>)*(^)
```

## `-` Decrement cell (non-wrapping)

```leaf
(>)     Move to the cell tail
(?      If the value is non-zero,
  -(^)  remove the tail leaf and return to the cell head
)
```

```leaf
(>)(?-(^))
```

## `+` Increment cell (wrapping)

```leaf
(>)*    Move to the cell tail and add a leaf
        Move up at most 254 places
^^^^^^^^ ^^^^^^^^ ^^^^^^^^ ^^^^^^^^
^^^^^^^^ ^^^^^^^^ ^^^^^^^^ ^^^^^^^^
^^^^^^^^ ^^^^^^^^ ^^^^^^^^ ^^^^^^^^
^^^^^^^^ ^^^^^^^^ ^^^^^^^^ ^^^^^^^^
^^^^^^^^ ^^^^^^^^ ^^^^^^^^ ^^^^^^^^
^^^^^^^^ ^^^^^^^^ ^^^^^^^^ ^^^^^^^^
^^^^^^^^ ^^^^^^^^ ^^^^^^^^ ^^^^^^^^
^^^^^^^^ ^^^^^^^^ ^^^^^^^^ ^^^^^^
(?      If the new value is greater than 255 (i.e., 256),
  -     delete the value to set it to 0
)
```

```leaf
(>)*^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^(?-)
```

## `-` Decrement cell (wrapping)

```leaf
(>)+    Move to the cell tail and add a "flag" left branch
(?      If the value is non-zero,
  -(^)  remove the leaf and flag and return to the root
)
<(?     If the flag is present,
  -     remove the flag,
        set the value to 255,
  *>*>*>*>*>*>*>*> *>*>*>*>*>*>*>*> *>*>*>*>*>*>*>*> *>*>*>*>*>*>*>*>
  *>*>*>*>*>*>*>*> *>*>*>*>*>*>*>*> *>*>*>*>*>*>*>*> *>*>*>*>*>*>*>*>
  *>*>*>*>*>*>*>*> *>*>*>*>*>*>*>*> *>*>*>*>*>*>*>*> *>*>*>*>*>*>*>*>
  *>*>*>*>*>*>*>*> *>*>*>*>*>*>*>*> *>*>*>*>*>*>*>*> *>*>*>*>*>*>*>*>
  *>*>*>*>*>*>*>*> *>*>*>*>*>*>*>*> *>*>*>*>*>*>*>*> *>*>*>*>*>*>*>*>
  *>*>*>*>*>*>*>*> *>*>*>*>*>*>*>*> *>*>*>*>*>*>*>*> *>*>*>*>*>*>*>*>
  *>*>*>*>*>*>*>*> *>*>*>*>*>*>*>*> *>*>*>*>*>*>*>*> *>*>*>*>*>*>*>*>
  *>*>*>*>*>*>*>*> *>*>*>*>*>*>*>*> *>*>*>*>*>*>*>*> *>*>*>*>*>*>*>
  (^)   and return to the cell head
)
```

```leaf
(>)+(?-(^))<(?-*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>*>(^))
```

## `[ … ]` Loop

```leaf
(>?^    While the value is non-zero
…       Execute the loop body
)       Repeat
```

```leaf
(>?^ … )
```

## Header comment

```leaf
(? … )
```