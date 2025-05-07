use std::collections::HashSet;
use crate::value::Value;
use crate::value::Function;
use std::sync::Arc;
use crate::unique::Unique;

use im::Vector;
use rayon::prelude::*;

#[derive(Debug,Clone,PartialEq)]
pub enum Generic{
	Knowen(Type),
	Ref(usize),
	Empty
}

impl Generic{
	pub fn get_next(self,v:Vector<Generic>)->Result<Generic,usize>{
		if let Generic::Ref(id) = self{
			let mut visited = HashSet::new();
			visited.insert(id);

			let mut cur = &v[id];
			while let Generic::Ref(id) = cur{
				if visited.insert(*id){
					return Err(*id);
				}
				cur = &v[*id];
			}
			Ok(cur.clone())

		}else{
			Ok(self)
		}
		
	}
}

#[derive(Debug,Clone,PartialEq)]
pub struct BoundType{
	pub generics:Vector<Generic>,
	tree:Type
}

//specific type stuff

/// captures the current generic enviorment to insert into a new type
#[derive(Debug,Clone,PartialEq)]
pub struct Regenric{
	core:Arc<Type>,
	pub generics:Arc<[(usize,Generic)]>
}

#[derive(Debug,Clone,PartialEq)]
pub struct Array{
	pub core:Type,
	pub len: Generic,
}

#[derive(Debug,Clone,PartialEq)]
pub enum Type{
	Basic(Unique),//unique identifier for anything from bools to enums
	Exact(Arc<Value>),

	Generic(usize),
	WithGenerics(Regenric),

	Filtered(Arc<Function>,usize),

	Func(Arc<[Type]>,Arc<Type>,usize),
	Array(Arc<Array>,usize),
	Tuple(Arc<[Type]>,usize),
	Union(Arc<[Type]>,usize),
}

impl Type{
	#[cfg(test)]
	pub (crate) fn assert_types(&self) -> usize{
		match self{
			Type::Basic(_) | Type::Exact(_)=> 0,
			Type::Generic(_)=>1,
			Type::WithGenerics(x)=>todo!(),//x.generics.len(),

			Type::Func(args,val,total) => {
				assert_eq!(
					*total,

					args.par_iter().map(Type::assert_types).sum::<usize>()
					+val.assert_types(),
				);
				*total
			}
			Type::Array(arr,total)=>{
				assert_eq!(*total,1+arr.core.assert_types());
				*total
			}

			Type::Tuple(v,total)|Type::Union(v,total)=>{
				assert_eq!(*total,v.par_iter().map(Type::assert_types).sum());
				*total
			}
			Type::Filtered(_, _)=>todo!()
		}
	}

	// pub fn matches (&self,v: &Value,genrics:Vector<Generic>) -> bool {
	// 	match self {
	// 	    Type::Basic(b)=> match v{
	// 	    	Value::Num(n) => n.get_type()==*self,
	// 	    	Value::Flag(f) => *f==*b,
	// 	    	_=> false
	// 	    },
	// 	    Type::Exact(e)=> **e==*v,

	// 	    Type::Union(r) => r.iter().any(|t| t.matches(v,genrics.clone())),
	// 	    Type::Tuple(at) => {
	// 	    	let Value::Array(av) = v else { return false};
	// 	    	if av.len()!=at.len(){
	// 	    		return false;
	// 	    	}

	// 	    	av.iter().zip(at.iter()).all(|(v, t)| t.matches(v,genrics.clone()))
	// 	    },

	// 	    Type::Filtered(f) => match f.try_run(&[v.clone()]){
	// 	    	Err(()) => false,
	// 	    	Ok(x) => x.bool_value().expect("condition functions must return bool"),
	// 	    },
		    
	// 	    //needs contravrient
	// 	    Type::Func(_, _)=>todo!(),

	// 	    // Type::Func(args, value) => match v {
	// 	    // 	Value::Func(f)=> *f.arguments==**args && f.output==**value,
	// 	    // 	_=>false
	// 	    // },
	// 	    Type::Generic(_) => todo!(),
	// 	    Type::WithGenerics(_, _) => todo!()
	// 	}
	// }
}