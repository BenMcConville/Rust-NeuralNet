pub mod matrixmath;
pub mod network;
pub mod layer;
pub mod trainer;

fn main() {
    let mut structure: Vec<layer::layerType> = vec![
        layer::layerType::Input(3),
        layer::layerType::Dense(2),
        layer::layerType::Output(2)];

    let mut a: Vec<Vec<f32>> = vec![vec![3.0,4.0,2.0], vec![1.0,4.0,6.0],vec![3.0,2.0,9.0]];
    let mut va = network::create_network();
    va.init_network(&structure);
    //va.print();
    let mut input: Vec<Vec<f32>> = vec![vec![2.0,4.0,-2.0]];
    let a = va.propagate(input);
    println!("{:?}", a); 
    
    println!(" -----------?");
    let sol: Vec<Vec<f32>> = vec![vec![1.0,0.0]];
    trainer::back_prop_network(va, &a, &sol);
}
