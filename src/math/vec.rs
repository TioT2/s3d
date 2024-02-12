/// ANIM-RS Project
/// `File` util/math/vec.rs
/// `Description` Vector math implementation module
/// `Author` TioT2
/// `Last changed` 17.12.2023

/// N-component vector declaration macro
macro_rules! vecn_declare_struct {
    ($type_name: ident, $($x: ident),* ) => {
        pub struct $type_name<T> {
            $( pub $x : T, )*
        }
    }
} // macro_rules! vecn_declare_struct

macro_rules! vecn_impl_new_copy_clone {
    ($type: ident, $($x: ident),*) => {
        impl<T: Clone> $type<T> {
            pub fn new($($x: T, )*) -> $type<T> {
                $type::<T> {$($x: $x.clone(), )*} }
        }

        impl<T: Clone> Clone for $type<T> {
            fn clone(&self) -> $type<T> {
                $type::<T> {$($x: self.$x.clone(), )*}
            }
        }

        impl<T: Copy> Copy for $type<T> {
        }
    }
}

// Implementation for +-*/ operators macro redefinition
macro_rules! vecn_impl_operator {
    ($op_name: ident, $op_fn_name: ident, $type: ident, $($x: ident),*) => {
        /* Vec2<T> op Vec2<T> */
        impl<T: Copy + Clone + core::ops::$op_name<Output = T>> core::ops::$op_name<$type<T>> for $type<T> {
            type Output = $type<T>;
            fn $op_fn_name(self, rhs: $type<T>) -> $type<T> {
                $type::<T> {
                    $( $x: core::ops::$op_name::<T>::$op_fn_name(self.$x, rhs.$x) ),*
                }
            }
        }

        /* Vec2<T> op T */
        impl<T: Copy + Clone + core::ops::$op_name<Output = T>> core::ops::$op_name<T> for $type<T> {
            type Output = $type<T>;
            fn $op_fn_name(self, rhs: T) -> $type<T> {
                $type::<T> {
                    $( $x: core::ops::$op_name::<T>::$op_fn_name(self.$x, rhs) ),*
                }
            }
        }
    }
} /* macro_rules! vec2_impl_operator */

macro_rules! vecn_impl_assign_operator {
    ($op_name: ident, $op_fn_name: ident, $type: ident, $($x: ident),*) => {
        impl<T: Copy + Clone + core::ops::$op_name<T>> core::ops::$op_name<$type<T>> for $type<T> {
            fn $op_fn_name(&mut self, rhs: $type<T>) {
                $( core::ops::$op_name::<T>::$op_fn_name(&mut self.$x, rhs.$x); )*
            }
        }

        impl<T: Copy + Clone + core::ops::$op_name<T>> core::ops::$op_name<T> for $type<T> {
            fn $op_fn_name(&mut self, rhs: T) {
                $( core::ops::$op_name::<T>::$op_fn_name(&mut self.$x, rhs); )*
            }
        }
    }
} /* macro_rules! vecn_impl_assign_operator */

macro_rules! vecn_impl_neg_operator {
    ($type: ident, $($x: ident),*) => {
        impl<T: Copy + Clone + core::ops::Neg<Output = T>> core::ops::Neg for $type<T> {
            type Output = $type<T>;
            fn neg(self) -> $type<T> {
                $type::<T> { $($x: -self.$x,)* }
            }
        }
    }
}

macro_rules! vecn_operator_on_variadic {
    ($operator: tt, $first: expr) => {
        $first
    };

    ($operator: tt, $first: expr, $($rest: expr),*) => {
        $first $operator vecn_operator_on_variadic!($operator, $($rest),*)
    };
}

macro_rules! vecn_impl_dot_operator {
    ($type: ident, $($x: ident),*) => {
        impl<T: Copy + Clone + core::ops::Mul<T, Output = T> + core::ops::Add<T, Output = T>> $type<T> {
            pub fn dot(&self, rhs: $type<T>) -> T {
                vecn_operator_on_variadic!(+, $(self.$x * rhs.$x),*)
            }
        }
    }
}

macro_rules! vecn_impl_length_normalize {
    ($fxx: ident, $type: ident, $($x: ident),*) => {
        impl $type<$fxx> {
            pub fn length2(&self) -> $fxx {
                vecn_operator_on_variadic!(+, $(self.$x * self.$x),*)
            }

            pub fn length(&self) -> $fxx {
                (vecn_operator_on_variadic!(+, $(self.$x * self.$x),*)).sqrt()
            }

            pub fn normalize(&mut self) -> &mut $type<$fxx> {
                let length = self.length();
                $(self.$x /= length;)*
                return self;
            }

            pub fn normalized(&self) -> $type<$fxx> {
                let length = self.length();
                $type::<$fxx> {
                    $($x: self.$x / length),*
                }
            }
        }
    }
}

macro_rules! vecn_impl {
    ($type: ident, $($x: ident),*) => {
        vecn_declare_struct!($type, $($x),*);
        vecn_impl_new_copy_clone!($type, $($x),*);

        vecn_impl_neg_operator!($type, $($x),*);
        vecn_impl_dot_operator!($type, $($x),*);

        vecn_impl_operator!(Add, add, $type, $($x),*);
        vecn_impl_operator!(Sub, sub, $type, $($x),*);
        vecn_impl_operator!(Mul, mul, $type, $($x),*);
        vecn_impl_operator!(Div, div, $type, $($x),*);

        vecn_impl_assign_operator!(AddAssign, add_assign, $type, $($x),*);
        vecn_impl_assign_operator!(SubAssign, sub_assign, $type, $($x),*);
        vecn_impl_assign_operator!(MulAssign, mul_assign, $type, $($x),*);
        vecn_impl_assign_operator!(DivAssign, div_assign, $type, $($x),*);

        vecn_impl_length_normalize!(f32, $type, $($x),*);
        vecn_impl_length_normalize!(f64, $type, $($x),*);
    }
}

// Default n-component vector declarations
vecn_impl!(Vec2, x, y);
vecn_impl!(Vec3, x, y, z);
vecn_impl!(Vec4, x, y, z, w);

/// 3-component vector cross product implementation
impl<T: Copy + core::ops::Mul<T, Output = T> + core::ops::Sub<T, Output = T>> core::ops::Rem<Vec3<T>> for Vec3<T> {
    type Output = Vec3<T>;

    /// Cross product getting operator
    /// * `rhs` - vector to get cross product with
    /// * Returns vectors cross product
    fn rem(self, rhs: Vec3<T>) -> Self::Output {
        Self::Output {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    } // fn rem
} // impl<T: Copy + core::ops::Mul<T, Output = T> + core::ops::Sub<T, Output = T>> core::ops::Rem<Vec3<T>> for Vec3<T>

// file vec.rs