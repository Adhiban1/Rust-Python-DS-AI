# Gradient Descent

**rand_array function:**

```rust
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
```

$$Input, X = 
\begin{bmatrix}
x_{11} \\
x_{21} \\
\vdots \\
x_{j1}
\end{bmatrix}
$$

```rust
let x = rand_array();
```

**rand_weight function:**

```rust
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
```

$$Weight, W = 
\begin{bmatrix}
w_{11} & w_{12} & \dots & w_{1j} \\
w_{21} & w_{22} & \dots & w_{2j} \\
\vdots & \vdots & \ddots & \vdots \\
w_{i1} & w_{i2} & \dots & w_{ij}
\end{bmatrix}
$$

```rust
let mut w = rand_weight();
```

$$Output, Y = 
\begin{bmatrix}
y_{11} \\
y_{21} \\
\vdots \\
y_{i1}
\end{bmatrix}
$$

```rust
let ytrue = rand_array();
```

$$Y predict, \hat{Y} = 
\begin{bmatrix}
\hat{y}_{11} \\
\hat{y}_{21} \\
\vdots \\
\hat{y}_{i1}
\end{bmatrix}
$$

```rust
let mut y;
//...
for _ in 0..n {
    //...
    y = dot(&x, &w);
    //...
}
//...
```

$$\hat{Y} = 
W.X = 
\begin{bmatrix}
w_{11} & w_{12} & \dots & w_{1j} \\
w_{21} & w_{22} & \dots & w_{2j} \\
\vdots & \vdots & \ddots & \vdots \\
w_{i1} & w_{i2} & \dots & w_{ij}
\end{bmatrix} \cdot
\begin{bmatrix}
x_{11} \\
x_{21} \\
\vdots \\
x_{j1}
\end{bmatrix}
$$

**Summation Equation:**

$$
\hat{y}_{i1} = \sum_{n=1}^{j} w_{in}.x_{n1}
$$

```rust
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
```

**Loss:**

$$
loss = \frac{1}{N} \sum (Y-\hat{Y})^2
$$

$$
\Rightarrow loss = \frac{1}{N} \sum (Y-W.X)^2
$$

$$
\Rightarrow loss = \frac{1}{N} \sum_{m=1}^{i} \left( y_{m1} - \sum_{n=1}^{j} w_{mn}.x_{n1} \right)^2
$$

```rust
fn loss(ytrue: &[[f64; 1]; 3], y: &[[f64; 1]; 3]) -> f64 {
    let mut loss = 0.0;
    for i in 0..3 {
        loss += (ytrue[i][0] - y[i][0]).powi(2);
    }
    loss / 3.0
}
```

**Grad:**
$$
\frac{d(loss)}{d(w_{ij})} = \frac{d}{d(w_{ij})} \left[ \frac{1}{N} \sum_{m=1}^{i} \left( y_{m1} - \sum_{n=1}^{j} w_{mn}.x_{n1} \right)^2 \right]
$$

$$
= \frac{1}{N} \sum_{m=1}^{i} \left[  \frac{d}{d(w_{ij})} \left( y_{m1} - \sum_{n=1}^{j} w_{mn}.x_{n1} \right)^2 \right]
$$

$$
= \frac{1}{N} \sum_{m=1}^{i} \left[ 2 \left( y_{m1} - \sum_{n=1}^{j} w_{mn}.x_{n1} \right) . \left( 0 - \sum_{n=1}^{j} \frac{d}{d(w_{ij})} (w_{mn}.x_{n1}) \right) \right]
$$

$$
= \frac{1}{N} \sum_{m=1}^{i} \left[ 2 \left( y_{m1} - \sum_{n=1}^{j} w_{mn}.x_{n1} \right) . \left( - x_{j1} \right) \right]
$$

$$
\frac{d(loss)}{d(w_{ij})}
= - \frac{2x_{j1}}{N} \sum_{m=1}^{i} \left( y_{m1} - \sum_{n=1}^{j} w_{mn}.x_{n1} \right)
$$

```rust
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
```