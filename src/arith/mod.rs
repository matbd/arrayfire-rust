extern crate libc;
extern crate num;

use self::libc::c_int;
use self::num::Complex;
use crate::array::Array;
use crate::data::{constant, tile, ConstGenerator};
use crate::defines::AfError;
use crate::dim4::Dim4;
use crate::error::HANDLE_ERROR;
use crate::num::Zero;
use crate::util::{AfArray, HasAfEnum, ImplicitPromote, MutAfArray};
use std::ops::Neg;
use std::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Not, Rem, Shl, Shr, Sub};

#[allow(dead_code)]
extern "C" {
    fn af_add(out: MutAfArray, lhs: AfArray, rhs: AfArray, batch: c_int) -> c_int;
    fn af_sub(out: MutAfArray, lhs: AfArray, rhs: AfArray, batch: c_int) -> c_int;
    fn af_mul(out: MutAfArray, lhs: AfArray, rhs: AfArray, batch: c_int) -> c_int;
    fn af_div(out: MutAfArray, lhs: AfArray, rhs: AfArray, batch: c_int) -> c_int;

    fn af_lt(out: MutAfArray, lhs: AfArray, rhs: AfArray, batch: c_int) -> c_int;
    fn af_gt(out: MutAfArray, lhs: AfArray, rhs: AfArray, batch: c_int) -> c_int;
    fn af_le(out: MutAfArray, lhs: AfArray, rhs: AfArray, batch: c_int) -> c_int;
    fn af_ge(out: MutAfArray, lhs: AfArray, rhs: AfArray, batch: c_int) -> c_int;
    fn af_eq(out: MutAfArray, lhs: AfArray, rhs: AfArray, batch: c_int) -> c_int;
    fn af_or(out: MutAfArray, lhs: AfArray, rhs: AfArray, batch: c_int) -> c_int;

    fn af_neq(out: MutAfArray, lhs: AfArray, rhs: AfArray, batch: c_int) -> c_int;
    fn af_and(out: MutAfArray, lhs: AfArray, rhs: AfArray, batch: c_int) -> c_int;
    fn af_rem(out: MutAfArray, lhs: AfArray, rhs: AfArray, batch: c_int) -> c_int;
    fn af_mod(out: MutAfArray, lhs: AfArray, rhs: AfArray, batch: c_int) -> c_int;

    fn af_bitand(out: MutAfArray, lhs: AfArray, rhs: AfArray, batch: c_int) -> c_int;
    fn af_bitor(out: MutAfArray, lhs: AfArray, rhs: AfArray, batch: c_int) -> c_int;
    fn af_bitxor(out: MutAfArray, lhs: AfArray, rhs: AfArray, batch: c_int) -> c_int;
    fn af_bitshiftl(out: MutAfArray, lhs: AfArray, rhs: AfArray, batch: c_int) -> c_int;
    fn af_bitshiftr(out: MutAfArray, lhs: AfArray, rhs: AfArray, batch: c_int) -> c_int;
    fn af_minof(out: MutAfArray, lhs: AfArray, rhs: AfArray, batch: c_int) -> c_int;
    fn af_maxof(out: MutAfArray, lhs: AfArray, rhs: AfArray, batch: c_int) -> c_int;
    fn af_clamp(out: MutAfArray, inp: AfArray, lo: AfArray, hi: AfArray, batch: c_int) -> c_int;

    fn af_not(out: MutAfArray, arr: AfArray) -> c_int;
    fn af_abs(out: MutAfArray, arr: AfArray) -> c_int;
    fn af_arg(out: MutAfArray, arr: AfArray) -> c_int;
    fn af_sign(out: MutAfArray, arr: AfArray) -> c_int;
    fn af_ceil(out: MutAfArray, arr: AfArray) -> c_int;
    fn af_round(out: MutAfArray, arr: AfArray) -> c_int;
    fn af_trunc(out: MutAfArray, arr: AfArray) -> c_int;
    fn af_floor(out: MutAfArray, arr: AfArray) -> c_int;

    fn af_hypot(out: MutAfArray, lhs: AfArray, rhs: AfArray, batch: c_int) -> c_int;

    fn af_sin(out: MutAfArray, arr: AfArray) -> c_int;
    fn af_cos(out: MutAfArray, arr: AfArray) -> c_int;
    fn af_tan(out: MutAfArray, arr: AfArray) -> c_int;
    fn af_asin(out: MutAfArray, arr: AfArray) -> c_int;
    fn af_acos(out: MutAfArray, arr: AfArray) -> c_int;
    fn af_atan(out: MutAfArray, arr: AfArray) -> c_int;

    fn af_atan2(out: MutAfArray, lhs: AfArray, rhs: AfArray, batch: c_int) -> c_int;
    fn af_cplx2(out: MutAfArray, lhs: AfArray, rhs: AfArray, batch: c_int) -> c_int;
    fn af_root(out: MutAfArray, lhs: AfArray, rhs: AfArray, batch: c_int) -> c_int;
    fn af_pow(out: MutAfArray, lhs: AfArray, rhs: AfArray, batch: c_int) -> c_int;

    fn af_cplx(out: MutAfArray, arr: AfArray) -> c_int;
    fn af_real(out: MutAfArray, arr: AfArray) -> c_int;
    fn af_imag(out: MutAfArray, arr: AfArray) -> c_int;
    fn af_conjg(out: MutAfArray, arr: AfArray) -> c_int;
    fn af_sinh(out: MutAfArray, arr: AfArray) -> c_int;
    fn af_cosh(out: MutAfArray, arr: AfArray) -> c_int;
    fn af_tanh(out: MutAfArray, arr: AfArray) -> c_int;
    fn af_asinh(out: MutAfArray, arr: AfArray) -> c_int;
    fn af_acosh(out: MutAfArray, arr: AfArray) -> c_int;
    fn af_atanh(out: MutAfArray, arr: AfArray) -> c_int;
    fn af_pow2(out: MutAfArray, arr: AfArray) -> c_int;
    fn af_exp(out: MutAfArray, arr: AfArray) -> c_int;
    fn af_sigmoid(out: MutAfArray, arr: AfArray) -> c_int;
    fn af_expm1(out: MutAfArray, arr: AfArray) -> c_int;
    fn af_erf(out: MutAfArray, arr: AfArray) -> c_int;
    fn af_erfc(out: MutAfArray, arr: AfArray) -> c_int;
    fn af_log(out: MutAfArray, arr: AfArray) -> c_int;
    fn af_log1p(out: MutAfArray, arr: AfArray) -> c_int;
    fn af_log10(out: MutAfArray, arr: AfArray) -> c_int;
    fn af_log2(out: MutAfArray, arr: AfArray) -> c_int;
    fn af_sqrt(out: MutAfArray, arr: AfArray) -> c_int;
    fn af_cbrt(out: MutAfArray, arr: AfArray) -> c_int;
    fn af_factorial(out: MutAfArray, arr: AfArray) -> c_int;
    fn af_tgamma(out: MutAfArray, arr: AfArray) -> c_int;
    fn af_lgamma(out: MutAfArray, arr: AfArray) -> c_int;
    fn af_iszero(out: MutAfArray, arr: AfArray) -> c_int;
    fn af_isinf(out: MutAfArray, arr: AfArray) -> c_int;
    fn af_isnan(out: MutAfArray, arr: AfArray) -> c_int;
}

