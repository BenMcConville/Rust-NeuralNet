use super::matrixmath;
use super::layer;

pub fn softmax(m: Vec<f32>) -> Vec<f32> {
    let mut new_matrix = vec![];
    let mut exp_sum = 0.0;
    for i in m.iter()   {
        exp_sum += i.exp();
    }
    for i in m.iter()   {
        new_matrix.push(i.exp()/exp_sum);
    }
    new_matrix
}
/*
pub fn cross_entropy_loss(m: Vec<f32>, solution: f32) -> f32    {
    let mut loss: f32 = 0.0;
    for i in m.iter()   {
        loss += softmax(i).log2()
    }
}*/
