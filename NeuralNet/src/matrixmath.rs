use super::layer;

pub fn matrix_mul(a: &Vec<Vec<f32>>, b: &Vec<Vec<f32>>) -> Vec<Vec<f32>>  {
    //This 1.Transposes 2. Multiplies
    let mut new_matrix: Vec<Vec<f32>> = vec![];
    if a[0].len() == b[0].len()    {
        for i in 0..a.len() {
            let mut new_layer: Vec<f32> = vec![];
            for j in 0..b.len()    {
                let mut sum:f32 = 0.0;
                for k in 0..a[0].len() {
                    sum += a[i][k] * b[k][j];
                }
                new_layer.push(sum)
            }
            new_matrix.push(new_layer);
        }
    }
    new_matrix
}

pub fn matrix_add(a: &Vec<Vec<f32>>, b: &Vec<Vec<f32>>) -> Vec<Vec<f32>>   {
    let mut new_matrix: Vec<Vec<f32>> = vec![];
    if a.len() == b[0].len()    {
        for i in 0..a.len() {
            let mut layer: Vec<f32> = vec![];
            for j in 0..a[0].len()  {
                layer.push(a[i][j] + b[i][j]);
            }
            new_matrix.push(layer);
        }
    }
    else    {
        println!("DNR");
    }
    new_matrix
}
