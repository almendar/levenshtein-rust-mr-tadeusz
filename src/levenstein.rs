use std::fmt::Display;

#[allow(dead_code)]
fn print_array<A: Display>(array: &Vec<Vec<A>>) {
    for i in array {
        for j in i {
            print!("{} ", j);
        }
        println!("\n")
    }
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
    let mut array = vec![
    vec![0; n+1]; m+1
  ];

    for i in 1..m+1 {
        array[i][0] = i;
    }
    for j in 1..n+1 {
        array[0][j] = j;
    }

    for (i, c1) in &mut s1.chars().enumerate() {
        for (j, c2) in &mut s2.chars().enumerate() {
            let mut subs_cost = 0;
            if c1 != c2 {
                subs_cost = 1;
            }
            let three_min = vec![
                  array[i][j+1] + 1,
                  array[i+1][j] + 1,
                  array[i][j] + subs_cost
                ];
                array[i + 1][j + 1] = *three_min.iter().min().unwrap();

        }
    }



    // print_array(&array);
    return array[m][n];
  }
