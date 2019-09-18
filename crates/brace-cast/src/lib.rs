use std::any::Any;
use std::rc::Rc;
use std::sync::Arc;

pub use inventory;

mod macros;

pub mod registry;

pub fn cast_ref<T, U>(item: &U) -> Option<&T>
where
    T: ?Sized,
    U: CastAsRef<T> + ?Sized,
{
    item.cast_as_ref()
}

pub fn cast_mut<T, U>(item: &mut U) -> Option<&mut T>
where
    T: ?Sized,
    U: CastAsMut<T> + ?Sized,
{
    item.cast_as_mut()
}

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

impl<T, U> CastFromRef<Rc<U>> for T
where
    T: ?Sized,
    U: CastAsRef<T> + ?Sized,
{
    fn cast_from_ref(from: &Rc<U>) -> Option<&Self> {
        (**from).cast_as_ref()
    }
}

impl<T, U> CastFromRef<Arc<U>> for T
where
    T: ?Sized,
    U: CastAsRef<T> + ?Sized,
{
    fn cast_from_ref(from: &Arc<U>) -> Option<&Self> {
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
    use crate::{cast_mut, cast_ref, impl_cast_as, impl_cast_from, Cast};

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

    impl Feline for Cat {
        fn eyes(&self) -> &usize {
            &self.eyes
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

    impl Canine for Dog {
        fn ears(&self) -> &usize {
            &self.ears
        }
    }

    impl_cast_from!(struct Dog: Animal, Canine);
    impl_cast_as!(struct Cat: Animal, Feline);
    impl_cast_as!(trait Animal: Feline, Canine);

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

        assert!(cast_ref::<Cat, _>(cat).is_some());
        assert!(cast_ref::<Dog, _>(cat).is_none());
        assert!(cast_mut::<Cat, _>(cat).is_some());
        assert!(cast_mut::<Dog, _>(cat).is_none());

        let mut cat: Box<dyn Feline> = Box::new(Cat::new("Felix"));
        let cat: &mut dyn Feline = &mut *cat;

        assert!(cast_ref::<Cat, _>(cat).is_some());
        assert!(cast_mut::<Cat, _>(cat).is_some());

        let mut dog: Box<dyn Animal> = Box::new(Dog::new("Rover"));
        let dog: &mut dyn Animal = &mut *dog;

        assert!(cast_ref::<Cat, _>(dog).is_none());
        assert!(cast_ref::<Dog, _>(dog).is_some());
        assert!(cast_mut::<Cat, _>(dog).is_none());
        assert!(cast_mut::<Dog, _>(dog).is_some());

        let mut dog: Box<dyn Canine> = Box::new(Dog::new("Rover"));
        let dog: &mut dyn Canine = &mut *dog;

        assert!(cast_ref::<Dog, _>(dog).is_some());
        assert!(cast_mut::<Dog, _>(dog).is_some());
    }

    #[test]
    fn test_cast_trait_object_box_as_trait_object() {
        let mut cat: Box<dyn Animal> = Box::new(Cat::new("Felix"));

        assert!(cat.cast_ref::<dyn Feline>().is_some());
        assert!(cat.cast_ref::<dyn Canine>().is_none());
        assert!(cat.cast_mut::<dyn Feline>().is_some());
        assert!(cat.cast_mut::<dyn Canine>().is_none());

        let mut dog: Box<dyn Animal> = Box::new(Dog::new("Rover"));

        assert!(dog.cast_ref::<dyn Feline>().is_none());
        assert!(dog.cast_ref::<dyn Canine>().is_some());
        assert!(dog.cast_mut::<dyn Feline>().is_none());
        assert!(dog.cast_mut::<dyn Canine>().is_some());
    }

    #[test]
    fn test_cast_trait_object_ref_as_trait_object() {
        let mut cat: Box<dyn Animal> = Box::new(Cat::new("Felix"));
        let cat: &mut dyn Animal = &mut *cat;

        assert!(cast_ref::<dyn Feline, _>(cat).is_some());
        assert!(cast_ref::<dyn Canine, _>(cat).is_none());
        assert!(cast_mut::<dyn Feline, _>(cat).is_some());
        assert!(cast_mut::<dyn Canine, _>(cat).is_none());

        let mut dog: Box<dyn Animal> = Box::new(Dog::new("Rover"));
        let dog: &mut dyn Animal = &mut *dog;

        assert!(cast_ref::<dyn Feline, _>(dog).is_none());
        assert!(cast_ref::<dyn Canine, _>(dog).is_some());
        assert!(cast_mut::<dyn Feline, _>(dog).is_none());
        assert!(cast_mut::<dyn Canine, _>(dog).is_some());
    }
}
