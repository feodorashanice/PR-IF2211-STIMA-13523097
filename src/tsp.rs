use std::usize;
use std::fs;
use std::io::{self, Write};

fn tsp(graph: &Vec<Vec<i32>>, n: usize) -> (i32, Vec<usize>) {
    let mut dp = vec![vec![(i32::MAX, vec![]); n]; 1 << n];
    
    dp[1][0] = (0, vec![0]);
    for mask in 1..(1 << n) {
        for curr in 0..n {
            if (mask & (1 << curr)) == 0 {
                continue;
            }
            for prev in 0..n {
                if prev == curr || (mask & (1 << prev)) == 0 {
                    continue;
                }
                let prev_mask = mask ^ (1 << curr);
                if dp[prev_mask][prev].0 != i32::MAX && graph[prev][curr] != i32::MAX && graph[prev][curr] <= 1000 {
                    let new_cost = dp[prev_mask][prev].0 + graph[prev][curr];
                    if new_cost < dp[mask][curr].0 || (new_cost == dp[mask][curr].0 && prev < *dp[mask][curr].1.last().unwrap_or(&usize::MAX)) {
                        let mut new_path = dp[prev_mask][prev].1.clone();
                        new_path.push(curr);
                        dp[mask][curr] = (new_cost, new_path);
                    }
                }
            }
        }
    }

    let mut min_cost = i32::MAX;
    let mut final_path = vec![];
    let final_mask = (1 << n) - 1;
    
    for last in 1..n {
        if dp[final_mask][last].0 != i32::MAX && graph[last][0] != i32::MAX && graph[last][0] <= 1000 {
            let cost = dp[final_mask][last].0 + graph[last][0];
            if cost < min_cost {
                min_cost = cost;
                final_path = dp[final_mask][last].1.clone();
                final_path.push(0);
            }
        }
    }

    (min_cost, final_path)
}

fn main() {
    print!("Enter test file name: ");
    io::stdout().flush().unwrap();

    let mut file_path = String::new();
    io::stdin()
        .read_line(&mut file_path)
        .expect("Gagal membaca input");

    let file_path = file_path.trim();

    let contents = match fs::read_to_string(file_path) {
        Ok(contents) => contents,
        Err(e) => {
            println!("Gagal membaca file: {}", e);
            return;
        }
    };

    let mut lines = contents.lines();
    
    let n: usize = match lines.next().and_then(|line| line.trim().parse().ok()) {
        Some(num) => num,
        None => {
            println!("Gagal membaca jumlah kota");
            return;
        }
    };

    let mut graph: Vec<Vec<i32>> = Vec::with_capacity(n);
    for (i, line) in lines.enumerate() {
        if i >= n {
            break;
        }
        let row: Vec<i32> = line
            .split_whitespace()
            .filter_map(|num| num.parse().ok())
            .collect();
        if row.len() != n {
            println!("Baris {} tidak memiliki cukup kolom, diharapkan {} kolom", i + 1, n);
            return;
        }
        graph.push(row);
    }

    if graph.len() != n {
        println!("Jumlah baris tidak sesuai, diharapkan {} baris", n);
        return;
    }

    let (cost, path) = tsp(&graph, n);
    
    if cost == i32::MAX {
        println!("Tidak ada solusi yang mungkin!");
    } else {
        println!("Biaya minimum: {}", cost);
        println!("Jalur: {:?}", path);
    }
}