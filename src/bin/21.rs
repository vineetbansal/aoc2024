use std::collections::HashMap;

fn find_paths(seq: &str) -> Vec<String> {
    // origin is bottom-left
    let positions: HashMap<char, (i32, i32)> = [
        ('0', (1, 0)), ('A', (2, 0)), ('1', (0, 1)),
        ('2', (1, 1)), ('3', (2, 1)), ('4', (0, 2)),
        ('5', (1, 2)), ('6', (2, 2)), ('7', (0, 3)),
        ('8', (1, 3)), ('9', (2, 3))
    ].iter().cloned().collect();

    let mut full_paths = vec![String::new()];
    let seq = format!("A{}", seq);

    for i in 0..seq.len() - 1 {
        let src = seq.chars().nth(i).unwrap();
        let tgt = seq.chars().nth(i + 1).unwrap();
        let (src_x, src_y) = positions[&src];
        let (tgt_x, tgt_y) = positions[&tgt];

        // We only consider paths with either all horiz or all verts grouped
        let horiz = if tgt_x > src_x {
            ">".repeat((tgt_x - src_x) as usize)
        } else {
            "<".repeat((src_x - tgt_x) as usize)
        };

        let vert = if tgt_y > src_y {
            "^".repeat((tgt_y - src_y) as usize)
        } else {
            "v".repeat((src_y - tgt_y) as usize)
        };

        let paths = if (src == '0' || src == 'A') && ["1", "4", "7"].contains(&tgt.to_string().as_str()) {
            // can't go horizontal first
            vec![format!("{}{}", vert, horiz)]
        } else if ["0", "A"].contains(&tgt.to_string().as_str()) && ["1", "4", "7"].contains(&src.to_string().as_str()) {
            // can't go vertical first
            vec![format!("{}{}", horiz, vert)]
        } else {
            // consider both paths, but only if we should (important!)
            match (horiz.is_empty(), vert.is_empty()) {
                (false, false) => vec![format!("{}{}", vert, horiz), format!("{}{}", horiz, vert)],
                (false, true) => vec![horiz],
                (true, false) => vec![vert],
                _ => vec![],
            }
        };

        // add newly-found paths to the current possible paths
        full_paths = full_paths.into_iter().flat_map(|full_path| {
            paths.iter().map(|path| format!("{}{}A", full_path, path)).collect::<Vec<_>>()
        }).collect();
    }

    full_paths
}

fn update_transition_counts_from_keystrokes(transition_counts: &mut HashMap<String, i64>, keystrokes: &str, inc_by: i64) {
    /*
    Update the transition_counts HashMap with the number of times each transition
    occurs in the given keystrokes.
    `inc_by` would normally be 1, but can be more if we wish to make bulk updates for a transition.
    */
    for (i, to_) in keystrokes.chars().enumerate() {
        let from_ = if i == 0 { 'A' } else { keystrokes.chars().nth(i - 1).unwrap() };
        let key = format!("{}{}", from_, to_);
        *transition_counts.entry(key).or_insert(0) += inc_by;
    }
}

fn update_transition_counts(transition_counts: &HashMap<String, i64>, transitions: &HashMap<String, &str>) -> (HashMap<String, i64>, i64) {
    /*
    Update the transition_counts HashMap as it would look after one additional pass,
    i.e. after each of the transitions seen in the HashMap are applied.

    Return the new transition_counts and the number of keystrokes needed to get there.
     */
    let mut new_transitions: HashMap<String, i64> = transitions.keys().map(|k| (k.clone(), 0)).collect();
    let mut n_keystrokes = 0;

    for (from_to, &count) in transition_counts.iter() {
        if count != 0 {
            if let Some(keystrokes) = transitions.get(from_to) {
                let keystrokes_with_a = format!("{}A", keystrokes);
                n_keystrokes += keystrokes_with_a.len() as i64 * count;
                update_transition_counts_from_keystrokes(&mut new_transitions, &keystrokes_with_a, count);
            }
        }
    }

    (new_transitions, n_keystrokes)
}

fn complexity_sum(outputs: Vec<&str>, n_robots: u8) -> i64 {
    /*
    For the directional keypad, what does it take to go from the 0th to the 1st
    char in each of the keys below? We prefer repeated keystrokes if possible, and avoid going
    out of bounds.

    The rest of the cases, e.g v> where >v would have worked just as well, I started with
    an arbitrary decision, and modified it if I saw a reduction in the final result.
    We could have cycled through all permutations in those cases I suppose.

    The trailing 'A' in each of these transitions is implied.
     */
    let transitions: HashMap<String, &str> = [
        ("AA", ""), ("A^", "<"), ("Av", "<v"), ("A<", "v<<"), ("A>", "v"),
        ("^A", ">"), ("^^", ""), ("^v", "v"), ("^<", "v<"), ("^>", "v>"),
        ("vA", "^>"), ("v^", "^"), ("vv", ""), ("v<", "<"), ("v>", ">"),
        ("<A", ">>^"), ("<^", ">^"), ("<v", ">"), ("<<", ""), ("<>", ">>"),
        (">A", "^"), (">^", "<^"), (">v", "<"), ("><", "<<"), (">>", "")
    ].iter().map(|(k, v)| (k.to_string(), *v)).collect();

    let mut result = 0;

    for output in outputs {
        let possible_initial_keystrokes = find_paths(output);
        let mut min_keystrokes = i64::MAX;

        for initial_keystrokes in possible_initial_keystrokes {

            // Inspiration from Day 11 - we only need to store the counts of each transition type,
            // not the transitions themselves as a huge string that we then inspect
            // (where they occur is immaterial since we always end up at the 'A' after each transition)

            // initial population of transition_counts to 0
            let mut transition_counts: HashMap<String, i64> = transitions.keys().map(|k| (k.clone(), 0)).collect();
            update_transition_counts_from_keystrokes(&mut transition_counts, &initial_keystrokes, 1);

            let mut total_keystrokes = 0;
            for _ in 0..n_robots {
                let (new_counts, n_keystrokes) = update_transition_counts(&transition_counts, &transitions);
                transition_counts = new_counts;
                total_keystrokes = n_keystrokes;
            }

            min_keystrokes = min_keystrokes.min(total_keystrokes);
        }

        let complexity = output[..output.len() - 1].parse::<i64>().unwrap() * min_keystrokes;
        result += complexity;
    }

    result
}

fn solution_a(input: &str) -> i64 {
    let outputs: Vec<&str> = input.lines().collect();
    complexity_sum(outputs, 2)
}

fn solution_b(input: &str) -> i64 {
    let outputs = input.lines().collect();
    complexity_sum(outputs, 25)
}

fn main() {
    aoc2024::run("21", solution_a, solution_b);
}