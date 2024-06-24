# Rust Arithmetic Circuit Compiler

This project is an arithmetic circuit compiler written in Rust, for educational purposes only. It parses arithmetic expressions, builds a corresponding circuit, evaluates the circuit with given inputs, and generates Solidity smart contract code to verify the circuit.

## Features

- Parse arithmetic expressions.
- Build arithmetic circuits from parsed expressions.
- Evaluate circuits with given input values.
- Generate Solidity smart contract code for circuit verification.

## Dependencies

- [nom](https://crates.io/crates/nom) for parsing arithmetic expressions.

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) programming language and Cargo.

### Installing

Clone the repository:

    git clone https://github.com/your-username/rust-arith-circuit.git
    cd rust-arith-circuit

### Building

To build the project, run:

    cargo build

### Running

To run the project, use:

    cargo run

This will parse the expression x + y, evaluate it with x = 2 and y = 5, and generate the corresponding Solidity smart contract.

Example Output
The expected output should be:

    Result: 7

    pragma solidity ^0.8.0;

    contract ArithmeticCircuit {
        function verify(int256[] memory x, int256[] memory y) public pure returns (int256) {
            return x[0] + y[1];
        }
    }

### Project Structure

src/main.rs: Entry point of the application.
src/circuit.rs: Contains the definition of the arithmetic circuit, parsing logic, evaluation, and Solidity code generation.

### Usage

Modify the expression in main.rs to your desired arithmetic expression.
Run the project to see the evaluated result and generated Solidity code.

### Contributing
Contributions are welcome! Please open an issue or submit a pull request for any improvements or bug fixes.

### License
This project is licensed under the MIT License - see the LICENSE file for details.