/// Enables use of `!` on objects of type [Array](./struct.Array.html)
impl<'f, T> Not for &'f Array<T>
where
    T: HasAfEnum,
{
    type Output = Array<T>;

    fn not(self) -> Self::Output {
        let mut temp: i64 = 0;
        unsafe {
            let err_val = af_not(&mut temp as MutAfArray, self.get() as AfArray);
            HANDLE_ERROR(AfError::from(err_val));
        }
        temp.into()
    }
}

macro_rules! unary_func {
    [$doc_str: expr, $fn_name: ident, $ffi_fn: ident, $out_type: ident] => (
        #[doc=$doc_str]
        ///
        /// This is an element wise unary operation.
        #[allow(unused_mut)]
        pub fn $fn_name<T: HasAfEnum>(input: &Array<T>) -> Array< T::$out_type >
        where T::$out_type: HasAfEnum {
            let mut temp: i64 = 0;
            unsafe {
                let err_val = $ffi_fn(&mut temp as MutAfArray, input.get() as AfArray);
                HANDLE_ERROR(AfError::from(err_val));
            }
            temp.into()
        }
    )
}

unary_func!("Computes absolute value", abs, af_abs, AbsOutType);
unary_func!("Computes phase value", arg, af_arg, ArgOutType);

unary_func!(
    "Truncate the values in an Array",
    trunc,
    af_trunc,
    AbsOutType
);
unary_func!(
    "Computes the sign of input Array values",
    sign,
    af_sign,
    AbsOutType
);
unary_func!("Round the values in an Array", round, af_round, AbsOutType);
unary_func!("Floor the values in an Array", floor, af_floor, AbsOutType);
unary_func!("Ceil the values in an Array", ceil, af_ceil, AbsOutType);

