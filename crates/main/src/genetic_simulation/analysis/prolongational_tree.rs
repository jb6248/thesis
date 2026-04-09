use crate::distance::pitch_class_space::PitchClassSpace;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use display_tree::DisplayTree;

/// Prolongational Tree
#[derive(Debug, DisplayTree)]
pub enum Tree<T> {
    Leaf(#[ignore_field] T),
    Tensing(#[tree] Box<Tree<T>>, #[tree] Box<Tree<T>>, Strength),
    Relaxing(#[tree] Box<Tree<T>>, #[tree] Box<Tree<T>>, Strength),
}

#[derive(Debug)]
pub enum Strength {
    Strong,
    Weak,
    Progression,
}

impl Display for Strength {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Strength::Strong => write!(f, "strong"),
            Strength::Weak => write!(f, "weak"),
            Strength::Progression => write!(f, "progression"),
        }
    }
}

/// Find the path from the root to the most distant chord, minimizing the total distance traveled.
/// Each chord is either selected or not. The total distance traveled is the sum of the distances between consecutive selected chords, starting with the first chord and ending with the last chord.
pub fn get_min_dist_path(chords: &Vec<PitchClassSpace>) -> Vec<usize> {
    // find the path from the root to the most distant chord, minimizing the total distance traveled
    if chords.len() <= 2 {
        // base case: if there are 2 or fewer chords, just return the first and last chord
        return (0..chords.len()).collect();
    }

    // memoize the results
    type Dist = usize;
    type Best = Option<usize>;
    let mut memo: HashMap<usize, (Dist, Best)> = HashMap::new();

    // Returns minimum distance from index to the end and updates memo
    fn helper(
        index: usize,
        chords: &Vec<PitchClassSpace>,
        memo: &mut HashMap<usize, (Dist, Best)>,
    ) -> (Dist, Best) {
        if let Some(result) = memo.get(&index) {
            return *result;
        }

        if index == chords.len() - 1 {
            // base case: if we are at the end, the distance is 0 and there is no best index
            memo.insert(index, (0, None));
            return (0, None);
        }

        let mut best_dist = usize::MAX;
        let mut best_index = 0;

        for next_index in index + 1..chords.len() {
            let dist = chords[index].total_distance(&chords[next_index]);
            let (next_dist, _) = helper(next_index, chords, memo);
            let total_dist = dist + next_dist;
            if total_dist < best_dist {
                best_dist = total_dist;
                best_index = next_index;
            }
        }

        let result = (best_dist, Some(best_index));
        memo.insert(index, result);
        result
    }

    helper(0, chords, &mut memo);

    // now reconstruct path from memo
    let mut path = Vec::new();
    let mut index = 0;
    path.push(index);
    while let Some((_, Some(next_index))) = memo.get(&index) {
        index = *next_index;
        path.push(index);
    }
    assert_eq!(
        path[path.len() - 1],
        chords.len() - 1,
        "last chord must be included in path: {:?}", path
    );
    path
}

impl<T> Tree<T> {
    pub fn count_tensing(&self) -> usize {
        match self {
            Tree::Leaf(_) => 0,
            Tree::Tensing(left, right, _) => 1 + left.count_tensing() + right.count_tensing(),
            Tree::Relaxing(left, right, _) => left.count_tensing() + right.count_tensing(),
        }
    }

    pub fn count_relaxing(&self) -> usize {
        match self {
            Tree::Leaf(_) => 0,
            Tree::Tensing(left, right, _) => left.count_relaxing() + right.count_relaxing(),
            Tree::Relaxing(left, right, _) => 1 + left.count_relaxing() + right.count_relaxing(),
        }
    }

    pub fn count_branchings(&self, tensing: bool) -> usize {
        if tensing {
            self.count_tensing()
        } else {
            self.count_relaxing()
        }
    }

