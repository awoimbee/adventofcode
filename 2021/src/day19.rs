use std::cmp::Ordering;
use std::collections::HashMap;

use nalgebra::base::{Matrix3, Vector3};

use nom::{
    bytes::complete::tag,
    character::complete,
    error::Error,
    sequence::{preceded, terminated},
};

use itertools::Itertools;

const INPUT: &str = include_str!("../input/day19.txt");

// The threshold for number of overlapping probes is 12, this translates to n*(n-1)/2 egdes.
const ALIGNMENT_THRESHOLD: u32 = 12;
const EDGE_THRESHOLD: u32 = ALIGNMENT_THRESHOLD * (ALIGNMENT_THRESHOLD - 1) / 2;

#[derive(Debug, Clone)]
struct Scanner {
    id: u32,
    beacons: Vec<Vector3<i32>>,
}

#[derive(Debug, Clone)]
/// Wrapper around [`Scanner`] with additional info about its state
struct ScannerInfo {
    scanner: Scanner,
    inner_distances: Vec<u32>, // needs to be sorted by dist for optimal performance
    position: Option<Vector3<i32>>,
    orientation: Option<Matrix3<i32>>,
}

fn part1(scanners: Vec<ScannerInfo>) -> String {
    let aligned = align_scanners(scanners);
    // Check Number of Beacons
    let mut beacons = Vec::new();
    for scanner_info in aligned.values() {
        let bs = scanner_info
            .scanner
            .beacons
            .iter()
            .map(|v| scanner_info.orientation.unwrap() * v + scanner_info.position.unwrap());
        beacons.extend(bs);
    }
    beacons.sort_by(compare_vector);
    beacons.dedup_by(|x, y| compare_vector(x, y) == Ordering::Equal);
    beacons.len().to_string()
}

fn part2(scanners: Vec<ScannerInfo>) -> String {
    let aligned = align_scanners(scanners);

    let mut scanner_positions = Vec::new();
    for scanner_info in aligned.values() {
        scanner_positions.push(scanner_info.position.unwrap());
    }

    let mut max_dist = 0;
    while let Some(pos) = scanner_positions.pop() {
        for p in scanner_positions.iter() {
            max_dist = i32::max(max_dist, manhattan_distance(&pos, p));
        }
    }
    max_dist.to_string()
}

/// Aligns `s1` with neighbour `s2`, requires that `s2` has a known position and orientation.
fn align_scanner(s1: &mut ScannerInfo, s2: &ScannerInfo) -> bool {
    assert!(
        s2.position.is_some() && s2.orientation.is_some(),
        "'s2' must have a known orientation and position"
    );

    let s2_diffs: Vec<Vector3<i32>> = position_differences(&s2.scanner.beacons);

    // Step 1. Find correct orientation
    let mut orientation = None;

    for rotation in possible_orientations() {
        // Apply rotation to beacon locations
        let mut beacons: Vec<Vector3<i32>> = s1
            .scanner
            .beacons
            .clone()
            .into_iter()
            .map(|v| rotation * v)
            .collect();
        beacons.sort_by(compare_vector);
        // Compute new differences between beacons
        let diffs = position_differences(&beacons);

        let eq_diffs = equal_vector_count(&diffs, &s2_diffs);

        if eq_diffs >= EDGE_THRESHOLD {
            orientation = Some(s2.orientation.unwrap() * rotation);
            break;
        }
    }
    if orientation.is_none() {
        return false;
    }

    // Step 2. Find offset which causes probes to overlap
    let mut position: Option<Vector3<i32>> = None;

    let mut s2_beacons: Vec<Vector3<i32>> = s2
        .scanner
        .beacons
        .clone()
        .into_iter()
        .map(|v| s2.orientation.unwrap() * v)
        .collect();
    s2_beacons.sort_by(|v1, v2| compare_vector(v1, v2));

    let mut s1_beacons: Vec<Vector3<i32>> = s1
        .scanner
        .beacons
        .clone()
        .into_iter()
        .map(|v| orientation.unwrap() * v)
        .collect();
    s1_beacons.sort_by(|v1, v2| compare_vector(v1, v2));

    let mut stack = s2_beacons.clone();
    'outer: while let Some(s2_beacon) = stack.pop() {
        // Use s1_beacon as reference point
        for s1_beacon in s1_beacons.iter() {
            // align s2_beacon with s1_beacon and check whether alignment is correct
            let offset = s2_beacon - s1_beacon; // offset + x1 = x2 (if correct)
            let mut aligned_beacons: Vec<Vector3<i32>> =
                s1_beacons.clone().into_iter().map(|v| offset + v).collect();
            aligned_beacons.sort_by(|x, y| compare_vector(x, y));

            let eq = equal_vector_count(&aligned_beacons, &s2_beacons);
            if eq >= ALIGNMENT_THRESHOLD {
                position = Some(s2.position.unwrap() + offset);
                break 'outer;
            }
        }
    }
    if position.is_none() {
        return false;
    }
    s1.position = position;
    s1.orientation = orientation;
    true
}