unary_func!("Compute sigmoid function", sigmoid, af_sigmoid, AbsOutType);
unary_func!(
    "Compute e raised to the power of value -1",
    expm1,
    af_expm1,
    AbsOutType
);
unary_func!("Compute error function value", erf, af_erf, AbsOutType);
unary_func!(
    "Compute the complementary error function value",
    erfc,
    af_erfc,
    AbsOutType
);

unary_func!("Compute logarithm base 10", log10, af_log10, AbsOutType);
unary_func!(
    "Compute the logarithm of input Array + 1",
    log1p,
    af_log1p,
    AbsOutType
);
unary_func!("Compute logarithm base 2", log2, af_log2, AbsOutType);

unary_func!("Compute the cube root", cbrt, af_cbrt, AbsOutType);
unary_func!("Compute gamma function", tgamma, af_tgamma, AbsOutType);
unary_func!(
    "Compute the logarithm of absolute values of gamma function",
    lgamma,
    af_lgamma,
    AbsOutType
);

unary_func!("Compute acosh", acosh, af_acosh, UnaryOutType);
unary_func!("Compute acos", acos, af_acos, UnaryOutType);
unary_func!("Compute asin", asin, af_asin, UnaryOutType);
unary_func!("Compute asinh", asinh, af_asinh, UnaryOutType);
unary_func!("Compute atan", atan, af_atan, UnaryOutType);
unary_func!("Compute atanh", atanh, af_atanh, UnaryOutType);
unary_func!("Compute cos", cos, af_cos, UnaryOutType);
unary_func!("Compute cosh", cosh, af_cosh, UnaryOutType);
unary_func!(
    "Compute e raised to the power of value",
    exp,
    af_exp,
    UnaryOutType
);
unary_func!("Compute the natural logarithm", log, af_log, UnaryOutType);
unary_func!("Compute sin", sin, af_sin, UnaryOutType);
unary_func!("Compute sinh", sinh, af_sinh, UnaryOutType);
unary_func!("Compute the square root", sqrt, af_sqrt, UnaryOutType);
unary_func!("Compute tan", tan, af_tan, UnaryOutType);
unary_func!("Compute tanh", tanh, af_tanh, UnaryOutType);

unary_func!(
    "Extract real values from a complex Array",
    real,
    af_real,
    AbsOutType
);
unary_func!(
    "Extract imaginary values from a complex Array",
    imag,
    af_imag,
    AbsOutType
);
unary_func!(
    "Create a complex Array from real Array",
    cplx,
    af_cplx,
    ComplexOutType
);
unary_func!(
    "Compute the complex conjugate",
    conjg,
    af_conjg,
    ComplexOutType
);
unary_func!(
    "Compute two raised to the power of value",
    pow2,
    af_pow2,
    UnaryOutType
);
unary_func!(
    "Compute the factorial",
    factorial,
    af_factorial,
    UnaryOutType
);

macro_rules! unary_boolean_func {
    [$doc_str: expr, $fn_name: ident, $ffi_fn: ident] => (
        #[doc=$doc_str]
        ///
        /// This is an element wise unary operation.
        #[allow(unused_mut)]
        pub fn $fn_name<T: HasAfEnum>(input: &Array<T>) -> Array<bool> {
            let mut temp: i64 = 0;
            unsafe {
                let err_val = $ffi_fn(&mut temp as MutAfArray, input.get() as AfArray);
                HANDLE_ERROR(AfError::from(err_val));
            }
            temp.into()
        }
    )
}

unary_boolean_func!("Check if values are zero", iszero, af_iszero);
unary_boolean_func!("Check if values are infinity", isinf, af_isinf);
unary_boolean_func!("Check if values are NaN", isnan, af_isnan);

