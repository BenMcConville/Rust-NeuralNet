use rand::Rng;
use super::matrixmath;


pub enum layerType  {
    Input(usize),
    Output(usize),
    Dense(usize),
}

pub struct DenseLayer  {
    pub weights: Vec<Vec<f32>>, 
    pub bias: Vec<Vec<f32>>
}

//Incorrect implementation (Not enough connections)
pub fn create_dense_layer() -> DenseLayer {
    DenseLayer  {
        weights:    vec![],
        bias:       vec![]
    }
}

impl DenseLayer  {
    pub fn init_dense_layer(&mut self, n: usize, m: usize)    {
        let mut rng = rand::thread_rng();
        for _j in 0..n  {
            let mut new_node:Vec<f32> = vec![];
            for _i in 0..m {
                let a = rng.gen::<f32>();
                new_node.push(a);
            }
            self.weights.push(new_node);
        }
        self.bias    = vec![vec![0.0; m]; 1];// 1 represents the dimentionality of the layer
    }
    pub fn propagate_layer(&self, input: &Vec<Vec<f32>>) -> Vec<Vec<f32>>  {
        let mut new_matrix = matrixmath::matrix_mul(&input, &self.weights);
        println!("Weights: {:?}", &self.weights);
        println!("Here: {:?}", &input);
        //new_matrix = matrixmath::matrix_add(&new_matrix, &self.bias);
        //new_matrix = activation_function_loop(&new_matrix);
        new_matrix
    }

    pub fn print_layer_weights(&self)    {
        println!("{:?}", self.weights);
    }
    pub fn print_layer_bais(&self)    {
        println!("{:?}", self.bias);
    }
}
fn activation_function_loop(a: &Vec<f32>) -> Vec<f32>    {
        let mut new_matrix: Vec<f32> = vec![];
        for i in a {
           new_matrix.push(activation_function(*i)); 
        }
        new_matrix
}
fn activation_function(a: f32) -> f32   {   //ReUL
    if a > 0.0    {
        a
    } else  {
        0.0
    }
}
