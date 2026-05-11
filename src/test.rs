use std::ops::AddAssign;

use named_type_for_fn_proc_macro::name_type;
use single_fn_static::SingleFnStatic;


#[name_type(Test1Fn)]
pub fn test1_fn<T:AddAssign+Copy>(mut a:T)->T{
	a+=a;
	a
}
#[test]
pub fn test(){

	let b=Test1Fn::call(2);
	assert_eq!(b,4);
	let dwawd:<Test1Fn<i32> as SingleFnStatic>::Output=2;
}