/// Aligns all scanners
fn align_scanners(scanners: Vec<ScannerInfo>) -> HashMap<u32, ScannerInfo> {
    let mut unaligned = HashMap::new();
    let mut visited = HashMap::new();

    for s in scanners.into_iter() {
        unaligned.insert(s.scanner.id, s);
    }

    // Remove first scanner and make it the base reference frame
    let mut s0 = unaligned.remove(&0).unwrap();
    s0.position = Some(Vector3::from_element(0));
    s0.orientation = Some(Matrix3::identity());

    let mut queue = vec![s0];

    // Align scanners through graph traversal
    while let Some(scanner_info) = queue.pop() {
        // Visit node and try to align neighbouring nodes
        let ids = potential_neighbouring_scanners(&scanner_info, unaligned.values());
        // Remove potential candidates
        for id in ids {
            let mut s = unaligned.remove(&id).unwrap(); // temporariliy take ownership of scanner
            let success = align_scanner(&mut s, &scanner_info);

            // Check whether scanner was aligned successfully
            if success {
                queue.push(s); // scanner can be visited next
            } else {
                unaligned.insert(id, s);
            }
        }
        visited.insert(scanner_info.scanner.id, scanner_info);
    }
    assert_eq!(
        unaligned.len(),
        0,
        "There are still unaligned scanners left over"
    );
    visited
}

/// Uses the distances between beacons to find potential other scanners, which overlap their regions with
/// the scanner
///
/// Note: This function does not guarantee that two scanners overlap, but there is strong evidence
/// that two scanners might overlap. If a scanner is not in the result then it does definitely not
/// overlap with `s1`.
fn potential_neighbouring_scanners<'a>(
    s1: &ScannerInfo,
    scanners: impl Iterator<Item = &'a ScannerInfo>,
) -> Vec<u32> {
    let mut potential = Vec::new();
    for scanner_info in scanners {
        let eq_dist = equal_distance_count(s1, scanner_info);
        if eq_dist > EDGE_THRESHOLD {
            potential.push(scanner_info.scanner.id);
        }
    }
    potential
}

/// First analysis of a scanner and its probes
fn analyze_scanner(scanner: Scanner) -> ScannerInfo {
    let inner_distances = manhattan_distances(&scanner);
    ScannerInfo {
        scanner,
        inner_distances,
        position: None,
        orientation: None,
    }
}

/// Manhattan distance between two vectors
fn manhattan_distance(v1: &Vector3<i32>, v2: &Vector3<i32>) -> i32 {
    v1.iter()
        .zip(v2.iter())
        .map(|(x1, x2)| (x1 - x2).abs())
        .sum()
}

/// Computes a list of all distances between the beacons
fn manhattan_distances(scanner: &Scanner) -> Vec<u32> {
    let mut distances =
        Vec::with_capacity(((scanner.beacons.len() - 1) * scanner.beacons.len()) / 2);
    let mut it = scanner.beacons.iter();
    while let Some(beacon1) = it.next() {
        for beacon2 in it.clone() {
            let dist = manhattan_distance(beacon1, beacon2) as u32;
            distances.push(dist);
        }
    }
    distances.sort_unstable();
    distances
}

