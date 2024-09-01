# rust-training

ReplyCodeChallenge 2022 - Rust training - `Less than 1 week for Zero -> Hero`
\
\
_Cool Note: This project can be used to learn any programming language because you just need to place the outputs (with adequate names) in the `outputs/player/` directory and run `cargo test` to check if everything is working correctly._

## Tests

### Basic Outputs

- it contains only numbers and newlines
- for each index i, i ∈ [0, D)
- each index is present in the file only once

### Simulating fight with invariant testing

- {optimisation - _**moved as an option -> wasted command**_} our stamina will always be included in the interval [0, Smax]   (the test must simulate a fight and FAILED if stamina > Smax).
- the score is dynamically calculated during the fight. At the end, if we recalculate everything using the double-sum formula [R = sum from n=0 to N-1 of ( sum from i=0 to min(len(A_n), T-T_n) of A_n[i] )
], we should get the same result.   (To achieve this, we'll need to create a Demon object that records the turn in which each demon was defeated). 
- Ensure the simulation returns exactly each results from the example in the subject.

### Win condition

- Verify if the win condition is hold (see [Player Goal](#player-goal)) `cargo test --test win_condition -- --color always --show-output`

## Logics

### Challenges & How to simulate other players scores to get a fixed goal

The reason for coding this logic is to simulate the ReplyCodeChallenge 2022, where the goal is to find the optimal strategy that yields the highest score. In the real competition, players would have tried different strategies to outscore each other. To replicate this, the code repeatedly shuffles the order of demons and simulates the battle, similar to how players would experiment with different strategies in each round of the game.

The code effectively captures the competitive nature of the challenge by:

- Randomizing the order of demons in each attempt, simulating how a player might test various approaches.
- Running multiple simulations to identify the best strategy, much like players would iteratively refine their strategies in a real competition.

This method ensures that the strategy chosen for each battle is the best among many possible permutations, maximizing the chances of winning one of the five challenges (the generated challenges are stored in `outputs/challenges/`).

### Player goal

The player's goal is to win every single challenge in one run, so they must develop a strong strategy with a good heuristic _(hoping the random challenges aren't too difficult, since in the real competition, players needed to accumulate the scores from each challenge to form a final score. This allowed for the possibility of performing poorly on one configuration but excelling in another)_.

### Player logic (Me)

**Spoiler: This may not be the most optimized algorithm, but that’s not the point. This project is simply meant to learn Rust (zero to hero in less than 1 week).**

The logic implemented to create a heuristic that seems reasonable is based on the following steps:

1. **Choosing the Most Profitable Demon**: 
    - Each turn, the algorithm selects the demon that maximizes the total accumulated points. This decision is made by calculating a `demon_value`, which factors in the demon's fragments earned, the stamina it restores, the stamina cost, and the number of turns it takes to recover. The formula used in `demon_value` prioritizes demons that offer a high return on investment in terms of both stamina and fragments.
    - It is possible to optimize it (not implemented here), the heuristic could considers only the fragments between the current turn and the `max_turn`, ensuring that only relevant points are included in the calculation [only worth for the last few demons].

2. **Handling Inaccessible Demons**:
    - If the player cannot afford to fight any demon (based on current stamina), the algorithm decides to skip the turn and wait for stamina recovery. The `get_affordable_demon` function is responsible for finding the most suitable demon to fight, considering those that have not been used yet and are within the player's stamina capacity.
    - If no affordable demon is available immediately, the algorithm considers future stamina recovery (especially from the last demon fought) to determine if a demon can be fought in subsequent turns.

3. **Fallback Strategy**:
    - If no immediate or future affordable demons are found, the algorithm resorts to picking the cheapest available demon. This ensures that progress continues even if no optimal choice is available.

4. **Optimization for Stamina Recovery Consideration (not implemented here)**: 
    - If no affordable demon is available and the algorithm defaults to waiting for stamina recovery from the last demon fought, there's an opportunity for optimization. Instead of only considering the last demon's stamina recovery, the algorithm could evaluate all demons that contribute to stamina recovery. This way, the player isn't solely reliant on a potentially inefficient recovery strategy, especially if the last demon fought is near the end of the battle and may not provide sufficient stamina in time. By considering the cumulative stamina recovery from all relevant demons, the algorithm can make a more informed decision on whether to wait or take a different strategic action.

5. **Optimization for the heuristic**:
    - The heuristic used is the second version, and it is sufficient to solve the challenges. However, with more testing, by adjusting coefficients or modifying the formula, it might be possible to boost the points earned.

## Usage

Note: The complete generation process for the player takes ~1 hour (but this can be optimized with threading or multiprocessing).

    Usage:
    [command] [options]

    Commands:
    generate                 Generate all outputs
    generate [challenge_id]  Generate a specific challenge output based on the given ID.
    simulate [input path] [output path]
                            Simulate the battle with the given input and output files. Prints the battle state.
    waste [input path] [output path]
                            Evaluate and print the wasted stamina and the associated turn based on the input and output files.
    score [input path] [output path]
                            Simulate the battle and print the final score based on the input and output files.

    Examples:
    cargo run -- generate               # Generate default challenges/outputs
    cargo run -- generate 2             # Generate outputs for the 2nd challenge
    cargo run -- simulate ./inputs/00-example.txt outputs/tests/good_output.txt
                                        # Simulate the battle and print the results
    cargo run -- waste ./inputs/00-example-waste.txt outputs/tests/good_output.txt
                                        # Evaluate and print wasted stamina
    cargo run -- score ./inputs/00-example.txt outputs/tests/good_output.txt
                                        # Simulate the battle and print the final score
