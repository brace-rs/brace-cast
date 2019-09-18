pub trait CastAsRef<T: ?Sized> {
    fn cast_as_ref(&self) -> Option<&T>;
}

impl<T, U> CastAsRef<U> for T
where
    U: CastFromRef<T> + ?Sized,
{
    fn cast_as_ref(&self) -> Option<&U> {
        CastFromRef::cast_from_ref(self)
    }
}

pub trait CastAsMut<T: ?Sized> {
    fn cast_as_mut(&mut self) -> Option<&mut T>;
}

impl<T, U> CastAsMut<U> for T
where
    U: CastFromMut<T> + ?Sized,
{
    fn cast_as_mut(&mut self) -> Option<&mut U> {
        CastFromMut::cast_from_mut(self)
    }
}

pub trait CastFromRef<T> {
    fn cast_from_ref(from: &T) -> Option<&Self>;
}

pub trait CastFromMut<T> {
    fn cast_from_mut(from: &mut T) -> Option<&mut Self>;
}

#[cfg(test)]
mod tests {
    use crate::{CastAsMut, CastAsRef, CastFromMut, CastFromRef};

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

    impl CastFromRef<Dog> for dyn Animal {
        fn cast_from_ref(from: &Dog) -> Option<&(dyn Animal + 'static)> {
            Some(from as &dyn Animal)
        }
    }

    impl CastFromMut<Dog> for dyn Animal {
        fn cast_from_mut(from: &mut Dog) -> Option<&mut (dyn Animal + 'static)> {
            Some(from as &mut dyn Animal)
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
