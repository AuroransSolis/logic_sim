#![allow(dead_code, unused_variables)]

pub mod board;

pub(crate) use board::gate::Gate;
pub(crate) use board::circuit::Circuit;
pub(crate) use board::board::Board;

fn main() {
    // Making the board
    let mut board = Board::new();
    // Making the first circuit
    let mut c1 = Circuit::new();
    let nor_1 = c1.add_gate(Gate::new_nor());
    let nor_2 = c1.add_gate(Gate::new_nor());
    c1.connect_i2(nor_1, nor_2);
    c1.connect_i1(nor_2, nor_1);
    c1.update_all_inputs();
    // Adding the first circuit to the board
    let c1 = board.add_circuit(c1);
    // Making the second circuit
    let mut c2 = Circuit::new();
    let toggle_low = c2.add_gate(Gate::new_source());
    let toggle_high = c2.add_gate(Gate::new_source());
    // Adding the second circuit to the board
    let c2 = board.add_circuit(c2);
    // Connecting toggle_high of c2 to i1 of nor_1 of c1
    board.make_inter_circuit_i1_connection(c1, nor_1, c2, toggle_high);
    // Connecting toggle_low of c2 to i2 of nor_2 of c1
    board.make_inter_circuit_i2_connection(c1, nor_2, c2, toggle_low);
    // Update the inputs of all gates in both circuits
    board[c1].update_all_inputs();
    board[c2].update_all_inputs();
    // Evaluate the initial board state by evaluating all gates twice
    board.eval_all_n_passes(2);
    // Print the initial state of the NOR gates
    println!("nor 1: {:?}, nor 2: {:?}", board[c1][nor_1].get_output(), board[c1][nor_2].get_output());
    // Toggle toggle_high and evaluate the board state after each step
    board[c2][toggle_high].set_high();
    board.eval_all_n_passes(2);
    board[c2][toggle_high].set_low();
    board.eval_all_n_passes(2);
    println!("nor 1: {:?}, nor 2: {:?}", board[c1][nor_1].get_output(), board[c1][nor_2].get_output());
    // Toggle toggle_low and evaluate the board state after each step
    board[c2][toggle_low].set_high();
    board.eval_all_n_passes(2);
    board[c2][toggle_low].set_low();
    board.eval_all_n_passes(2);
    println!("nor 1: {:?}, nor 2: {:?}", board[c1][nor_1].get_output(), board[c1][nor_2].get_output());
}