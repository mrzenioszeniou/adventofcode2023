use std::collections::BTreeSet;
use std::collections::HashMap;
use std::collections::HashSet;
use std::convert::TryFrom;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl TryFrom<char> for Direction {
    type Error = String;

    fn try_from(from: char) -> Result<Self, Self::Error> {
        match from {
            'N' | '^' => Ok(Self::North),
            'S' | 'v' => Ok(Self::South),
            'W' | '<' => Ok(Self::West),
            'E' | '>' => Ok(Self::East),
            _ => Err(format!("`{}` is not a valid as direction", from)),
        }
    }
}

impl From<Direction> for char {
    fn from(from: Direction) -> Self {
        match from {
            Direction::North => '^',
            Direction::South => 'v',
            Direction::East => '>',
            Direction::West => '<',
        }
    }
}

impl Direction {
    pub fn turn_left(&self) -> Self {
        match self {
            Self::North => Self::West,
            Self::East => Self::North,
            Self::South => Self::East,
            Self::West => Self::South,
        }
    }

    pub fn turn_right(&self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }

    pub fn u_turn(&self) -> Self {
        match self {
            Self::North => Self::South,
            Self::East => Self::West,
            Self::South => Self::North,
            Self::West => Self::East,
        }
    }

    pub fn forward_step(&self) -> (isize, isize) {
        match self {
            Self::North => (-1, 0),
            Self::South => (1, 0),
            Self::East => (0, 1),
            Self::West => (0, -1),
        }
    }
}

pub fn neighbours(pos: (isize, isize)) -> Vec<((isize, isize), Direction)> {
    [
        Direction::North,
        Direction::South,
        Direction::East,
        Direction::West,
    ]
    .iter()
    .map(|dir| {
        let step = dir.forward_step();

        ((pos.0 + step.0, pos.1 + step.1), *dir)
    })
    .collect()
}

/// ```
/// o--pos.0-->
/// |
/// pos.1
/// |
/// ⌄
/// ```
pub fn step(pos: (isize, isize), dir: Direction) -> (isize, isize) {
    let step = dir.forward_step();
    (pos.0 + step.0, pos.1 + step.1)
}

/// * `start`: initial state
/// * `finished`: given a state, returns `true` if it satisfies goal
/// * `nexts`: given a state, returns valid next states and the cost of transitions as a `usize`
/// * `heur`: given a state, returns optimistic estimation of remaining cost (estimation <= real)
/// * `debug`: given current state, pending states, and current costs prints a message
pub fn a_star<S, F, N, H, D>(
    start: S,
    finished: F,
    nexts: N,
    heur: H,
    debug: D,
) -> Option<(Vec<S>, usize)>
where
    S: Clone + std::hash::Hash + PartialEq + Eq + PartialOrd + Ord + std::fmt::Debug,
    F: Fn(&S) -> bool,
    N: Fn(&S) -> HashSet<(S, usize)>,
    H: Fn(&S) -> usize,
    D: Fn(&S, &HashMap<S, usize>, &BTreeSet<(usize, S)>),
{
    let mut prevs: HashMap<S, S> = HashMap::new();
    let mut dists: HashMap<S, usize> = HashMap::from([(start.clone(), 0)]);
    let mut to_visit: BTreeSet<(usize, S)> = BTreeSet::from([(0, start.clone())]);

    while let Some((_, mut curr)) = to_visit.pop_first() {
        debug(&curr, &dists, &to_visit);
        if finished(&curr) {
            let cost = *dists.get(&curr).unwrap();
            let mut path = vec![curr.clone()];
            while curr != start {
                curr = prevs.get(&curr).unwrap().clone();
                path.push(curr.clone());
            }
            path.reverse();
            return Some((path, cost));
        }

        for (next, cost) in nexts(&curr) {
            let dist = cost + *dists.get(&curr).unwrap();

            if *dists.get(&next).unwrap_or(&usize::MAX) > dist {
                dists.insert(next.clone(), dist);
                prevs.insert(next.clone(), curr.clone());
                to_visit.insert((dist + heur(&next), next));
            }
        }
    }

    None
}
