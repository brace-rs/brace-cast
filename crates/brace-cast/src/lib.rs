pub trait Cast {
    fn cast_ref<T>(&self) -> Option<&T>
    where
        T: ?Sized,
        Self: CastAsRef<T>;

    fn cast_mut<T>(&mut self) -> Option<&mut T>
    where
        T: ?Sized,
        Self: CastAsMut<T>;
}

impl<T> Cast for T {
    fn cast_ref<U>(&self) -> Option<&U>
    where
        U: ?Sized,
        Self: CastAsRef<U>,
    {
        self.cast_as_ref()
    }

    fn cast_mut<U>(&mut self) -> Option<&mut U>
    where
        U: ?Sized,
        Self: CastAsMut<U>,
    {
        self.cast_as_mut()
    }
}

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
    use crate::{Cast, CastAsMut, CastAsRef, CastFromMut, CastFromRef};

    trait Animal {
        fn name(&self) -> &str;
    }

    trait Feline: Animal {
        fn eyes(&self) -> &usize;
    }

    trait Canine: Animal {
        fn ears(&self) -> &usize;
    }

    struct Cat {
        name: String,
        eyes: usize,
    }

    impl Cat {
        fn new<S>(name: S) -> Self
        where
            S: Into<String>,
        {
            Self {
                name: name.into(),
                eyes: 2,
            }
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

    impl Feline for Cat {
        fn eyes(&self) -> &usize {
            &self.eyes
        }
    }

    impl CastAsRef<dyn Feline> for Cat {
        fn cast_as_ref(&self) -> Option<&(dyn Feline + 'static)> {
            Some(self as &dyn Feline)
        }
    }

    impl CastAsMut<dyn Feline> for Cat {
        fn cast_as_mut(&mut self) -> Option<&mut (dyn Feline + 'static)> {
            Some(self as &mut dyn Feline)
        }
    }

    struct Dog {
        name: String,
        ears: usize,
    }

    impl Dog {
        fn new<S>(name: S) -> Self
        where
            S: Into<String>,
        {
            Self {
                name: name.into(),
                ears: 2,
            }
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

    impl Canine for Dog {
        fn ears(&self) -> &usize {
            &self.ears
        }
    }

    impl CastFromRef<Dog> for dyn Canine {
        fn cast_from_ref(from: &Dog) -> Option<&(dyn Canine + 'static)> {
            Some(from as &dyn Canine)
        }
    }

    impl CastFromMut<Dog> for dyn Canine {
        fn cast_from_mut(from: &mut Dog) -> Option<&mut (dyn Canine + 'static)> {
            Some(from as &mut dyn Canine)
        }
    }

    #[test]
    fn test_cast_struct_as_trait_object() {
        let mut cat = Cat::new("Felix");

        assert!(cat.cast_ref::<dyn Animal>().is_some());
        assert!(cat.cast_mut::<dyn Animal>().is_some());
        assert!(cat.cast_ref::<dyn Feline>().is_some());
        assert!(cat.cast_mut::<dyn Feline>().is_some());

        let mut dog = Dog::new("Rover");

        assert!(dog.cast_ref::<dyn Animal>().is_some());
        assert!(dog.cast_mut::<dyn Animal>().is_some());
        assert!(dog.cast_ref::<dyn Canine>().is_some());
        assert!(dog.cast_mut::<dyn Canine>().is_some());
    }
}
