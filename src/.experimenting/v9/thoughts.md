# Thoughts

## combinator/middleware

- knows what it's receiving (since it wraps the previous combinator)
  - and doesn't need to say what it is (since it already "has" the value)
- knows what it's propagating
  - and needs to say what it is (so other assertions/combinators know if they can be built from it)
- may have additional bounds on next assertion
  - all/any need to be able to clone the next assertion
    - alternatively, all assertions can be executed multiple times, but this is constraining
  - always a bound on next assertion that it receives what the combinator propagates

type params:

- next assertion

assoc types:

- propagated value

## assertion

TODO

- knows what it's receiving
- knows what it's propagating