/// Calculates the (sorted) differences between the beacon locations
fn position_differences(beacons: &[Vector3<i32>]) -> Vec<Vector3<i32>> {
    let mut differences = Vec::with_capacity(((beacons.len() - 1) * beacons.len()) / 2);
    let mut stack: Vec<&Vector3<i32>> = beacons.iter().collect();
    while stack.len() > 1 {
        let beacon1 = stack.pop().unwrap();
        // compare element with elements left in stack
        for beacon2 in stack.iter() {
            differences.push(beacon1 - *beacon2);
        }
    }
    differences.sort_by(compare_vector);
    differences
}

/// Computes the number of equal distances between beacons.
/// Useful heuristic for checking whether two scanners might have overlapping regions
fn equal_distance_count(s1: &ScannerInfo, s2: &ScannerInfo) -> u32 {
    let d1 = &s1.inner_distances; // inner_distances are sorted
    let d2 = &s2.inner_distances;
    let mut count = 0;
    let (mut i1, mut i2) = (0, 0);
    while i1 < d1.len() && i2 < d2.len() {
        match d1[i1].cmp(&d2[i2]) {
            Ordering::Less => i1 += 1,
            Ordering::Greater => i2 += 1,
            Ordering::Equal => {
                count += 1;
                i1 += 1;
                i2 += 1;
            }
        }
    }
    count
}

/// Requires `s1` and `s2` are sorted
fn equal_vector_count(s1: &[Vector3<i32>], s2: &[Vector3<i32>]) -> u32 {
    let d1 = s1; // inner_distances are sorted
    let d2 = s2;
    let mut count = 0;
    let (mut i1, mut i2) = (0, 0);
    while i1 < d1.len() && i2 < d2.len() {
        match compare_vector(&d1[i1], &d2[i2]) {
            Ordering::Less => i1 += 1,
            Ordering::Greater => i2 += 1,
            Ordering::Equal => {
                count += 1;
                i1 += 1;
                i2 += 1;
            }
        }
    }
    count
}

/// Returns all possible orientations that the scanner could be in the form of rotation matrices
fn possible_orientations() -> Vec<Matrix3<i32>> {
    let matrices: Vec<Matrix3<i32>> = vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]]
        .into_iter()
        .permutations(3)
        .map(|e| Matrix3::from_iterator(e.concat().into_iter()))
        .map(|m| {
            let mut m2 = m;
            multiply_row(&mut m2, 0, -1);
            vec![m, m2]
        })
        .flatten()
        .map(|m| {
            let mut m2 = m;
            multiply_row(&mut m2, 1, -1);
            vec![m, m2]
        })
        .flatten()
        .map(|m| {
            let mut m2 = m;
            multiply_row(&mut m2, 2, -1);
            vec![m, m2]
        })
        .flatten()
        .filter(|m| det(m) == 1)
        .collect();
    matrices
}

fn multiply_row(matrix: &mut Matrix3<i32>, index: usize, scalar: i32) {
    for i in 0..3 {
        matrix[i * 3 + index] *= scalar;
    } // column major matrix
}

/// Determinant of 3x3 Matrix
fn det(m: &Matrix3<i32>) -> i32 {
    let mut d = m[0] * (m[3 * 1 + 1] * m[3 * 2 + 2] - m[3 * 2 + 1] * m[3 * 1 + 2]);
    d -= m[3] * (m[1] * m[8] - m[7] * m[2]);
    d += m[6] * (m[1] * m[5] - m[4] * m[2]);
    d
}

/// Utility function for comparing vectors (useful for sorting)
fn compare_vector(v1: &Vector3<i32>, v2: &Vector3<i32>) -> Ordering {
    match v1[0].cmp(&v2[0]) {
        Ordering::Equal => match v1[1].cmp(&v2[1]) {
            Ordering::Equal => v1[2].cmp(&v2[2]),
            ord => ord,
        },
        o => o,
    }
}

// --- PARSING ---
fn parse_scanner_id(input: &str) -> u32 {
    let mut parser = preceded::<&str, _, _, Error<&str>, _, _>(
        tag("--- scanner "),
        terminated(complete::u32, tag(" ---")),
    );
    parser(input).unwrap().1
}

