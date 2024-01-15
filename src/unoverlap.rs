/// build a new vec of dots with no dot less than
/// `margin` from another one.
/// When this can't be done, return `None`.
/// The `dots` vec is assumed
/// - to contain 2 or more elements
/// - to be strictly increasing
/// Extremities (first and last point) are guaranteed to be
/// returned unchanged.
pub fn unoverlap(mut dots: Vec<i64>, margin: i64) -> Option<Vec<i64>> {
    let l = dots.len();
    let w = dots[l - 1] - dots[0];
    assert!(w > 0);
    if margin * (l - 1) as i64 > w {
        return None;
    }
    #[derive(Debug)]
    struct Subset {
        first_idx: usize,
        len: usize, // number of dots
        width: i64,
    }
    #[allow(unused_variables)]
    #[allow(clippy::collapsible_else_if)]
    for i in 0..2 * l {
        // 2l is probably overkill but I can't prove it
        // candidate subsets are subsets with more than margin before
        // and after and with more dots than allowed.
        // The best one is the one with smallest width/dots_counts ratio.
        let mut best_subset: Option<Subset> = None;
        let mut cur: Option<usize> = None; // index of the subset start
        for idx in 0..l {
            if let Some(mut first_idx) = cur {
                if idx == l - 1 || dots[idx] + margin < dots[idx + 1] {
                    // we build the subset, starting by looking back
                    // if we must take back some points
                    while first_idx > 0 {
                        if dots[first_idx] - dots[first_idx - 1] > margin {
                            break;
                        }
                        first_idx -= 1;
                    }
                    let width = dots[idx] - dots[first_idx];
                    let subset = Subset {
                        first_idx,
                        len: idx - first_idx + 1,
                        width,
                    };
                    // we close this subset and compare it wit the previous one
                    if let Some(best) = &best_subset {
                        if (best.width * subset.len as i64) < (subset.width * best.len as i64) {
                            best_subset = Some(subset);
                        }
                    } else {
                        best_subset = Some(subset);
                    }
                    cur = None;
                }
                // the dot is implictely added to the current subset
            } else {
                if idx < l - 1 && dots[idx] + margin > dots[idx + 1] {
                    // we start a new subset
                    cur = Some(idx);
                }
            }
        }
        let Subset {
            first_idx,
            width,
            len,
        } = match best_subset {
            Some(s) => s,
            None => {
                // we should have finished
                break;
            }
        };
        // by construction, the subset contains 2 points or more
        // and is more dense than allowed.
        let last_idx = first_idx + len - 1;
        //println!(
        //    " subset before enlargment: {}",
        //    dots[first_idx..last_idx+1].iter()
        //        .map(i64::to_string)
        //        .collect::<Vec<String>>()
        //        .join(",")
        //);
        // The optimal new position of the subset is symetric around
        // the mean of its dots but we must account for the available
        // space
        let mean = dots[first_idx..last_idx + 1]
            .iter()
            .map(|&d| d as f64)
            .sum::<f64>()
            / (len as f64);
        let optimal_width = (len - 1) as i64 * margin;
        let optimal_start = (mean - optimal_width as f64 / 2.0).round() as i64;
        let optimal_range = (optimal_start, optimal_start + optimal_width);
        let left_limit = if first_idx > 0 {
            dots[first_idx - 1] + margin
        } else {
            dots[0]
        };
        let right_limit = if first_idx + len < l {
            dots[first_idx + len] - margin
        } else {
            dots[l - 1]
        };
        let (o_left, o_right) = optimal_range;
        let (new_start, new_end) = if left_limit <= o_left && right_limit >= o_right {
            // we can enlarge the subset as optimal
            optimal_range
        } else if left_limit > o_left {
            // we're limited at left
            (left_limit, (left_limit + optimal_width).min(right_limit))
        } else {
            // we're right limited
            ((right_limit - optimal_width).max(left_limit), right_limit)
        };
        // now we move the dots to be equidistant in (new_start, new_end)
        let w = (new_end - new_start) as f64 / (len - 1) as f64;
        for i in 0..len {
            dots[first_idx + i] = (new_start as f64 + w * i as f64).round() as i64;
        }
        //println!(
        //    " subset after enlargment: {}",
        //    dots[first_idx..last_idx+1].iter()
        //        .map(i64::to_string)
        //        .collect::<Vec<String>>()
        //        .join(",")
        //);
        //println!(
        //    " all dots after enlargment: {}",
        //    dots.iter()
        //        .map(i64::to_string)
        //        .collect::<Vec<String>>()
        //        .join(",")
        //);
    }
    Some(dots)
}

#[cfg(test)]
mod unoverlap_tests {
    use super::*;

    #[test]
    fn test_unoverlap_1() {
        assert_eq!(
            unoverlap(vec![0, 49, 51, 100], 10),
            Some(vec![0, 45, 55, 100]),
        );
    }
    #[test]
    fn test_unoverlap_2() {
        assert_eq!(
            unoverlap(vec![0, 51, 52, 53, 100], 10),
            Some(vec![0, 42, 52, 62, 100]),
        );
    }
    #[test]
    fn test_unoverlap_3() {
        assert_eq!(unoverlap(vec![0, 51, 52, 53, 100], 60), None,);
    }
    #[test]
    fn test_unoverlap_4() {
        assert_eq!(
            unoverlap(vec![0, 1, 2, 20, 51, 52, 53, 100], 10),
            Some(vec![0, 10, 20, 30, 42, 52, 62, 100]),
        );
    }
    #[test]
    fn test_unoverlap_5() {
        assert_eq!(
            unoverlap(vec![0, 1, 2, 20, 26, 28, 51, 52, 53, 100], 10),
            Some(vec![0, 10, 20, 30, 40, 50, 60, 70, 80, 100]),
        );
    }
    #[test]
    fn test_unoverlap_6() {
        assert_eq!(
            unoverlap(vec![0, 1, 2, 20, 26, 28, 51, 52, 53, 99, 100], 10),
            Some(vec![0, 10, 20, 30, 40, 50, 60, 70, 80, 90, 100]),
        );
    }
    #[test]
    fn test_unoverlap_7() {
        assert_eq!(
            unoverlap(vec![0, 16, 17, 20, 86, 87, 99, 100], 10),
            Some(vec![0, 10, 20, 30, 70, 80, 90, 100]),
        );
    }
}
