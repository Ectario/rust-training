# rust-training

ReplyCodeChallenge 2022 - Rust training


## Tests

### Basic Outputs

- it contains only numbers and newlines
- for each index i, i ∈ [0, D)
- each index is present in the file only once

### Simulating fight with invariant testing

- {optimisation} our stamina will always be included in the interval [0, Smax]   (the test must simulate a fight and FAILED if stamina > Smax)
- the score is dynamically calculated during the fight. At the end, if we recalculate everything using the double-sum formula [R = sum from n=0 to N-1 of ( sum from i=0 to min(len(A_n), T-T_n)-1 of A_n[i] )
], we should get the same result.   (To achieve this, we'll need to create a Demon object that records the turn in which each demon was defeated) 