macro_rules! binary_func {
    ($doc_str: expr, $fn_name: ident, $ffi_fn: ident) => {
        #[doc=$doc_str]
        ///
        /// This is an element wise binary operation.
        #[allow(unused_mut)]
        pub fn $fn_name<A, B>(lhs: &Array<A>, rhs: &Array<B>, batch: bool) -> Array<A::Output>
        where
            A: HasAfEnum + ImplicitPromote<B>,
            B: HasAfEnum + ImplicitPromote<A>,
            <A as ImplicitPromote<B>>::Output: HasAfEnum,
        {
            let mut temp: i64 = 0;
            unsafe {
                let err_val = $ffi_fn(
                    &mut temp as MutAfArray,
                    lhs.get() as AfArray,
                    rhs.get() as AfArray,
                    batch as c_int,
                );
                HANDLE_ERROR(AfError::from(err_val));
            }
            Into::<Array<A::Output>>::into(temp)
        }
    };
}

binary_func!(
    "Elementwise AND(bit) operation of two Arrays",
    bitand,
    af_bitand
);
binary_func!(
    "Elementwise OR(bit) operation of two Arrays",
    bitor,
    af_bitor
);
binary_func!(
    "Elementwise XOR(bit) operation of two Arrays",
    bitxor,
    af_bitxor
);
binary_func!(
    "Elementwise not equals comparison of two Arrays",
    neq,
    af_neq
);
binary_func!(
    "Elementwise logical and operation of two Arrays",
    and,
    af_and
);
binary_func!("Elementwise logical or operation of two Arrays", or, af_or);
binary_func!(
    "Elementwise minimum operation of two Arrays",
    minof,
    af_minof
);
binary_func!(
    "Elementwise maximum operation of two Arrays",
    maxof,
    af_maxof
);
binary_func!(
    "Compute length of hypotenuse of two Arrays",
    hypot,
    af_hypot
);

/// Type Trait to convert to an [Array](./struct.Array.html)
///
/// Generic functions that overload the binary operations such as add, div, mul, rem, ge etc. are
/// bound by this trait to allow combinations of scalar values and Array objects as parameters
/// to those functions.
///
/// Internally, Convertable trait is implemented by following types.
///
/// - f32
/// - f64
/// - num::Complex\<f32\>
/// - num::Complex\<f64\>
/// - bool
/// - i32
/// - u32
/// - u8
/// - i64
/// - u64
/// - i16
/// - u16
///
pub trait Convertable {
    /// This type alias always points to `Self` which is the
    /// type of [Array](./struct.Array.html) returned by the
    /// trait method [convert](./trait.Convertable.html#tymethod.convert).
    type OutType;

    /// Get an Array of implementors type
    fn convert(&self) -> Array<Self::OutType>
    where
        <Self as Convertable>::OutType: HasAfEnum;
}

macro_rules! convertable_type_def {
    ($rust_type: ty) => {
        impl Convertable for $rust_type {
            type OutType = $rust_type;

            fn convert(&self) -> Array<Self::OutType> {
                constant(*self, Dim4::new(&[1, 1, 1, 1]))
            }
        }
    };
}

convertable_type_def!(Complex<f64>);
convertable_type_def!(Complex<f32>);
convertable_type_def!(u64);
convertable_type_def!(i64);
convertable_type_def!(f64);
convertable_type_def!(f32);
convertable_type_def!(i32);
convertable_type_def!(u32);
convertable_type_def!(i16);
convertable_type_def!(u16);
convertable_type_def!(u8);
convertable_type_def!(bool);

impl<T: HasAfEnum> Convertable for Array<T> {
    type OutType = T;

    fn convert(&self) -> Array<Self::OutType> {
        self.clone()
    }
}

