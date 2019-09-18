pub trait CastAsRef<T: ?Sized> {
    fn cast_as_ref(&self) -> Option<&T>;
}

pub trait CastAsMut<T: ?Sized> {
    fn cast_as_mut(&mut self) -> Option<&mut T>;
}

#[cfg(test)]
mod tests {
    use crate::{CastAsMut, CastAsRef};

    trait Animal {
        fn name(&self) -> &str;
    }

    struct Cat {
        name: String,
    }

    impl Cat {
        fn new<S>(name: S) -> Self
        where
            S: Into<String>,
        {
            Self { name: name.into() }
        }
    }

    impl Animal for Cat {
        fn name(&self) -> &str {
            &self.name
        }
    }

    impl CastAsRef<dyn Animal> for Cat {
        fn cast_as_ref(&self) -> Option<&(dyn Animal + 'static)> {
            Some(self as &dyn Animal)
        }
    }

    impl CastAsMut<dyn Animal> for Cat {
        fn cast_as_mut(&mut self) -> Option<&mut (dyn Animal + 'static)> {
            Some(self as &mut dyn Animal)
        }
    }

    struct Dog {
        name: String,
    }

    impl Dog {
        fn new<S>(name: S) -> Self
        where
            S: Into<String>,
        {
            Self { name: name.into() }
        }
    }

    impl Animal for Dog {
        fn name(&self) -> &str {
            &self.name
        }
    }

    impl CastAsRef<dyn Animal> for Dog {
        fn cast_as_ref(&self) -> Option<&(dyn Animal + 'static)> {
            Some(self as &dyn Animal)
        }
    }

    impl CastAsMut<dyn Animal> for Dog {
        fn cast_as_mut(&mut self) -> Option<&mut (dyn Animal + 'static)> {
            Some(self as &mut dyn Animal)
        }
    }

    #[test]
    fn test_cast_struct_as_trait_object() {
        let mut cat = Cat::new("Felix");
        let _: &dyn Animal = cat.cast_as_ref().unwrap();
        let _: &mut dyn Animal = cat.cast_as_mut().unwrap();

        let mut dog = Dog::new("Rover");
        let _: &dyn Animal = dog.cast_as_ref().unwrap();
        let _: &mut dyn Animal = dog.cast_as_mut().unwrap();
    }
}
