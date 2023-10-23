use super::matrixmath;
use super::layer;
use super::network;

pub fn softmax(m: &Vec<f32>) -> Vec<Vec<f32>> {
    let mut new_matrix: Vec<Vec<f32>> = vec![];
    let mut exp_sum = 0.0;
    for i in m.iter()   {
        exp_sum += i.exp();
    }
    let mut new_layer: Vec<f32> = vec![];
    for i in m.iter()   {
        new_layer.push(i.exp()/exp_sum);
    }
    new_matrix.push(new_layer);
    new_matrix
}
pub fn cross_entropy_loss(m: Vec<f32>, solution: Vec<f32>) -> f32    {
    let mut loss: f32 = 0.0;
    for i in 0..m.len()   {
        loss += m[i].log2()*solution[i];
    }
    loss * -1.0
}

//Back Prop is front prop but just subtracting numbers.
pub fn back_prop_network(n: network::Network, guess: &Vec<Vec<f32>>, sol: &Vec<Vec<f32>>) -> network::Network {
    let y = matrixmath::mul_by_constant(&sol[0], -1.0);
    let y_y = matrixmath::matrix_add(&guess, &y);     
    for i in (0..n.layers.len()).rev()  {
        let mutlayer_sols: Vec<Vec<f32>> = vec![vec![]];
        for j in (0..n.layers[i].weights.len()).rev()  {
            for k in 0..n.layers[i].weights[j].len()   {
                println!("{} {} {}", i, j, k); 
                
                println!("{:?}", matrixmath::transpose(&n.layers[i].weights));
            
            }
        }
    }
    n
}
