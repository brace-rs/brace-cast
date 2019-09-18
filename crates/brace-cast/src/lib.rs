use std::any::Any;

pub trait Cast: CastAsAny {
    fn cast_ref<T>(&self) -> Option<&T>
    where
        T: ?Sized,
        Self: CastAsRef<T> + Sized;

    fn cast_mut<T>(&mut self) -> Option<&mut T>
    where
        T: ?Sized,
        Self: CastAsMut<T> + Sized;
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

pub trait CastAsAny {
    fn cast_as_any_ref(&self) -> &dyn Any
    where
        Self: 'static;

    fn cast_as_any_mut(&mut self) -> &mut dyn Any
    where
        Self: 'static;
}

impl<T> CastAsAny for T {
    fn cast_as_any_ref(&self) -> &dyn Any
    where
        T: 'static,
    {
        self
    }

    fn cast_as_any_mut(&mut self) -> &mut dyn Any
    where
        T: 'static,
    {
        self
    }
}

pub trait CastAsRef<T: ?Sized> {
    fn cast_as_ref(&self) -> Option<&T>;
}

impl<T, U> CastAsRef<U> for T
where
    T: ?Sized,
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
    T: ?Sized,
    U: CastFromMut<T> + ?Sized,
{
    fn cast_as_mut(&mut self) -> Option<&mut U> {
        CastFromMut::cast_from_mut(self)
    }
}

pub trait CastFromRef<T: ?Sized> {
    fn cast_from_ref(from: &T) -> Option<&Self>;
}

impl<T, U> CastFromRef<Box<U>> for T
where
    T: ?Sized,
    U: CastAsRef<T> + ?Sized,
{
    fn cast_from_ref(from: &Box<U>) -> Option<&Self> {
        (**from).cast_as_ref()
    }
}

pub trait CastFromMut<T: ?Sized> {
    fn cast_from_mut(from: &mut T) -> Option<&mut Self>;
}

impl<T, U> CastFromMut<Box<U>> for T
where
    T: ?Sized,
    U: CastAsMut<T> + ?Sized,
{
    fn cast_from_mut(from: &mut Box<U>) -> Option<&mut Self> {
        (**from).cast_as_mut()
    }
}

#[cfg(test)]
mod tests {
    use crate::{Cast, CastAsMut, CastAsRef, CastFromMut, CastFromRef};

    trait Animal: Cast {
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

    impl CastAsRef<Cat> for dyn Animal {
        fn cast_as_ref(&self) -> Option<&Cat> {
            self.cast_as_any_ref().downcast_ref()
        }
    }

    impl CastAsMut<Cat> for dyn Animal {
        fn cast_as_mut(&mut self) -> Option<&mut Cat> {
            self.cast_as_any_mut().downcast_mut()
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

    impl CastAsRef<Cat> for dyn Feline {
        fn cast_as_ref(&self) -> Option<&Cat> {
            self.cast_as_any_ref().downcast_ref()
        }
    }

    impl CastAsMut<Cat> for dyn Feline {
        fn cast_as_mut(&mut self) -> Option<&mut Cat> {
            self.cast_as_any_mut().downcast_mut()
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

    impl CastFromRef<dyn Animal> for Dog {
        fn cast_from_ref<'a>(from: &'a (dyn Animal + 'static)) -> Option<&'a Self> {
            from.cast_as_any_ref().downcast_ref()
        }
    }

    impl CastFromMut<dyn Animal> for Dog {
        fn cast_from_mut<'a>(from: &'a mut (dyn Animal + 'static)) -> Option<&'a mut Self> {
            from.cast_as_any_mut().downcast_mut()
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

    impl CastFromRef<dyn Canine> for Dog {
        fn cast_from_ref<'a>(from: &'a (dyn Canine + 'static)) -> Option<&'a Self> {
            from.cast_as_any_ref().downcast_ref()
        }
    }

    impl CastFromMut<dyn Canine> for Dog {
        fn cast_from_mut<'a>(from: &'a mut (dyn Canine + 'static)) -> Option<&'a mut Self> {
            from.cast_as_any_mut().downcast_mut()
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

    #[test]
    fn test_cast_trait_object_box_as_struct() {
        let mut cat: Box<dyn Animal> = Box::new(Cat::new("Felix"));

        assert!(cat.cast_ref::<Cat>().is_some());
        assert!(cat.cast_ref::<Dog>().is_none());
        assert!(cat.cast_mut::<Cat>().is_some());
        assert!(cat.cast_mut::<Dog>().is_none());

        let mut cat: Box<dyn Feline> = Box::new(Cat::new("Felix"));

        assert!(cat.cast_ref::<Cat>().is_some());
        assert!(cat.cast_mut::<Cat>().is_some());

        let mut dog: Box<dyn Animal> = Box::new(Dog::new("Rover"));

        assert!(dog.cast_ref::<Cat>().is_none());
        assert!(dog.cast_ref::<Dog>().is_some());
        assert!(dog.cast_mut::<Cat>().is_none());
        assert!(dog.cast_mut::<Dog>().is_some());

        let mut dog: Box<dyn Canine> = Box::new(Dog::new("Rover"));

        assert!(dog.cast_ref::<Dog>().is_some());
        assert!(dog.cast_mut::<Dog>().is_some());
    }

    #[test]
    fn test_cast_trait_object_ref_as_struct() {
        let mut cat: Box<dyn Animal> = Box::new(Cat::new("Felix"));
        let cat: &mut dyn Animal = &mut *cat;

        assert!(CastAsRef::<Cat>::cast_as_ref(cat).is_some());
        assert!(CastAsRef::<Dog>::cast_as_ref(cat).is_none());
        assert!(CastAsMut::<Cat>::cast_as_mut(cat).is_some());
        assert!(CastAsMut::<Dog>::cast_as_mut(cat).is_none());

        let mut cat: Box<dyn Feline> = Box::new(Cat::new("Felix"));
        let cat: &mut dyn Feline = &mut *cat;

        assert!(CastAsRef::<Cat>::cast_as_ref(cat).is_some());
        assert!(CastAsMut::<Cat>::cast_as_mut(cat).is_some());

        let mut dog: Box<dyn Animal> = Box::new(Dog::new("Rover"));
        let dog: &mut dyn Animal = &mut *dog;

        assert!(CastAsRef::<Cat>::cast_as_ref(dog).is_none());
        assert!(CastAsRef::<Dog>::cast_as_ref(dog).is_some());
        assert!(CastAsMut::<Cat>::cast_as_mut(dog).is_none());
        assert!(CastAsMut::<Dog>::cast_as_mut(dog).is_some());

        let mut dog: Box<dyn Canine> = Box::new(Dog::new("Rover"));
        let dog: &mut dyn Canine = &mut *dog;

        assert!(CastAsRef::<Dog>::cast_as_ref(dog).is_some());
        assert!(CastAsMut::<Dog>::cast_as_mut(dog).is_some());
    }
}