    pub fn count_leaves(&self) -> usize {
        match self {
            Tree::Leaf(_) => 1,
            Tree::Tensing(left, right, _) | Tree::Relaxing(left, right, _) => {
                left.count_leaves() + right.count_leaves()
            }
        }
    }
}

impl<T: Clone> Tree<T> {
    pub fn pop_last(&mut self) -> Option<T> {
        match self {
            Tree::Leaf(_) => None,
            Tree::Tensing(_, right, _) | Tree::Relaxing(_, right, _) => {
                if !matches!(right.as_ref(), Tree::Leaf(_)) {
                    return right.pop_last();
                }
                // right is a leaf: extract its value and collapse self to left
                let val = match right.as_ref() {
                    Tree::Leaf(v) => v.clone(),
                    _ => unreachable!(),
                };
                // Replace self with a dummy so we can take ownership of left
                let old = std::mem::replace(self, Tree::Leaf(val.clone()));
                *self = match old {
                    Tree::Tensing(left, _, _) | Tree::Relaxing(left, _, _) => *left,
                    _ => unreachable!(),
                };
                Some(val)
            }
        }
    }

    pub fn pop_first(&mut self) -> Option<T> {
        match self {
            Tree::Leaf(_) => None,
            Tree::Tensing(left, _, _) | Tree::Relaxing(left, _, _) => {
                if !matches!(left.as_ref(), Tree::Leaf(_)) {
                    return left.pop_first();
                }
                // left is a leaf: extract its value and collapse self to right
                let val = match left.as_ref() {
                    Tree::Leaf(v) => v.clone(),
                    _ => unreachable!(),
                };
                // Replace self with a dummy so we can take ownership of right
                let old = std::mem::replace(self, Tree::Leaf(val.clone()));
                *self = match old {
                    Tree::Tensing(_, right, _) | Tree::Relaxing(_, right, _) => *right,
                    _ => unreachable!(),
                };
                Some(val)
            }
        }
    }
}

fn join_with_tensing<T>(
    root: &PitchClassSpace,
    left: (Tree<T>, &PitchClassSpace),
    right: (Tree<T>, &PitchClassSpace),
) -> Tree<T> {
    let left_dist = root.total_distance(left.1);
    let right_dist = root.total_distance(right.1);
    if right_dist > left_dist {
        // tensing
        Tree::Tensing(Box::new(left.0), Box::new(right.0), Strength::Progression)
    } else if right_dist < left_dist {
        // relaxing
        Tree::Relaxing(Box::new(left.0), Box::new(right.0), Strength::Progression)
    } else {
        // strong prolongation
        Tree::Tensing(Box::new(left.0), Box::new(right.0), Strength::Strong)
    }
}

/// Take the non-sequence chords and build a prolongational sub-tree that relates them to the left and right sequence chords.
/// The tree will include the left and right chords.
pub fn build_inter_sequence_prolongational_tree(
    tensing: bool,
    root: &PitchClassSpace,
    left_chord: &PitchClassSpace,
    inter_chords: &Vec<PitchClassSpace>,
    right_chord: &PitchClassSpace,
) -> Tree<usize> {
    if inter_chords.is_empty() {
        // if there are no inter-chords, just return the branching for the two chords
        return join_with_tensing(
            root,
            (Tree::Leaf(0), left_chord),
            (Tree::Leaf(0), right_chord),
        );
    }
    // try every pivot and see which one makes the most sense based on the tensing
    let mut best_count = usize::MAX;
    let mut best_tree = Tree::Leaf(0);
    for i in 0..inter_chords.len() {
        let is_first = i == 0;
        let is_last = i == inter_chords.len() - 1;

        let pivot = &inter_chords[i];
        let left_dist = left_chord.total_distance(pivot);
        let right_dist = right_chord.total_distance(pivot);
        if left_dist < right_dist {
            // recursive for (left, pivot) and (pivot + 1, right)
            let left_tree = build_inter_sequence_prolongational_tree(
                tensing,
                root,
                left_chord,
                &inter_chords[0..i].to_vec(),
                pivot,
            );
            let right_tree = if is_last {
                Tree::Leaf(0)
            } else {
                build_inter_sequence_prolongational_tree(
                    tensing,
                    root,
                    &inter_chords[i + 1],
                    &inter_chords[i + 2..].to_vec(),
                    right_chord,
                )
            };
            let tree = join_with_tensing(root, (left_tree, left_chord), (right_tree, right_chord));
            let count = tree.count_branchings(tensing);
            if count < best_count {
                best_count = count;
                best_tree = tree;
            }
        } else {
            // recursive for (left, pivot) and (pivot + 1, right)
            let left_tree = if is_first {
                Tree::Leaf(0)
            } else {
                build_inter_sequence_prolongational_tree(
                    tensing,
                    root,
                    left_chord,
                    &inter_chords[0..i - 1].to_vec(),
                    &inter_chords[i - 1],
                )
            };
            let right_tree = build_inter_sequence_prolongational_tree(
                tensing,
                root,
                pivot,
                &inter_chords[i + 1..].to_vec(),
                right_chord,
            );
            let tree = join_with_tensing(root, (left_tree, left_chord), (right_tree, right_chord));
            let count = tree.count_branchings(tensing);
            if count < best_count {
                best_count = count;
                best_tree = tree;
            }
        }
    }
    best_tree
}

/// Build a full prolongational tree for the entire chord sequence, with the tonal center as root.
/// This should find the most stable interpretation of the chord sequence, then build subtrees for the remaining
/// chords between the sequence chords.
pub fn build_full_prolongational_tree(
    chords: &Vec<PitchClassSpace>,
    root: &PitchClassSpace,
    tensing: bool,
) -> Option<Tree<usize>> {
    if chords.len() == 0 {
        return None;
    }
    if chords.len() == 1 {
        return Some(Tree::Leaf(0));
    }
    let path = get_min_dist_path(chords);
    let mut tree = None;
    for i in 0..path.len() - 1 {
        let left_index = path[i];
        let right_index = path[i + 1];
        let inter_chords = chords[left_index + 1..right_index].to_vec();
        let mut subtree = build_inter_sequence_prolongational_tree(
            tensing,
            root,
            &chords[left_index],
            &inter_chords,
            &chords[right_index],
        );
        if let Some(existing) = tree.take() {
            subtree.pop_first(); // remove the leftmost leaf to avoid duplication
            tree = Some(join_with_tensing(
                root,
                (existing, &chords[left_index]),
                (subtree, &chords[right_index]),
            ));
        } else {
            tree = Some(subtree);
        }
    }
    tree
}

pub fn find_best_tension_relaxation_split_and_score(
    chords: &Vec<PitchClassSpace>,
    root: &PitchClassSpace,
) -> (usize, Tree<usize>, Tree<usize>) {
    if chords.len() < 4 {
        return (0, Tree::Leaf(0), Tree::Leaf(3));
    }
    let mut best_score = 0;
    let mut best_sol = (Tree::Leaf(0), Tree::Leaf(0));
    for split in 2..chords.len() - 1 {
        // where `split` represents the beginning of the relaxing
        let left_chords = &chords[0..split].to_vec();
        let right_chords = &chords[split..].to_vec();
        let left_tree = build_full_prolongational_tree(left_chords, root, true);
        let right_tree = build_full_prolongational_tree(right_chords, root, false);
        let total = left_tree.as_ref().map(|t| t.count_tensing()).unwrap_or(0)
            + right_tree.as_ref().map(|t| t.count_relaxing()).unwrap_or(0);
        if total > best_score {
            best_score = total;
            best_sol = (left_tree.unwrap_or(Tree::Leaf(0)), right_tree.unwrap_or(Tree::Leaf(0)));
        }
    }
    (best_score, best_sol.0, best_sol.1)
}
#[cfg(test)]
mod test {
    use super::*;
    use crate::distance::pitch_class_space::SpaceLevel;

