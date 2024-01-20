# CNN from scratch in Rust

Matrix Multiplication:

$$c_{ik} = \sum_{j = 1}^{n} (a_{ij} \cdot b_{jk})$$

Conv output matrix shape:

$$\text{output\_shape} = (\text{input\_rows}-\text{kernel\_rows}+1, \text{input\_columns}-\text{kernel\_columns}+1)$$