use super::layer;
use super::matrixmath;

pub struct Network  {
    pub layers: Vec<layer::DenseLayer>
}

pub fn create_network() -> Network  {
    Network {
        layers: vec![]
    }
}

impl Network    {
    pub fn init_network(&mut self, structure: &Vec<layer::layerType>)   {
        let mut previous_layer_size: usize = 0;
        for i in structure  {
            match i {
                layer::layerType::Input(a) => {
                    previous_layer_size = *a;
                }
                layer::layerType::Output(a) =>    {
                    let mut new_layer = layer::create_dense_layer();
                    new_layer.init_dense_layer(previous_layer_size, *a);
                    self.layers.push(new_layer);
                }
                layer::layerType::Dense(a) => {
                    let mut new_layer = layer::create_dense_layer();
                    new_layer.init_dense_layer(previous_layer_size, *a);
                    previous_layer_size = *a;
                    self.layers.push(new_layer);
                }
            }
        }
    }  
    pub fn print(&self) {
        for i in self.layers.iter()  {
            print!("\nWeights: ");
            i.print_layer_weights();
            print!("Bias: ");
            i.print_layer_bais();
            print!("\n");
        
        }
    }
    pub fn propagate(&self, input: Vec<Vec<f32>>) -> Vec<Vec<f32>>    {
        let mut new_matrix: Vec<Vec<f32>> = input;
        for i in &self.layers  {
            new_matrix = i.propagate_layer(&new_matrix);
        }    
        new_matrix
    }
}
