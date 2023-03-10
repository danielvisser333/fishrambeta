mod math;
mod parser;

use clap::Parser;
use crate::math::{Constant, Equation, Symbol, Variable};

#[derive(Parser, Debug)]
pub struct Args{
    //The equation to solve formatted in LaTeX
    #[arg(short, long)]
    equation : String,
}

fn main() {
    let addition = Equation::Addition(
        vec!(
            Equation::Variable(Variable::Constant(Constant::PI)),
            Equation::Variable(Variable::Constant(Constant::PI)),
        )
    );
    let multiplication = Equation::Multiplication(
        vec!(
            addition.clone(),
            addition.clone(),
        )
    );

    let simplified = multiplication.simplify().simplify().simplify();

    println!("{:?}", simplified)
    //let args = Args::parse();
}
