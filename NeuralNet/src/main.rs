pub mod matrixmath;
pub mod network;
pub mod layer;
pub mod trainer;

fn main() {
    let mut structure: Vec<layer::layerType> = vec![
        layer::layerType::Input(2),
        layer::layerType::Dense(2),
        layer::layerType::Output(1)];

    let mut va = network::create_network();
    va.init_network(&structure);
    va.print();
    let mut input: Vec<Vec<f32>> = vec![vec![1.0,1.0]];
    let a = va.propagate(input);
    println!("{:?}", a); 
    //println!("{:?}", trainer::softmax(a)); 
}
