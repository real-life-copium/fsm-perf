# FSM-PERF

I am a big fan of the finite state machine, the first time I got to know this
was learning algorithms for LCS, and after that I become more and more fascinate
about it.

## ABOUT THIS REPO

This repository contains two implementation of validating the given time string
is a valid time or not, for example: "20:04" is valid while "24+7" is not.

And I want to let you know how fast FSM can be.

## TEST OUTPUT

The 1 million strings are generated randomly to feed as test input. And FSM
implementation is about 3X as fast as the common one.

`it_works` is to prove that `fsm` and `common` produce identical results.

```bash
✔ ~/projects/github/fsm-perf [main|✔] 
pseudoc $ cargo test --tests --release -- --show-output
    Finished release [optimized] target(s) in 0.00s
     Running unittests src/lib.rs (target/release/deps/fsm_perf-ce885a8409f4abb3)

running 3 tests
test tests::fsm ... ok
test tests::it_works ... ok
test tests::common ... ok

successes:

---- tests::fsm stdout ----
The test took 26.386528ms to finish.

---- tests::common stdout ----
The test took 74.52521ms to finish.


successes:
    tests::common
    tests::fsm
    tests::it_works

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.20s
```
