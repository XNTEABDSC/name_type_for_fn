

use proc_macro::TokenStream;
use quote::quote;
use syn::{FnArg, Ident, ItemFn, ReturnType, parse_macro_input, parse_quote};

/// 属性宏，用法：`#[name_type(StructName)]`
/// 作用于单参数函数，生成一个同名的结构体并为它实现 `SingleFunc`
#[proc_macro_attribute]
pub fn name_type(attr: TokenStream, item: TokenStream) -> TokenStream {
    // 解析属性参数（结构体名称）
    let struct_name: Ident = parse_macro_input!(attr as Ident);

    // 解析函数定义
    let mut input_fn: ItemFn = parse_macro_input!(item as ItemFn);

    // 检查函数参数个数（必须只有一个参数）
    let fn_args = &input_fn.sig.inputs;
    if fn_args.len() != 1 {
        return syn::Error::new_spanned(
            &input_fn.sig,
            "this macro only supports functions with exactly one parameter",
        )
        .to_compile_error()
        .into();
    }

    // 提取参数类型和返回类型
    let arg_pat = match fn_args.first().unwrap() {
        FnArg::Typed(pat_type) => pat_type,
        FnArg::Receiver(_) => {
            return syn::Error::new_spanned(
                &input_fn.sig,
                "method receiver `self` is not supported",
            )
            .to_compile_error()
            .into();
        }
    };
    let input_ty = &arg_pat.ty;
    let output_ty = match &input_fn.sig.output {
        ReturnType::Default => &parse_quote! { () }, // 无返回值时返回单元类型
        ReturnType::Type(_, ty) => ty.as_ref(),
    };

    // 获取函数的泛型参数和 where 子句
    let generics = input_fn.sig.generics.clone();
    let where_clause = input_fn.sig.generics.where_clause.clone();

    // 构造结构体的泛型参数（与函数完全一致）
    // 注意：结构体定义中的泛型约束可以省略，但这里为了明确与函数保持一致，保留约束
    // 由于结构体字段是 PhantomData，我们还需要确保所有类型参数都被使用，否则会有警告
    // 因此需要生成一个包含所有类型参数的 PhantomData<(...)>
    let type_params: Vec<_> = generics
        .params
        .iter()
        .filter_map(|param| {
            if let syn::GenericParam::Type(ty) = param {
                Some(ty.ident.clone())
            } else {
                None
            }
        })
        .collect();

    // 如果存在非类型泛型（如生命周期或 const），我们暂时忽略它们，但为了消除未使用警告，
    // 可以在结构体定义上添加 allow(unused) 属性（简单处理）
    let has_non_type_param = generics
        .params
        .iter()
        .any(|p| !matches!(p, syn::GenericParam::Type(_)));

    // 为结构体生成 PhantomData 字段的类型标记
    let phantom_ty = if type_params.is_empty() {
        // 没有类型参数时使用 ()，避免空元组导致语法错误
        quote! { () }
    } else {
        quote! { ( #(#type_params),* ) }
    };

    // 生成结构体定义
    let struct_def = if has_non_type_param {
        // 如果存在非类型参数，添加允许未使用的属性
        quote! {
            #[allow(unused)]
            pub struct #struct_name #generics (::std::marker::PhantomData<#phantom_ty>)
            #where_clause ;
        }
    } else {
        quote! {
            pub struct #struct_name #generics (::std::marker::PhantomData<#phantom_ty>)
            #where_clause ;
        }
    };

    // 生成 impl SingleFuncStatic 块
    let fn_name = input_fn.sig.ident.clone();
    let impl_generics = generics.clone();
    let (impl_generics_without_defaults, ty_generics, where_clause_for_impl) =
        impl_generics.split_for_impl();

	let turbo_fish=ty_generics.as_turbofish();
    // 注意：我们需要使用 'static 版本的泛型参数（与函数定义同一组）
    let impl_block = quote! {
        impl #impl_generics_without_defaults name_type_for_fn::SingleFnStatic for #struct_name #ty_generics #where_clause_for_impl {
            type Input = #input_ty;
            type Output = #output_ty;

            fn call_static(input: Self::Input) -> Self::Output {
                #fn_name #turbo_fish (input)
            }
        }
    };

    // 移除原函数上的这个宏属性，避免属性宏被重复应用导致无限递归
    input_fn.attrs.retain(|attr| !attr.path().is_ident("name_type"));

    // 输出：原函数（已去掉本属性） + 生成的结构体 + impl
    let expanded = quote! {
        #input_fn
        #struct_def
        #impl_block
    };

    TokenStream::from(expanded)
}

// #[proc_macro_attribute]
// pub fn fn_type_name(args:TokenStream,input:TokenStream) -> TokenStream{
// 	let args = parse_macro_input!(args as Ident);
// 	let type_name=args.to_string();
// 	let function=parse_macro_input!(input as ItemFn);
// 	let sig=function.sig;
// 	let generics=sig.generics;
// 	let fn_block = &function.block;
// 	let a:std::marker::PhantomData;

// 	let generic_args=generics.split_for_impl();
// 	// let generic_type=TypeGenerics(&generics);

// 	// let dawdawd=Token
// 	let dwa=generic_args.1.;
// 	let type_def=ItemStruct{
// 		ident:args,
// 		vis:function.vis,
// 		generics:generics,
// 		fields:syn::Fields::Named(FieldsNamed{
// 			brace_token:Default::default(),
// 			named:{
// 				let mut a=Punctuated::new();
// 				a.push(Field{
// 					vis:syn::Visibility::Public(Default::default()),
// 					ident:Some(Ident::new("_p", Span::mixed_site())),
// 					ty:syn::Type::Verbatim(quote! {std::marker::PhantomData<()>})
// 				});
// 				a
// 			}
// 		})
// 	};
// 	// let type_def=quote! {
// 	// 	struct #type_name 
// 	// }
// 	// let def=ItemStruct::
// }