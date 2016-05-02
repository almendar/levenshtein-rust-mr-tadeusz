use std::fmt::Display;

type Matrix<A> = Vec<Vec<A>>;

#[allow(dead_code)]
fn print_array<A: Display>(array: &Vec<Vec<A>>) {
    for i in array {
        for j in i {
            print!("{} ", j);
        }
        println!("\n")
    }
}


fn compute_matrix(s1: &str, s2: &str) -> Matrix<usize> {
    let m = s1.chars()
              .count();
    let n = s2.chars()
              .count();

    let mut array: Matrix<usize> = vec![
  vec![0; n+1]; m+1
  ];

    for i in 1..m + 1 {
        array[i][0] = i;
    }
    for j in 1..n + 1 {
        array[0][j] = j;
    }

    for (i, c1) in &mut s1.chars().enumerate() {
        for (j, c2) in &mut s2.chars().enumerate() {
            let mut subs_cost = 0;
            if c1 != c2 {
                subs_cost = 1;
            }
            let three_min = vec![array[i][j + 1] + 1, array[i + 1][j] + 1, array[i][j] + subs_cost];
            array[i + 1][j + 1] = *three_min.iter().min().unwrap();
        }
    }
    return array;
}


pub fn distance(s1: &str, s2: &str) -> usize {
    if s1.is_empty() {
        return s2.len();
    };
    if s2.is_empty() {
        return s1.len();
    };
    let m = s1.chars().count();
    let n = s2.chars().count();
    return compute_matrix(s1, s2)[m][n];
}



#[test]
fn distance_test() {
    assert_eq!(1, distance("tomek", "romek"));
}


#[test]
fn check_matrix() {
    let m_results: Matrix<usize> = compute_matrix("Sunday", "Saturday");
    let m_expected: Matrix<usize> = vec![vec![0, 1, 2, 3, 4, 5, 6, 7, 8],
                                         vec![1, 0, 1, 2, 3, 4, 5, 6, 7],
                                         vec![2, 1, 1, 2, 2, 3, 4, 5, 6],
                                         vec![3, 2, 2, 2, 3, 3, 4, 5, 6],
                                         vec![4, 3, 3, 3, 3, 4, 3, 4, 5],
                                         vec![5, 4, 3, 4, 4, 4, 4, 3, 4],
                                         vec![6, 5, 4, 4, 5, 5, 5, 4, 3]];
    assert_eq!(m_results, m_expected);
}
