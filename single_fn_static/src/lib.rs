pub trait SingleFnStatic{
	type Input;
	type Output;
	fn call(input:Self::Input)->Self::Output;
}