fn parse_beacon(input: &str) -> Vector3<i32> {
    let mut coords = input.trim().split(',');
    let x = coords.next().unwrap().parse().unwrap();
    let y = coords.next().unwrap().parse().unwrap();
    let z = coords.next().unwrap().parse().unwrap();
    Vector3::from_iterator(vec![x, y, z].into_iter())
}

fn parse(input: &str) -> Vec<Scanner> {
    let scans = input.split("\n\n");
    let mut scanners = Vec::new();

    for scan in scans {
        let mut lines = scan.lines();
        let heading = lines.next().unwrap();
        let id = parse_scanner_id(heading);
        let mut beacons = Vec::new();
        for beacon in lines {
            beacons.push(parse_beacon(beacon));
        }
        beacons.sort_by(compare_vector);
        scanners.push(Scanner { id, beacons });
    }
    scanners
}

pub fn day19() -> (String, String) {
    let parsed: Vec<_> = parse(INPUT).into_iter().map(analyze_scanner).collect();
    let p1 = part1(parsed.clone());
    let p2 = part2(parsed);
    (p1, p2)
}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_INPUT: &str = "--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390

--- scanner 2 ---
649,640,665
682,-795,504
-784,533,-524
-644,584,-595
-588,-843,648
-30,6,44
-674,560,763
500,723,-460
609,671,-379
-555,-800,653
-675,-892,-343
697,-426,-610
578,704,681
493,664,-388
-671,-858,530
-667,343,800
571,-461,-707
-138,-166,112
-889,563,-600
646,-828,498
640,759,510
-630,509,768
-681,-892,-333
673,-379,-804
-742,-814,-386
577,-820,562

--- scanner 3 ---
-589,542,597
605,-692,669
-500,565,-823
-660,373,557
-458,-679,-417
-488,449,543
-626,468,-788
338,-750,-386
528,-832,-391
562,-778,733
-938,-730,414
543,643,-506
-524,371,-870
407,773,750
-104,29,83
378,-903,-323
-778,-728,485
426,699,580
-438,-605,-362
-469,-447,-387
509,732,623
647,635,-688
-868,-804,481
614,-800,639
595,780,-596

--- scanner 4 ---
727,592,562
-293,-554,779
441,611,-461
-714,465,-776
-743,427,-804
-660,-479,-426
832,-632,460
927,-485,-438
408,393,-506
466,436,-512
110,16,151
-258,-428,682
-393,719,612
-211,-452,876
808,-476,-593
-575,615,604
-485,667,467
-680,325,-822
-627,-443,-432
872,-547,-609
833,512,582
807,604,487
839,-516,451
891,-625,532
-652,-548,-490
30,-46,-14";

    #[test]
    fn test_manhattan_dist() {
        let v1 = Vector3::from_iterator(vec![0, 2, -2].into_iter());
        let v2 = Vector3::from_iterator(vec![0, -2, 1].into_iter());
        assert_eq!(manhattan_distance(&v1, &v2), 7);
    }

    #[test]
    fn test_possible_orientations() {
        assert_eq!(
            possible_orientations().len(),
            24,
            "Invalid number of orientations"
        );
    }

    #[test]
    fn test_utilities() {
        let v1 = Vector3::from_iterator(vec![0, 2, -2].into_iter());
        let v2 = Vector3::from_iterator(vec![0, 2, -2].into_iter());
        let v3 = Vector3::from_iterator(vec![-1, 2, -2].into_iter());
        let vec1 = vec![v3, v2];
        let vec2 = vec![v1];
        assert_eq!(equal_vector_count(&vec1, &vec2), 1);
    }

    #[test]
    fn test_parsing() {
        let parsed = parse(TEST_INPUT);
        assert_eq!(parsed.len(), 5);
        assert_eq!(parsed[0].id, 0);
        assert_eq!(parsed[0].beacons.len(), 25);
        assert_eq!(
            parsed[1].beacons[0],
            Vector3::from_iterator(vec![-500, -761, 534].into_iter())
        );
    }
}
