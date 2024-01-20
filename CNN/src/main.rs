mod tensor3;
mod tensor2;
use tensor3::{rand_tensor, conv2d, print_tensor, Tensor3};
fn main() {
    let a:Tensor3 = rand_tensor(3, 4, 4);
    let k:Tensor3 = rand_tensor(4, 2, 2);
    print_tensor(&conv2d(&a, &k));
}

