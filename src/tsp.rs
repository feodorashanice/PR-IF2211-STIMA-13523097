use std::usize;

fn tsp(graph: &Vec<Vec<i32>>, n: usize) -> (i32, Vec<usize>) {
    // Membuat tabel DP: dp[mask][curr] menyimpan biaya minimum dan jalur untuk subset kota (mask) yang berakhir di curr
    let mut dp = vec![vec![(i32::MAX, vec![]); n]; 1 << n];
    
    dp[1][0] = (0, vec![0]);
    // Iterasi melalui semua kemungkinan subset kota
    for mask in 1..(1 << n) {
        for curr in 0..n {
            // Skip jika kota curr tidak ada dalam mask
            if (mask & (1 << curr)) == 0 {
                continue;
            }
            
            // Coba setiap kota sebelumnya sebagai langkah sebelumnya
            for prev in 0..n {
                if prev == curr || (mask & (1 << prev)) == 0 {
                    continue;
                }
                
                // Subset tanpa kota saat ini
                let prev_mask = mask ^ (1 << curr);
                
                // Jika ada jalur valid dari prev_mask ke prev
                if dp[prev_mask][prev].0 != i32::MAX && graph[prev][curr] != i32::MAX {
                    let new_cost = dp[prev_mask][prev].0 + graph[prev][curr];
                    if new_cost < dp[mask][curr].0 {
                        let mut new_path = dp[prev_mask][prev].1.clone();
                        new_path.push(curr);
                        dp[mask][curr] = (new_cost, new_path);
                    }
                }
            }
        }
    }

    // Mencari biaya minimum untuk kembali ke kota awal (0)
    let mut min_cost = i32::MAX;
    let mut final_path = vec![];
    let final_mask = (1 << n) - 1;
    
    for last in 1..n {
        if dp[final_mask][last].0 != i32::MAX && graph[last][0] != i32::MAX {
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
    // Contoh graf: matriks ketetanggaan dengan bobot
    let graph = vec![ // Test case sementara
        vec![0, 10, 15, 20],
        vec![10, 0, 35, 25],
        vec![15, 35, 0, 30],
        vec![20, 25, 30, 0],
    ];
    
    let n = graph.len();
    let (cost, path) = tsp(&graph, n);
    
    if cost == i32::MAX {
        println!("Tidak ada solusi yang mungkin");
    } else {
        println!("Biaya minimum: {}", cost);
        println!("Jalur: {:?}", path);
    }
}