macro_rules! overloaded_binary_func {
    ($doc_str: expr, $fn_name: ident, $help_name: ident, $ffi_name: ident) => {
        fn $help_name<A, B>(lhs: &Array<A>, rhs: &Array<B>, batch: bool) -> Array<A::Output>
        where
            A: HasAfEnum + ImplicitPromote<B>,
            B: HasAfEnum + ImplicitPromote<A>,
            <A as ImplicitPromote<B>>::Output: HasAfEnum,
        {
            let mut temp: i64 = 0;
            unsafe {
                let err_val = $ffi_name(
                    &mut temp as MutAfArray,
                    lhs.get() as AfArray,
                    rhs.get() as AfArray,
                    batch as c_int,
                );
                HANDLE_ERROR(AfError::from(err_val));
            }
            temp.into()
        }

        #[doc=$doc_str]
        ///
        /// This is a binary elementwise operation.
        ///
        ///# Parameters
        ///
        /// - `arg1`is an argument that implements an internal trait `Convertable`.
        /// - `arg2`is an argument that implements an internal trait `Convertable`.
        /// - `batch` is an boolean that indicates if the current operation is an batch operation.
        ///
        /// Both parameters `arg1` and `arg2` can be either an Array or a value of rust integral
        /// type.
        ///
        ///# Return Values
        ///
        /// An Array with results of the binary operation.
        ///
        ///# Note
        ///
        /// The trait `Convertable` essentially translates to a scalar native type on rust or Array.
        pub fn $fn_name<T, U>(
            arg1: &T,
            arg2: &U,
            batch: bool,
        ) -> Array<
            <<T as Convertable>::OutType as ImplicitPromote<<U as Convertable>::OutType>>::Output,
        >
        where
            T: Convertable,
            U: Convertable,
            <T as Convertable>::OutType: HasAfEnum + ImplicitPromote<<U as Convertable>::OutType>,
            <U as Convertable>::OutType: HasAfEnum + ImplicitPromote<<T as Convertable>::OutType>,
            <<T as Convertable>::OutType as ImplicitPromote<<U as Convertable>::OutType>>::Output:
                HasAfEnum,
        {
            let lhs = arg1.convert(); // Convert to Array<T>
            let rhs = arg2.convert(); // Convert to Array<T>
            match (lhs.is_scalar(), rhs.is_scalar()) {
                (true, false) => {
                    let l = tile(&lhs, rhs.dims());
                    $help_name(&l, &rhs, batch)
                }
                (false, true) => {
                    let r = tile(&rhs, lhs.dims());
                    $help_name(&lhs, &r, batch)
                }
                _ => $help_name(&lhs, &rhs, batch),
            }
        }
    };
}

overloaded_binary_func!("Addition of two Arrays", add, add_helper, af_add);
overloaded_binary_func!("Subtraction of two Arrays", sub, sub_helper, af_sub);
overloaded_binary_func!("Multiplication of two Arrays", mul, mul_helper, af_mul);
overloaded_binary_func!("Division of two Arrays", div, div_helper, af_div);
overloaded_binary_func!("Compute remainder from two Arrays", rem, rem_helper, af_rem);
overloaded_binary_func!("Compute left shift", shiftl, shiftl_helper, af_bitshiftl);
overloaded_binary_func!("Compute right shift", shiftr, shiftr_helper, af_bitshiftr);
overloaded_binary_func!(
    "Compute modulo of two Arrays",
    modulo,
    modulo_helper,
    af_mod
);
overloaded_binary_func!(
    "Calculate atan2 of two Arrays",
    atan2,
    atan2_helper,
    af_atan2
);
overloaded_binary_func!(
    "Create complex array from two Arrays",
    cplx2,
    cplx2_helper,
    af_cplx2
);
overloaded_binary_func!("Compute root", root, root_helper, af_root);
overloaded_binary_func!("Computer power", pow, pow_helper, af_pow);

