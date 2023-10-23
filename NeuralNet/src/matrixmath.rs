use super::layer;

pub fn matrix_mul(a: &Vec<Vec<f32>>, b: &Vec<Vec<f32>>) -> Vec<Vec<f32>>  {
    let mut new_matrix: Vec<Vec<f32>> = vec![];
    if a[0].len() == b.len()    {
        for i in 0..a.len() {
            let mut new_layer: Vec<f32> = vec![];
            for j in 0..b[0].len()    {
                let mut sum:f32 = 0.0;
                for k in 0..a[0].len() {
                    sum += b[k][j] * a[i][k];//a[i][k] * b[j][k];
                }
                new_layer.push(sum);
            }
            new_matrix.push(new_layer);
        }
    }
    new_matrix
}
pub fn mul_by_constant(m: &Vec<f32>, c: f32) -> Vec<Vec<f32>>    {
    let mut new_matrix = vec![];
    for i in m  {
        new_matrix.push(i * c);
    }
    vec![new_matrix]
}

pub fn transpose(a: &Vec<Vec<f32>>) -> Vec<Vec<f32>> {
    let mut new_matrix: Vec<Vec<f32>> = vec![];
    for i in 0..a[0].len() {
        let mut new_layer: Vec<f32> = vec![];
        for j in 0..a.len()  {
            new_layer.push(a[j][i]);
        }
        new_matrix.push(new_layer);
    }
    new_matrix
}

pub fn matrix_add(a: &Vec<Vec<f32>>, b: &Vec<Vec<f32>>) -> Vec<Vec<f32>>   {
    let mut new_matrix: Vec<Vec<f32>> = vec![];
    if a.len() == b.len()    {
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
