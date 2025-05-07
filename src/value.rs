use im::Vector;
use crate::types::Type;
use crate::unique::Unique;
use std::sync::Arc;

use num_bigint::{BigUint,BigInt};
use num_rational::BigRational;
use num_traits::*;

#[derive(Debug,Clone,PartialEq)]
pub enum Value{
    Num(Num),
    Type(Type),
    Flag(Unique),
    Func(Arc<Function>),
    Array(Arc<[Value]>)
}

impl Value{
    pub fn bool_value(&self) -> Option<bool>{
        match self{
            Value::Flag(f)=>f.bool_value(),
            _=>None
        }
    }
}

#[derive(Debug,Clone,PartialEq)]
pub enum Num{
    Nat(BigUint),
    Int(BigInt),
    Frac(BigRational),
    Float(f64),   
}

#[derive(Debug,PartialEq)]
pub struct Function{
    pub arguments: Vector<Type>,
    pub output: Type,
    pub code: Code,
}

impl Function{
    pub fn try_run(&self,inputs:&[Value]) -> Result<Value,()>{todo!()}
}

#[derive(Debug,Clone,PartialEq)]
pub struct Code;