macro_rules! overloaded_compare_func {
    ($doc_str: expr, $fn_name: ident, $help_name: ident, $ffi_name: ident) => {
        fn $help_name<A, B>(lhs: &Array<A>, rhs: &Array<B>, batch: bool) -> Array<bool>
        where
            A: HasAfEnum + ImplicitPromote<B>,
            B: HasAfEnum + ImplicitPromote<A>,
        {
            let mut temp: i64 = 0;
            unsafe {
                let err_val = $ffi_name(
                    &mut temp as MutAfArray,
                    lhs.get() as AfArray,
                    rhs.get() as AfArray,
                    batch as c_int,
                );
                HANDLE_ERROR(AfError::from(err_val));
            }
            temp.into()
        }

        #[doc=$doc_str]
        ///
        /// This is a comparison operation.
        ///
        ///# Parameters
        ///
        /// - `arg1`is an argument that implements an internal trait `Convertable`.
        /// - `arg2`is an argument that implements an internal trait `Convertable`.
        /// - `batch` is an boolean that indicates if the current operation is an batch operation.
        ///
        /// Both parameters `arg1` and `arg2` can be either an Array or a value of rust integral
        /// type.
        ///
        ///# Return Values
        ///
        /// An Array with results of the comparison operation a.k.a an Array of boolean values.
        ///# Note
        ///
        /// The trait `Convertable` essentially translates to a scalar native type on rust or Array.
        pub fn $fn_name<T, U>(
            arg1: &T,
            arg2: &U,
            batch: bool,
        ) -> Array<bool>
        where
            T: Convertable,
            U: Convertable,
            <T as Convertable>::OutType: HasAfEnum + ImplicitPromote<<U as Convertable>::OutType>,
            <U as Convertable>::OutType: HasAfEnum + ImplicitPromote<<T as Convertable>::OutType>,
        {
            let lhs = arg1.convert(); // Convert to Array<T>
            let rhs = arg2.convert(); // Convert to Array<T>
            match (lhs.is_scalar(), rhs.is_scalar()) {
                (true, false) => {
                    let l = tile(&lhs, rhs.dims());
                    $help_name(&l, &rhs, batch)
                }
                (false, true) => {
                    let r = tile(&rhs, lhs.dims());
                    $help_name(&lhs, &r, batch)
                }
                _ => $help_name(&lhs, &rhs, batch),
            }
        }
    };
}

overloaded_compare_func!(
    "Perform `less than` comparison operation",
    lt,
    lt_helper,
    af_lt
);
overloaded_compare_func!(
    "Perform `greater than` comparison operation",
    gt,
    gt_helper,
    af_gt
);
overloaded_compare_func!(
    "Perform `less than equals` comparison operation",
    le,
    le_helper,
    af_le
);
overloaded_compare_func!(
    "Perform `greater than equals` comparison operation",
    ge,
    ge_helper,
    af_ge
);
overloaded_compare_func!(
    "Perform `equals` comparison operation",
    eq,
    eq_helper,
    af_eq
);

fn clamp_helper<X, Y>(
    inp: &Array<X>,
    lo: &Array<Y>,
    hi: &Array<Y>,
    batch: bool,
) -> Array<<X as ImplicitPromote<Y>>::Output>
where
    X: HasAfEnum + ImplicitPromote<Y>,
    Y: HasAfEnum + ImplicitPromote<X>,
    <X as ImplicitPromote<Y>>::Output: HasAfEnum,
{
    let mut temp: i64 = 0;
    unsafe {
        let err_val = af_clamp(
            &mut temp as MutAfArray,
            inp.get() as AfArray,
            lo.get() as AfArray,
            hi.get() as AfArray,
            batch as c_int,
        );
        HANDLE_ERROR(AfError::from(err_val));
    }
    temp.into()
}

/// Clamp the values of Array
///
/// # Parameters
///
/// - `arg1`is an argument that implements an internal trait `Convertable`.
/// - `arg2`is an argument that implements an internal trait `Convertable`.
/// - `batch` is an boolean that indicates if the current operation is an batch operation.
///
/// Both parameters `arg1` and `arg2` can be either an Array or a value of rust integral
/// type.
///
/// # Return Values
///
/// An Array with results of the binary operation.
///
/// # Note
///
/// The trait `Convertable` essentially translates to a scalar native type on rust or Array.
pub fn clamp<T, C>(
    input: &Array<T>,
    arg1: &C,
    arg2: &C,
    batch: bool,
) -> Array<<T as ImplicitPromote<<C as Convertable>::OutType>>::Output>
where
    T: HasAfEnum + ImplicitPromote<<C as Convertable>::OutType>,
    C: Convertable,
    <C as Convertable>::OutType: HasAfEnum + ImplicitPromote<T>,
    <T as ImplicitPromote<<C as Convertable>::OutType>>::Output: HasAfEnum,
{
    let lo = arg1.convert(); // Convert to Array<T>
    let hi = arg2.convert(); // Convert to Array<T>
    match (lo.is_scalar(), hi.is_scalar()) {
        (true, false) => {
            let l = tile(&lo, hi.dims());
            clamp_helper(&input, &l, &hi, batch)
        }
        (false, true) => {
            let r = tile(&hi, lo.dims());
            clamp_helper(&input, &lo, &r, batch)
        }
        (true, true) => {
            let l = tile(&lo, input.dims());
            let r = tile(&hi, input.dims());
            clamp_helper(&input, &l, &r, batch)
        }
        _ => clamp_helper(&input, &lo, &hi, batch),
    }
}

