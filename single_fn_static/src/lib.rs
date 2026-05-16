pub trait SingleFnStatic{
	type Input;
	type Output;
	fn call_static(input:Self::Input)->Self::Output;
}