    fn chords_from_diatonic_offsets(offsets: &[isize]) -> Vec<PitchClassSpace> {
        offsets
            .iter()
            .map(|&o| {
                let mut c = PitchClassSpace::c_maj();
                c.rotate_on_level(SpaceLevel::Diatonic, o);
                c
            })
            .collect()
    }

    /// Helper: assert a path is valid for the given chord count.
    fn assert_valid_path(path: &Vec<usize>, n: usize) {
        assert!(!path.is_empty(), "path must not be empty");
        assert_eq!(path[0], 0, "path must start at 0");
        assert_eq!(path[path.len() - 1], n - 1, "path must end at last index");
        // strictly increasing
        for w in path.windows(2) {
            assert!(w[0] < w[1], "path must be strictly increasing: {:?}", path);
        }
    }

    #[test]
    fn get_min_dist_path_one_chord() {
        let chords = vec![PitchClassSpace::c_maj()];
        let path = get_min_dist_path(&chords);
        assert_valid_path(&path, 1);
        assert_eq!(path, vec![0]);
    }

    #[test]
    fn get_min_dist_path_two_chords() {
        let chords = chords_from_diatonic_offsets(&[0, 4]);
        let path = get_min_dist_path(&chords);
        assert_valid_path(&path, 2);
        assert_eq!(path, vec![0, 1]);
    }