macro_rules! arith_scalar_func {
    ($rust_type: ty, $op_name:ident, $fn_name: ident) => {
        impl<'f, T> $op_name<$rust_type> for &'f Array<T>
        where
            T: HasAfEnum + ImplicitPromote<$rust_type>,
            $rust_type: HasAfEnum + ImplicitPromote<T>,
            <T as ImplicitPromote<$rust_type>>::Output: HasAfEnum,
        {
            type Output = Array<<T as ImplicitPromote<$rust_type>>::Output>;

            fn $fn_name(self, rhs: $rust_type) -> Self::Output {
                let temp = rhs.clone();
                $fn_name(self, &temp, false)
            }
        }

        impl<T: HasAfEnum> $op_name<$rust_type> for Array<T>
        where
            T: HasAfEnum + ImplicitPromote<$rust_type>,
            $rust_type: HasAfEnum + ImplicitPromote<T>,
            <T as ImplicitPromote<$rust_type>>::Output: HasAfEnum,
        {
            type Output = Array<<T as ImplicitPromote<$rust_type>>::Output>;

            fn $fn_name(self, rhs: $rust_type) -> Self::Output {
                let temp = rhs.clone();
                $fn_name(&self, &temp, false)
            }
        }
    };
}

macro_rules! arith_scalar_spec {
    ($ty_name:ty) => {
        arith_scalar_func!($ty_name, Add, add);
        arith_scalar_func!($ty_name, Sub, sub);
        arith_scalar_func!($ty_name, Mul, mul);
        arith_scalar_func!($ty_name, Div, div);
    };
}

arith_scalar_spec!(Complex<f64>);
arith_scalar_spec!(Complex<f32>);
arith_scalar_spec!(f64);
arith_scalar_spec!(f32);
arith_scalar_spec!(u64);
arith_scalar_spec!(i64);
arith_scalar_spec!(u32);
arith_scalar_spec!(i32);
arith_scalar_spec!(u8);

macro_rules! arith_func {
    ($op_name:ident, $fn_name:ident, $delegate:ident) => {
        impl<A, B> $op_name<Array<B>> for Array<A>
        where
            A: HasAfEnum + ImplicitPromote<B>,
            B: HasAfEnum + ImplicitPromote<A>,
            <A as ImplicitPromote<B>>::Output: HasAfEnum,
        {
            type Output = Array<<A as ImplicitPromote<B>>::Output>;

            fn $fn_name(self, rhs: Array<B>) -> Self::Output {
                $delegate(&self, &rhs, false)
            }
        }

        impl<'a, A, B> $op_name<&'a Array<B>> for Array<A>
        where
            A: HasAfEnum + ImplicitPromote<B>,
            B: HasAfEnum + ImplicitPromote<A>,
            <A as ImplicitPromote<B>>::Output: HasAfEnum,
        {
            type Output = Array<<A as ImplicitPromote<B>>::Output>;

            fn $fn_name(self, rhs: &'a Array<B>) -> Self::Output {
                $delegate(&self, rhs, false)
            }
        }

        impl<'a, A, B> $op_name<Array<B>> for &'a Array<A>
        where
            A: HasAfEnum + ImplicitPromote<B>,
            B: HasAfEnum + ImplicitPromote<A>,
            <A as ImplicitPromote<B>>::Output: HasAfEnum,
        {
            type Output = Array<<A as ImplicitPromote<B>>::Output>;

            fn $fn_name(self, rhs: Array<B>) -> Self::Output {
                $delegate(self, &rhs, false)
            }
        }

        impl<'a, 'b, A, B> $op_name<&'a Array<B>> for &'b Array<A>
        where
            A: HasAfEnum + ImplicitPromote<B>,
            B: HasAfEnum + ImplicitPromote<A>,
            <A as ImplicitPromote<B>>::Output: HasAfEnum,
        {
            type Output = Array<<A as ImplicitPromote<B>>::Output>;

            fn $fn_name(self, rhs: &'a Array<B>) -> Self::Output {
                $delegate(self, rhs, false)
            }
        }
    };
}

