fn main() {
  let matrix_a = [
    [1, 2, 3],
    [4, 5, 6]
  ];

  let matrix_b = [
    [7, 8],
    [9, 10],
    [11, 12]
  ];

  let matrix_result = multiply_matrix(matrix_a, matrix_b);
  println!("{:?}", &matrix_result);
}

fn multiply_matrix(matrix_a: [[i32; 3]; 2], matrix_b: [[i32; 2]; 3]) -> [[i32; 2]; 2] {
  let mut matrix_result: [[i32; 2]; 2] = [
    [0, 0],
    [0, 0]
  ];
  for i in 0..matrix_a.len() {
    for j in 0..matrix_b[0].len() {
      let mut sum = 0;
      for k in 0..matrix_b.len() {
        sum += matrix_a[i][k] * matrix_b[k][j];
      }
      matrix_result[i][j] = sum;
    }
  }

  return matrix_result;
}