    #[test]
    fn get_min_dist_path_three_chords_valid() {
        // Three chords: must start at 0, end at 2, be strictly increasing.
        let chords = chords_from_diatonic_offsets(&[0, 4, 1]);
        let path = get_min_dist_path(&chords);
        assert_valid_path(&path, 3);
    }

    #[test]
    fn get_min_dist_path_four_chords_valid() {
        let chords = chords_from_diatonic_offsets(&[0, 4, 1, 5]);
        let path = get_min_dist_path(&chords);
        assert_valid_path(&path, 4);
    }

    #[test]
    fn get_min_dist_path_five_chords_valid() {
        let chords = chords_from_diatonic_offsets(&[0, 3, 1, 5, 2]);
        let path = get_min_dist_path(&chords);
        assert_valid_path(&path, 5);
    }

    /// The chosen path must have total distance ≤ every other valid path.
    #[test]
    fn get_min_dist_path_is_actually_minimum() {
        let chords = chords_from_diatonic_offsets(&[0, 3, 1, 5, 2]);
        let n = chords.len();
        let path = get_min_dist_path(&chords);
        assert_valid_path(&path, n);

        let path_dist: usize = path
            .windows(2)
            .map(|w| chords[w[0]].total_distance(&chords[w[1]]))
            .sum();

        // Brute-force all subsets that include 0 and n-1
        let mut best = usize::MAX;
        for mask in 0u32..(1u32 << n) {
            if mask & 1 == 0 || mask & (1 << (n - 1)) == 0 {
                continue;
            }
            let subset: Vec<usize> = (0..n).filter(|&i| mask & (1 << i) != 0).collect();
            let d: usize = subset
                .windows(2)
                .map(|w| chords[w[0]].total_distance(&chords[w[1]]))
                .sum();
            if d < best {
                best = d;
            }
        }
        assert_eq!(
            path_dist, best,
            "path distance {} is not the minimum {}; path: {:?}",
            path_dist, best, path
        );
    }


    fn leaf(v: i32) -> Tree<i32> {
        Tree::Leaf(v)
    }

    fn tensing(left: Tree<i32>, right: Tree<i32>) -> Tree<i32> {
        Tree::Tensing(Box::new(left), Box::new(right), Strength::Weak)
    }

    fn relaxing(left: Tree<i32>, right: Tree<i32>) -> Tree<i32> {
        Tree::Relaxing(Box::new(left), Box::new(right), Strength::Weak)
    }

    fn collect_leaves(tree: &Tree<i32>) -> Vec<i32> {
        match tree {
            Tree::Leaf(v) => vec![*v],
            Tree::Tensing(l, r, _) | Tree::Relaxing(l, r, _) => {
                let mut v = collect_leaves(l);
                v.extend(collect_leaves(r));
                v
            }
        }
    }

    #[test]
    fn pop_last_on_leaf_returns_none() {
        let mut t = leaf(42);
        assert_eq!(t.pop_last(), None);
        // tree unchanged
        assert_eq!(collect_leaves(&t), vec![42]);
    }

    #[test]
    fn pop_last_two_node_tree() {
        // tensing(1, 2) → pops 2, tree becomes leaf(1)
        let mut t = tensing(leaf(1), leaf(2));
        assert_eq!(t.pop_last(), Some(2));
        assert_eq!(collect_leaves(&t), vec![1]);
    }

    #[test]
    fn pop_last_two_node_relaxing() {
        let mut t = relaxing(leaf(10), leaf(20));
        assert_eq!(t.pop_last(), Some(20));
        assert_eq!(collect_leaves(&t), vec![10]);
    }

    #[test]
    fn pop_last_deep_right_chain() {
        // tensing(1, tensing(2, tensing(3, 4)))
        // pops 4, leaving tensing(1, tensing(2, 3))
        let mut t = tensing(leaf(1), tensing(leaf(2), tensing(leaf(3), leaf(4))));
        assert_eq!(t.pop_last(), Some(4));
        assert_eq!(collect_leaves(&t), vec![1, 2, 3]);
    }