arith_func!(Add, add, add);
arith_func!(Sub, sub, sub);
arith_func!(Mul, mul, mul);
arith_func!(Div, div, div);
arith_func!(Rem, rem, rem);
arith_func!(Shl, shl, shiftl);
arith_func!(Shr, shr, shiftr);
arith_func!(BitAnd, bitand, bitand);
arith_func!(BitOr, bitor, bitor);
arith_func!(BitXor, bitxor, bitxor);

#[cfg(op_assign)]
mod op_assign {

    use super::*;
    use crate::array::Array;
    use crate::index::{assign_gen, Indexer};
    use crate::seq::Seq;
    use std::mem;
    use std::ops::{AddAssign, DivAssign, MulAssign, RemAssign, SubAssign};
    use std::ops::{BitAndAssign, BitOrAssign, BitXorAssign, ShlAssign, ShrAssign};

    macro_rules! arith_assign_func {
        ($op_name:ident, $fn_name:ident, $func: ident) => {
            impl<A, B> $op_name<Array<B>> for Array<A>
            where
                A: HasAfEnum + ImplicitPromote<B>,
                B: HasAfEnum + ImplicitPromote<A>,
                <A as ImplicitPromote<B>>::Output: HasAfEnum,
                <B as ImplicitPromote<A>>::Output: HasAfEnum,
            {
                #[allow(unused_variables)]
                fn $fn_name(&mut self, rhs: Array<B>) {
                    let tmp_seq = Seq::<f32>::default();
                    let mut idxrs = Indexer::new();
                    for n in 0..self.numdims() {
                        idxrs.set_index(&tmp_seq, n, Some(false));
                    }
                    let opres = $func(self as &Array<A>, &rhs, false).cast::<A>();
                    let tmp = assign_gen(self as &Array<A>, &idxrs, &opres);
                    let old = mem::replace(self, tmp);
                }
            }
        };
    }

    arith_assign_func!(AddAssign, add_assign, add);
    arith_assign_func!(SubAssign, sub_assign, sub);
    arith_assign_func!(MulAssign, mul_assign, mul);
    arith_assign_func!(DivAssign, div_assign, div);
    arith_assign_func!(RemAssign, rem_assign, rem);
    arith_assign_func!(ShlAssign, shl_assign, shiftl);
    arith_assign_func!(ShrAssign, shr_assign, shiftr);

    macro_rules! bit_assign_func {
        ($op_name:ident, $fn_name:ident, $func: ident) => {
            impl<A, B> $op_name<Array<B>> for Array<A>
            where
                A: HasAfEnum + ImplicitPromote<B>,
                B: HasAfEnum + ImplicitPromote<A>,
                <A as ImplicitPromote<B>>::Output: HasAfEnum,
                <B as ImplicitPromote<A>>::Output: HasAfEnum,
            {
                #[allow(unused_variables)]
                fn $fn_name(&mut self, rhs: Array<B>) {
                    let tmp_seq = Seq::<f32>::default();
                    let mut idxrs = Indexer::new();
                    for n in 0..self.numdims() {
                        idxrs.set_index(&tmp_seq, n, Some(false));
                    }
                    let opres = $func(self as &Array<A>, &rhs, false).cast::<A>();
                    let tmp = assign_gen(self as &Array<A>, &idxrs, &opres);
                    let old = mem::replace(self, tmp);
                }
            }
        };
    }

    bit_assign_func!(BitAndAssign, bitand_assign, bitand);
    bit_assign_func!(BitOrAssign, bitor_assign, bitor);
    bit_assign_func!(BitXorAssign, bitxor_assign, bitxor);

}

///Implement negation trait for Array
impl<T> Neg for Array<T>
where
    T: HasAfEnum + Zero + ConstGenerator,
    <T as ConstGenerator>::OutType: HasAfEnum,
    <T as ConstGenerator>::OutType: ImplicitPromote<T>,
    T: ImplicitPromote<<T as ConstGenerator>::OutType>,
    <<T as ConstGenerator>::OutType as ImplicitPromote<T>>::Output: HasAfEnum,
{
    type Output = Array<<<T as ConstGenerator>::OutType as ImplicitPromote<T>>::Output>;

    fn neg(self) -> Self::Output {
        let cnst = constant(T::zero(), self.dims());
        sub(&cnst, &self, true)
    }
}