pub mod vec3 {
    use core::f32;
    use std::ops::{Add, Index, Neg, Sub};
    #[derive(Debug, PartialEq, PartialOrd)]
    pub struct Vec3<T>([T; 3])
    where
        T: Add + Copy + Neg;

    impl<T, U> Add<Vec3<T>> for Vec3<T>
    where
        T: Add<Output = U> + Copy + Neg,
        U: Add + Copy + Neg, // necessary to close under addition
    {
        type Output = Vec3<U>;

        fn add(self, _rhs: Self) -> Self::Output {
            let new_x = *self.x() + *_rhs.x();
            let new_y = *self.y() + *_rhs.y();
            let new_z = *self.z() + *_rhs.z();

            Vec3([new_x, new_y, new_z])
        }
    }

    impl<T, U> Neg for Vec3<T>
    where
        T: Add + Copy + Neg<Output = U>,
        U: Add + Copy + Neg,
    {
        type Output = Vec3<U>;

        fn neg(self) -> Self::Output {
            Vec3([-*self.x(), -*self.y(), -*self.z()])
        }
    }

    impl<T, U> Sub for Vec3<T>
    where
        T: Add<Output = U> + Copy + Neg<Output = T>,
        U: Add + Copy + Neg,
    {
        type Output = Vec3<U>;

        fn sub(self, _rhs: Self) -> Self::Output {
            let new_x = *self.x() + -*_rhs.x();
            let new_y = *self.y() + -*_rhs.y();
            let new_z = *self.z() + -*_rhs.z();

            Vec3([new_x, new_y, new_z])
        }
    }

    impl<T> Index<i32> for Vec3<T>
    where
        T: Add + Copy + Neg,
    {
        type Output = T;

        fn index(&self, i: i32) -> &Self::Output {
            match i {
                // Weird modulo arithmetic here is to ensure positive only value
                idx if ((idx % 3) + 3) % 3 == 0 => self.x(),
                idx if ((idx % 3) + 3) % 3 == 1 => self.y(),
                idx if ((idx % 3) + 3) % 3 == 2 => self.z(),
                _ => panic!("Vec3 Index Out of Bounds"),
            }
        }
    }

    impl<T> Vec3<T>
    where
        T: Add + Copy + Neg,
    {
        pub fn new(x: T, y: T, z: T) -> Vec3<T> {
            Vec3([x, y, z])
        }

        pub fn x(&self) -> &T {
            &self.0[0]
        }

        pub fn y(&self) -> &T {
            &self.0[1]
        }

        pub fn z(&self) -> &T {
            &self.0[2]
        }
    }

    pub type Color = Vec3<f32>;
    pub type Point = Vec3<f32>;
}
