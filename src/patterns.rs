use rand::{rng, Rng};
use predefined::{full_pairs, all_patterns};

//                     Top           Down
#[derive(Eq, PartialEq, Clone, Copy)]
pub struct Pattern(pub (bool, bool), pub (bool, bool));

fn is_full(patterns: (Pattern, Pattern)) -> bool {
    for p in full_pairs {
        if patterns.1 == p.1 && patterns.0 == p.0 {
            return true;
        }
        if patterns.1 == p.0 && patterns.0 == p.1 {
            return true;
        }
    }
    false
}

pub fn get_full() -> (Pattern, Pattern) {
    let mut thread_rng = rng();
    let idx = thread_rng.random_range(0..full_pairs.len()) as usize;
    if thread_rng.random_bool(0.5) {
        full_pairs[idx]
    } else {
        (full_pairs[idx].1, full_pairs[idx].0)
    }
}

pub fn get_empty() -> (Pattern, Pattern) {
    let mut thread_rng = rng();
    loop {
        let idx1 = thread_rng.random_range(0..all_patterns.len()) as usize;
        let idx2 = thread_rng.random_range(0..all_patterns.len()) as usize;
        if !is_full((all_patterns[idx1], all_patterns[idx2])) {
            return (all_patterns[idx1], all_patterns[idx2])
        }
    }
}

#[allow(non_upper_case_globals)]
pub mod predefined {
    use crate::patterns::Pattern;

    const left: Pattern = Pattern((true, false), (true, false));
    const right: Pattern = Pattern((false, true), (false, true));
    const bottom: Pattern = Pattern((false, false), (true, true));
    const top: Pattern = Pattern((true, true), (false, false));
    const checker1: Pattern = Pattern((true, false), (false, true));
    const checker2: Pattern = Pattern((false, true), (true, false));

    pub const all_patterns: [Pattern; 6] = [
        left,
        right,
        bottom,
        top,
        checker1,
        checker2,
    ];
    pub const full_pairs: [(Pattern, Pattern); 3] = [
        (left, right),
        (top, bottom),
        (checker1, checker2),
    ];
}