    #[test]
    fn pop_last_does_not_touch_left_subtree() {
        // tensing(tensing(1, 2), tensing(3, 4))
        // rightmost is 4; left subtree stays intact
        let mut t = tensing(tensing(leaf(1), leaf(2)), tensing(leaf(3), leaf(4)));
        assert_eq!(t.pop_last(), Some(4));
        assert_eq!(collect_leaves(&t), vec![1, 2, 3]);
    }

    #[test]
    fn pop_last_multiple_consecutive() {
        // tensing(1, tensing(2, tensing(3, 4)))
        // pop → 4, pop → 3, pop → 2, pop → None (single leaf left)
        let mut t = tensing(leaf(1), tensing(leaf(2), tensing(leaf(3), leaf(4))));
        assert_eq!(t.pop_last(), Some(4));
        assert_eq!(t.pop_last(), Some(3));
        assert_eq!(t.pop_last(), Some(2));
        // tree is now just leaf(1)
        assert_eq!(collect_leaves(&t), vec![1]);
        assert_eq!(t.pop_last(), None);
    }

    #[test]
    fn pop_last_mixed_tensing_relaxing() {
        // tensing(relaxing(1, 2), relaxing(3, 4))
        let mut t = tensing(relaxing(leaf(1), leaf(2)), relaxing(leaf(3), leaf(4)));
        assert_eq!(t.pop_last(), Some(4));
        assert_eq!(collect_leaves(&t), vec![1, 2, 3]);
    }

    #[test]
    fn pop_first_on_leaf_returns_none() {
        let mut t = leaf(42);
        assert_eq!(t.pop_first(), None);
        assert_eq!(collect_leaves(&t), vec![42]);
    }

    #[test]
    fn pop_first_two_node_tree() {
        // tensing(1, 2) → pops 1, tree becomes leaf(2)
        let mut t = tensing(leaf(1), leaf(2));
        assert_eq!(t.pop_first(), Some(1));
        assert_eq!(collect_leaves(&t), vec![2]);
    }

    #[test]
    fn pop_first_two_node_relaxing() {
        let mut t = relaxing(leaf(10), leaf(20));
        assert_eq!(t.pop_first(), Some(10));
        assert_eq!(collect_leaves(&t), vec![20]);
    }

    #[test]
    fn pop_first_deep_left_chain() {
        // tensing(tensing(tensing(1, 2), 3), 4)
        // pops 1, leaving tensing(tensing(2, 3), 4)
        let mut t = tensing(tensing(tensing(leaf(1), leaf(2)), leaf(3)), leaf(4));
        assert_eq!(t.pop_first(), Some(1));
        assert_eq!(collect_leaves(&t), vec![2, 3, 4]);
    }

    #[test]
    fn pop_first_does_not_touch_right_subtree() {
        // tensing(tensing(1, 2), tensing(3, 4))
        // leftmost is 1; right subtree stays intact
        let mut t = tensing(tensing(leaf(1), leaf(2)), tensing(leaf(3), leaf(4)));
        assert_eq!(t.pop_first(), Some(1));
        assert_eq!(collect_leaves(&t), vec![2, 3, 4]);
    }

    #[test]
    fn pop_first_multiple_consecutive() {
        // tensing(tensing(tensing(1, 2), 3), 4)
        // pop → 1, pop → 2, pop → 3, tree is leaf(4), pop → None
        let mut t = tensing(tensing(tensing(leaf(1), leaf(2)), leaf(3)), leaf(4));
        assert_eq!(t.pop_first(), Some(1));
        assert_eq!(t.pop_first(), Some(2));
        assert_eq!(t.pop_first(), Some(3));
        assert_eq!(collect_leaves(&t), vec![4]);
        assert_eq!(t.pop_first(), None);
    }

    #[test]
    fn pop_first_mixed_tensing_relaxing() {
        let mut t = tensing(relaxing(leaf(1), leaf(2)), relaxing(leaf(3), leaf(4)));
        assert_eq!(t.pop_first(), Some(1));
        assert_eq!(collect_leaves(&t), vec![2, 3, 4]);
    }
}
