use std::fs;
use std::process::Command;
use rand::prelude::*;

fn main() {
    let n = 100;
    let x = rand_array();
    let mut w = rand_weight();
    let ytrue = rand_array();
    let mut y;
    let mut losses = Vec::new();
    for _ in 0..n {
        w = grad(&ytrue, &w, &x, 0.01);
        y = dot(&x, &w);
        losses.push(loss(&ytrue, &y));
    }
    save_csv(losses);
    save_weight(&w);
    run_python();
}

fn run_python() {
    let output = Command::new("/home/adhiban/anaconda3/bin/python")
        .arg("graph.py")
        .output()
        .expect("Failed to execute command");
    if output.status.success() {
        // Print the output if the command ran successfully
        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("Output: {}", stdout);
    } else {
        // Print the error message if the command failed
        let stderr = String::from_utf8_lossy(&output.stderr);
        println!("Error: {}", stderr);
    }
}

fn dot(x: &[[f64; 1]; 3], w: &[[f64; 3]; 3]) -> [[f64; 1]; 3] {
    let mut y = [[0.0; 1]; 3];
    for i in 0..3 {
        for k in 0..1 {
            for j in 0..3 {
                y[i][k] += w[i][j] * x[j][k];
            }
        }
    }
    y
}

fn rand_array() -> [[f64; 1]; 3] {
    let mut rng = StdRng::seed_from_u64(0);
    let mut a = [[0.0; 1]; 3];
    for i in 0..3 {
        for j in 0..1 {
            a[i][j] = rng.gen();
        }
    }
    a
}

fn rand_weight() -> [[f64; 3]; 3] {
    let mut rng = StdRng::seed_from_u64(0);
    let mut a = [[0.0; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            a[i][j] = rng.gen();
        }
    }
    a
}

fn loss(ytrue: &[[f64; 1]; 3], y: &[[f64; 1]; 3]) -> f64 {
    let mut loss = 0.0;
    for i in 0..3 {
        loss += (ytrue[i][0] - y[i][0]).powi(2);
    }
    loss / 3.0
}

fn grad(ytrue: &[[f64; 1]; 3], w: &[[f64; 3]; 3], x: &[[f64; 1]; 3], lr: f64) -> [[f64; 3]; 3] {
    let mut dw = [[0.0; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            let mut temp = 0.0;
            for k in 0..3 {
                temp += w[i][k] * x[k][0];
            }
            dw[i][j] = w[i][j] + 2.0 * lr * x[j][0] * (ytrue[i][0] - temp);
        }
    }
    dw
}

fn save_csv(a: Vec<f64>) {
    let mut s = String::new();
    for i in a {
        s.push_str(&i.to_string());
        s.push_str("\n");
    }
    fs::write("data.csv", s).unwrap();
}

fn save_weight(w:&[[f64; 3]; 3]) {
    let mut s = String::new();
    for i in 0..3 {
        for j in 0..3 {
            s.push_str(&w[i][j].to_string());
            if j < 2 {
                s.push(',');
            }
        }
        if i < 2 {
            s.push('\n');
        }
    }
    fs::write("weight.csv", s).unwrap();
}