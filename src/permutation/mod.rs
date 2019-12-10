fn _perm2(input: Vec<i64>) ->  Vec<Vec<i64>> {
    let f = input[0];
    let s = input[1];
    vec![vec![f.clone(), s.clone()], vec![s, f]]
}

fn _recursive_perm(mut input: Vec<i64>) -> Vec<Vec<i64>> {
    match input.len() {
        0 => Vec::new(),
        1 => vec![input],
        2 => _perm2(input),
        _ => {
            let first = input.pop().expect("expected atleast 1 element");
            let mut permutations = Vec::new();
            let mut sub_perms = _recursive_perm(input);
            while let Some(sub_perm) = sub_perms.pop() {
                for pos in 0..=sub_perm.len() {
                    let mut perm: Vec<i64> = sub_perm.clone();
                    perm.insert(pos, first.clone());
                    permutations.push(perm);
                }
            }
            permutations
        }
    }
}

pub fn permutations(input: Vec<i64>) -> Vec<Vec<i64>> {
    _recursive_perm(input)
} 