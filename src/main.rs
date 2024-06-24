use rust_arith_circuit::circuit::ArithmeticCircuit;

fn main() {
    let expr = "(x+y)*y";
    let mut circuit = ArithmeticCircuit::from_expression(expr);
    let result = circuit.evaluate(&[("x", 2), ("y", 3)]);
    println!("Result: {}", result); // Should print 15
    let solidity_code = circuit.to_solidity();
    println!("{}", solidity_code);
}