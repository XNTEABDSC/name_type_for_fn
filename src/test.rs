use std::ops::AddAssign;

use name_type_for_fn_proc_macro::name_type;
use single_fn_static::SingleFnStatic;



#[name_type(Test1Fn)]
pub fn test1_fn<T:Copy,const DIM:usize>(mut a:T)->T
	where T:AddAssign<T>+AddAssign<usize>
{
	for i in 0..DIM {
		a+=a;
		a+=i;
	}
	a
}
#[test]
pub fn test(){

	let b=Test1Fn::<_,2>::call_static(2usize);
	assert_eq!(b,4);
	let _dwawd:<Test1Fn<usize,2> as SingleFnStatic>::Output=2;
}