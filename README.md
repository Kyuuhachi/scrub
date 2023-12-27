# scrub

A macro for letting macros absolve themselves of guilt, blaming any and all errors on their callers.

For example, consider a macro that has as its contract that the input must be an expression of a specific type.
If a caller breaks this contract, a part of the macro body is highlighted as the error cause, even though it *obviously* was the caller's fault.

```
error[E0308]: mismatched types
 --> examples/blame.rs:3:7
  |
3 |         let () = $e;
  |             ^^ expected `A`, found `()`
...
9 |     b!(A);
  |     -----
  |     |  |
  |     |  this expression has type `A`
  |     in this macro invocation
```

Now you can finally tell those callers whose fault it *really* is.

```
error[E0308]: mismatched types
 --> examples/blameless.rs:9:2
  |
9 |     b!(A);
  |     ^^^-^
  |     |  |
  |     |  this expression has type `A`
  |     expected `A`, found `()`
```

Due to using the `proc_macro_span` feature, this crate requires nightly.
