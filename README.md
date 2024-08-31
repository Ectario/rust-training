# rust-training

ReplyCodeChallenge 2022 - Rust training


## Tests

### Basic Outputs

- it contains only numbers and newlines
- for each index i, i ∈ [0, D)
- each index is present in the file only once

### Simulating fight with invariant testing

- {optimisation - _**moved as an option**_} our stamina will always be included in the interval [0, Smax]   (the test must simulate a fight and FAILED if stamina > Smax)
- the score is dynamically calculated during the fight. At the end, if we recalculate everything using the double-sum formula [R = sum from n=0 to N-1 of ( sum from i=0 to min(len(A_n), T-T_n) of A_n[i] )
], we should get the same result.   (To achieve this, we'll need to create a Demon object that records the turn in which each demon was defeated) 

## Logics

### Simulate other players scores to get a fixed goal

The reason for coding this logic is to simulate the ReplyCodeChallenge 2022, where the goal is to find the optimal strategy that yields the highest score. In the real competition, players would have tried different strategies to outscore each other. To replicate this, the code repeatedly shuffles the order of demons and simulates the battle, similar to how players would experiment with different strategies in each round of the game.

The code effectively captures the competitive nature of the challenge by:

- Randomizing the order of demons in each attempt, simulating how a player might test various approaches.
- Running multiple simulations to identify the best strategy, much like players would iteratively refine their strategies in a real competition.

This method ensures that the strategy chosen for each battle is the best among many possible permutations, maximizing the chances of winning one of the five challenges (the generated challenges are stored in `outputs/challenges/`).

The player's goal is to win every single challenge in one run, so they must develop a strong strategy with a good heuristic _(hoping the random challenges aren't too difficult, since in the real competition, players needed to accumulate the scores from each challenge to form a final score. This allowed for the possibility of performing poorly on one configuration but excelling in another)_.

### Player logic (Me)

**Spoiler: This may not be the most optimized algorithm, but that’s not the point. This project is simply meant to learn Rust (zero to hero in 1 week).**

The logic implemented to create a heuristic that seems reasonable is as follows:

Each turn, you need to choose the most profitable demon based on the total accumulated points (this can be optimized by considering only the points between the current turn and the max_turn) that you can afford. 
If you can’t afford any, you skip the turn (the goal being to wait to receive more stamina). [Another immediate optimization is highlight the stamina the demon restores—the more stamina and points it offers, the more valuable it becomes].