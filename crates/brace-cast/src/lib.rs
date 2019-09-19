extern crate self as brace_cast;

use std::any::Any;
use std::rc::Rc;
use std::sync::Arc;

pub use brace_cast_macros::cast;
pub use inventory;

pub mod macros;
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
    use crate::{cast, cast_mut, cast_ref, impl_cast_as, impl_cast_from, Cast};

    trait Animal: Cast {
        fn name(&self) -> &str;
    }

    trait Mammal: Animal {
        fn legs(&self) -> &usize;
    }

    trait Feline: Mammal {
        fn eyes(&self) -> &usize;
    }

    trait Canine: Mammal {
        fn ears(&self) -> &usize;
    }

    #[cast]
    trait Rodent: Animal + Mammal {
        fn tail(&self) -> &bool;
    }

    struct Cat {
        name: String,
        legs: usize,
        eyes: usize,
    }

    impl Cat {
        fn new<S>(name: S) -> Self
        where
            S: Into<String>,
        {
            Self {
                name: name.into(),
                legs: 4,
                eyes: 2,
            }
        }
    }

    impl Animal for Cat {
        fn name(&self) -> &str {
            &self.name
        }
    }

    impl Mammal for Cat {
        fn legs(&self) -> &usize {
            &self.legs
        }
    }

    impl Feline for Cat {
        fn eyes(&self) -> &usize {
            &self.eyes
        }
    }

    impl_cast_as!(struct Cat: Animal, Mammal, Feline);
    impl_cast_as!(trait Animal: Mammal, Feline);
    impl_cast_as!(trait Mammal: Feline);

    struct Dog {
        name: String,
        legs: usize,
        ears: usize,
    }

    impl Dog {
        fn new<S>(name: S) -> Self
        where
            S: Into<String>,
        {
            Self {
                name: name.into(),
                legs: 4,
                ears: 2,
            }
        }
    }

    impl Animal for Dog {
        fn name(&self) -> &str {
            &self.name
        }
    }

    impl Mammal for Dog {
        fn legs(&self) -> &usize {
            &self.legs
        }
    }

    impl Canine for Dog {
        fn ears(&self) -> &usize {
            &self.ears
        }
    }

    impl_cast_from!(struct Dog: Animal, Mammal, Canine);
    impl_cast_from!(trait Animal: Canine);
    impl_cast_from!(trait Mammal: Canine);

    struct Rat {
        name: String,
        legs: usize,
        tail: bool,
    }

    impl Rat {
        fn new<S>(name: S) -> Self
        where
            S: Into<String>,
        {
            Self {
                name: name.into(),
                legs: 4,
                tail: true,
            }
        }
    }

    #[cast]
    impl Animal for Rat {
        fn name(&self) -> &str {
            &self.name
        }
    }

    #[cast]
    impl Mammal for Rat {
        fn legs(&self) -> &usize {
            &self.legs
        }
    }

    #[cast]
    impl Rodent for Rat {
        fn tail(&self) -> &bool {
            &self.tail
        }
    }

    #[test]
    fn test_cast_struct_as_trait_object() {
        let mut cat = Cat::new("Felix");

        assert!(cat.cast_ref::<dyn Animal>().is_some());
        assert!(cat.cast_ref::<dyn Mammal>().is_some());
        assert!(cat.cast_ref::<dyn Feline>().is_some());
        assert!(cat.cast_mut::<dyn Animal>().is_some());
        assert!(cat.cast_mut::<dyn Mammal>().is_some());
        assert!(cat.cast_mut::<dyn Feline>().is_some());

        let mut dog = Dog::new("Rover");

        assert!(dog.cast_ref::<dyn Animal>().is_some());
        assert!(dog.cast_ref::<dyn Mammal>().is_some());
        assert!(dog.cast_ref::<dyn Canine>().is_some());
        assert!(dog.cast_mut::<dyn Animal>().is_some());
        assert!(dog.cast_mut::<dyn Mammal>().is_some());
        assert!(dog.cast_mut::<dyn Canine>().is_some());

        let mut rat = Rat::new("Daisy");

        assert!(rat.cast_ref::<dyn Animal>().is_some());
        assert!(rat.cast_ref::<dyn Mammal>().is_some());
        assert!(rat.cast_ref::<dyn Rodent>().is_some());
        assert!(rat.cast_mut::<dyn Animal>().is_some());
        assert!(rat.cast_mut::<dyn Mammal>().is_some());
        assert!(rat.cast_mut::<dyn Rodent>().is_some());
    }

    #[test]
    fn test_cast_trait_object_box_as_struct() {
        let mut cat: Box<dyn Animal> = Box::new(Cat::new("Felix"));

        assert!(cat.cast_ref::<Cat>().is_some());
        assert!(cat.cast_ref::<Dog>().is_none());
        assert!(cat.cast_ref::<Rat>().is_none());
        assert!(cat.cast_mut::<Cat>().is_some());
        assert!(cat.cast_mut::<Dog>().is_none());
        assert!(cat.cast_mut::<Rat>().is_none());

        let mut cat: Box<dyn Mammal> = Box::new(Cat::new("Felix"));

        assert!(cat.cast_ref::<Cat>().is_some());
        assert!(cat.cast_ref::<Dog>().is_none());
        assert!(cat.cast_ref::<Rat>().is_none());
        assert!(cat.cast_mut::<Cat>().is_some());
        assert!(cat.cast_mut::<Dog>().is_none());
        assert!(cat.cast_mut::<Rat>().is_none());

        let mut cat: Box<dyn Feline> = Box::new(Cat::new("Felix"));

        assert!(cat.cast_ref::<Cat>().is_some());
        assert!(cat.cast_mut::<Cat>().is_some());

        let mut dog: Box<dyn Animal> = Box::new(Dog::new("Rover"));

        assert!(dog.cast_ref::<Cat>().is_none());
        assert!(dog.cast_ref::<Dog>().is_some());
        assert!(dog.cast_ref::<Rat>().is_none());
        assert!(dog.cast_mut::<Cat>().is_none());
        assert!(dog.cast_mut::<Dog>().is_some());
        assert!(dog.cast_mut::<Rat>().is_none());

        let mut dog: Box<dyn Mammal> = Box::new(Dog::new("Rover"));

        assert!(dog.cast_ref::<Cat>().is_none());
        assert!(dog.cast_ref::<Dog>().is_some());
        assert!(dog.cast_ref::<Rat>().is_none());
        assert!(dog.cast_mut::<Cat>().is_none());
        assert!(dog.cast_mut::<Dog>().is_some());
        assert!(dog.cast_mut::<Rat>().is_none());

        let mut dog: Box<dyn Canine> = Box::new(Dog::new("Rover"));

        assert!(dog.cast_ref::<Dog>().is_some());
        assert!(dog.cast_mut::<Dog>().is_some());

        let mut rat: Box<dyn Animal> = Box::new(Rat::new("Daisy"));

        assert!(rat.cast_ref::<Cat>().is_none());
        assert!(rat.cast_ref::<Dog>().is_none());
        assert!(rat.cast_ref::<Rat>().is_some());
        assert!(rat.cast_mut::<Cat>().is_none());
        assert!(rat.cast_mut::<Dog>().is_none());
        assert!(rat.cast_mut::<Rat>().is_some());

        let mut rat: Box<dyn Mammal> = Box::new(Rat::new("Daisy"));

        assert!(rat.cast_ref::<Cat>().is_none());
        assert!(rat.cast_ref::<Dog>().is_none());
        assert!(rat.cast_ref::<Rat>().is_some());
        assert!(rat.cast_mut::<Cat>().is_none());
        assert!(rat.cast_mut::<Dog>().is_none());
        assert!(rat.cast_mut::<Rat>().is_some());

        let mut rat: Box<dyn Rodent> = Box::new(Rat::new("Daisy"));

        assert!(rat.cast_ref::<Rat>().is_some());
        assert!(rat.cast_mut::<Rat>().is_some());
    }

    #[test]
    fn test_cast_trait_object_ref_as_struct() {
        let mut cat: Box<dyn Animal> = Box::new(Cat::new("Felix"));
        let cat: &mut dyn Animal = &mut *cat;

        assert!(cast_ref::<Cat, _>(cat).is_some());
        assert!(cast_ref::<Dog, _>(cat).is_none());
        assert!(cast_ref::<Rat, _>(cat).is_none());
        assert!(cast_mut::<Cat, _>(cat).is_some());
        assert!(cast_mut::<Dog, _>(cat).is_none());
        assert!(cast_mut::<Rat, _>(cat).is_none());

        let mut cat: Box<dyn Mammal> = Box::new(Cat::new("Felix"));
        let cat: &mut dyn Mammal = &mut *cat;

        assert!(cast_ref::<Cat, _>(cat).is_some());
        assert!(cast_ref::<Dog, _>(cat).is_none());
        assert!(cast_ref::<Rat, _>(cat).is_none());
        assert!(cast_mut::<Cat, _>(cat).is_some());
        assert!(cast_mut::<Dog, _>(cat).is_none());
        assert!(cast_mut::<Rat, _>(cat).is_none());

        let mut cat: Box<dyn Feline> = Box::new(Cat::new("Felix"));
        let cat: &mut dyn Feline = &mut *cat;

        assert!(cast_ref::<Cat, _>(cat).is_some());
        assert!(cast_mut::<Cat, _>(cat).is_some());

        let mut dog: Box<dyn Animal> = Box::new(Dog::new("Rover"));
        let dog: &mut dyn Animal = &mut *dog;

        assert!(cast_ref::<Cat, _>(dog).is_none());
        assert!(cast_ref::<Dog, _>(dog).is_some());
        assert!(cast_ref::<Rat, _>(dog).is_none());
        assert!(cast_mut::<Cat, _>(dog).is_none());
        assert!(cast_mut::<Dog, _>(dog).is_some());
        assert!(cast_mut::<Rat, _>(dog).is_none());

        let mut dog: Box<dyn Mammal> = Box::new(Dog::new("Rover"));
        let dog: &mut dyn Mammal = &mut *dog;

        assert!(cast_ref::<Cat, _>(dog).is_none());
        assert!(cast_ref::<Dog, _>(dog).is_some());
        assert!(cast_ref::<Rat, _>(dog).is_none());
        assert!(cast_mut::<Cat, _>(dog).is_none());
        assert!(cast_mut::<Dog, _>(dog).is_some());
        assert!(cast_mut::<Rat, _>(dog).is_none());

        let mut dog: Box<dyn Canine> = Box::new(Dog::new("Rover"));
        let dog: &mut dyn Canine = &mut *dog;

        assert!(cast_ref::<Dog, _>(dog).is_some());
        assert!(cast_mut::<Dog, _>(dog).is_some());

        let mut rat: Box<dyn Animal> = Box::new(Rat::new("Daisy"));
        let rat: &mut dyn Animal = &mut *rat;

        assert!(cast_ref::<Cat, _>(rat).is_none());
        assert!(cast_ref::<Dog, _>(rat).is_none());
        assert!(cast_ref::<Rat, _>(rat).is_some());
        assert!(cast_mut::<Cat, _>(rat).is_none());
        assert!(cast_mut::<Dog, _>(rat).is_none());
        assert!(cast_mut::<Rat, _>(rat).is_some());

        let mut rat: Box<dyn Mammal> = Box::new(Rat::new("Daisy"));
        let rat: &mut dyn Mammal = &mut *rat;

        assert!(cast_ref::<Cat, _>(rat).is_none());
        assert!(cast_ref::<Dog, _>(rat).is_none());
        assert!(cast_ref::<Rat, _>(rat).is_some());
        assert!(cast_mut::<Cat, _>(rat).is_none());
        assert!(cast_mut::<Dog, _>(rat).is_none());
        assert!(cast_mut::<Rat, _>(rat).is_some());

        let mut rat: Box<dyn Rodent> = Box::new(Rat::new("Daisy"));
        let rat: &mut dyn Rodent = &mut *rat;

        assert!(cast_ref::<Rat, _>(rat).is_some());
        assert!(cast_mut::<Rat, _>(rat).is_some());
    }

    #[test]
    fn test_cast_trait_object_box_as_trait_object() {
        let mut cat: Box<dyn Animal> = Box::new(Cat::new("Felix"));

        assert!(cat.cast_ref::<dyn Mammal>().is_some());
        assert!(cat.cast_ref::<dyn Feline>().is_some());
        assert!(cat.cast_ref::<dyn Canine>().is_none());
        assert!(cat.cast_ref::<dyn Rodent>().is_none());
        assert!(cat.cast_mut::<dyn Mammal>().is_some());
        assert!(cat.cast_mut::<dyn Feline>().is_some());
        assert!(cat.cast_mut::<dyn Canine>().is_none());
        assert!(cat.cast_mut::<dyn Rodent>().is_none());

        let mut cat: Box<dyn Mammal> = Box::new(Cat::new("Felix"));

        assert!(cat.cast_ref::<dyn Animal>().is_some());
        assert!(cat.cast_ref::<dyn Feline>().is_some());
        assert!(cat.cast_ref::<dyn Canine>().is_none());
        assert!(cat.cast_ref::<dyn Rodent>().is_none());
        assert!(cat.cast_mut::<dyn Animal>().is_some());
        assert!(cat.cast_mut::<dyn Feline>().is_some());
        assert!(cat.cast_mut::<dyn Canine>().is_none());
        assert!(cat.cast_mut::<dyn Rodent>().is_none());

        let mut cat: Box<dyn Feline> = Box::new(Cat::new("Felix"));

        assert!(cat.cast_ref::<dyn Animal>().is_some());
        assert!(cat.cast_ref::<dyn Mammal>().is_some());
        assert!(cat.cast_mut::<dyn Animal>().is_some());
        assert!(cat.cast_mut::<dyn Mammal>().is_some());

        let mut dog: Box<dyn Animal> = Box::new(Dog::new("Rover"));

        assert!(dog.cast_ref::<dyn Mammal>().is_some());
        assert!(dog.cast_ref::<dyn Feline>().is_none());
        assert!(dog.cast_ref::<dyn Canine>().is_some());
        assert!(dog.cast_ref::<dyn Rodent>().is_none());
        assert!(dog.cast_mut::<dyn Mammal>().is_some());
        assert!(dog.cast_mut::<dyn Feline>().is_none());
        assert!(dog.cast_mut::<dyn Canine>().is_some());
        assert!(dog.cast_mut::<dyn Rodent>().is_none());

        let mut dog: Box<dyn Mammal> = Box::new(Dog::new("Rover"));

        assert!(dog.cast_ref::<dyn Animal>().is_some());
        assert!(dog.cast_ref::<dyn Feline>().is_none());
        assert!(dog.cast_ref::<dyn Canine>().is_some());
        assert!(dog.cast_ref::<dyn Rodent>().is_none());
        assert!(dog.cast_mut::<dyn Animal>().is_some());
        assert!(dog.cast_mut::<dyn Feline>().is_none());
        assert!(dog.cast_mut::<dyn Canine>().is_some());
        assert!(dog.cast_mut::<dyn Rodent>().is_none());

        let mut dog: Box<dyn Canine> = Box::new(Dog::new("Rover"));

        assert!(dog.cast_ref::<dyn Animal>().is_some());
        assert!(dog.cast_ref::<dyn Mammal>().is_some());
        assert!(dog.cast_mut::<dyn Animal>().is_some());
        assert!(dog.cast_mut::<dyn Mammal>().is_some());

        let mut rat: Box<dyn Animal> = Box::new(Rat::new("Daisy"));

        assert!(rat.cast_ref::<dyn Mammal>().is_some());
        assert!(rat.cast_ref::<dyn Feline>().is_none());
        assert!(rat.cast_ref::<dyn Canine>().is_none());
        assert!(rat.cast_ref::<dyn Rodent>().is_some());
        assert!(rat.cast_mut::<dyn Mammal>().is_some());
        assert!(rat.cast_mut::<dyn Feline>().is_none());
        assert!(rat.cast_mut::<dyn Canine>().is_none());
        assert!(rat.cast_mut::<dyn Rodent>().is_some());

        let mut rat: Box<dyn Mammal> = Box::new(Rat::new("Daisy"));

        assert!(rat.cast_ref::<dyn Animal>().is_some());
        assert!(rat.cast_ref::<dyn Feline>().is_none());
        assert!(rat.cast_ref::<dyn Canine>().is_none());
        assert!(rat.cast_ref::<dyn Rodent>().is_some());
        assert!(rat.cast_mut::<dyn Animal>().is_some());
        assert!(rat.cast_mut::<dyn Feline>().is_none());
        assert!(rat.cast_mut::<dyn Canine>().is_none());
        assert!(rat.cast_mut::<dyn Rodent>().is_some());

        let mut rat: Box<dyn Rodent> = Box::new(Rat::new("Daisy"));

        assert!(rat.cast_ref::<dyn Animal>().is_some());
        assert!(rat.cast_ref::<dyn Mammal>().is_some());
        assert!(rat.cast_mut::<dyn Animal>().is_some());
        assert!(rat.cast_mut::<dyn Mammal>().is_some());
    }

    #[test]
    fn test_cast_trait_object_ref_as_trait_object() {
        let mut cat: Box<dyn Animal> = Box::new(Cat::new("Felix"));
        let cat: &mut dyn Animal = &mut *cat;

        assert!(cast_ref::<dyn Mammal, _>(cat).is_some());
        assert!(cast_ref::<dyn Feline, _>(cat).is_some());
        assert!(cast_ref::<dyn Canine, _>(cat).is_none());
        assert!(cast_ref::<dyn Rodent, _>(cat).is_none());
        assert!(cast_mut::<dyn Mammal, _>(cat).is_some());
        assert!(cast_mut::<dyn Feline, _>(cat).is_some());
        assert!(cast_mut::<dyn Canine, _>(cat).is_none());
        assert!(cast_mut::<dyn Rodent, _>(cat).is_none());

        let mut cat: Box<dyn Mammal> = Box::new(Cat::new("Felix"));
        let cat: &mut dyn Mammal = &mut *cat;

        assert!(cast_ref::<dyn Animal, _>(cat).is_some());
        assert!(cast_ref::<dyn Feline, _>(cat).is_some());
        assert!(cast_ref::<dyn Canine, _>(cat).is_none());
        assert!(cast_ref::<dyn Rodent, _>(cat).is_none());
        assert!(cast_mut::<dyn Animal, _>(cat).is_some());
        assert!(cast_mut::<dyn Feline, _>(cat).is_some());
        assert!(cast_mut::<dyn Canine, _>(cat).is_none());
        assert!(cast_mut::<dyn Rodent, _>(cat).is_none());

        let mut cat: Box<dyn Feline> = Box::new(Cat::new("Felix"));
        let cat: &mut dyn Feline = &mut *cat;

        assert!(cast_ref::<dyn Animal, _>(cat).is_some());
        assert!(cast_ref::<dyn Mammal, _>(cat).is_some());
        assert!(cast_mut::<dyn Animal, _>(cat).is_some());
        assert!(cast_mut::<dyn Mammal, _>(cat).is_some());

        let mut dog: Box<dyn Animal> = Box::new(Dog::new("Rover"));
        let dog: &mut dyn Animal = &mut *dog;

        assert!(cast_ref::<dyn Mammal, _>(dog).is_some());
        assert!(cast_ref::<dyn Feline, _>(dog).is_none());
        assert!(cast_ref::<dyn Canine, _>(dog).is_some());
        assert!(cast_ref::<dyn Rodent, _>(dog).is_none());
        assert!(cast_mut::<dyn Mammal, _>(dog).is_some());
        assert!(cast_mut::<dyn Feline, _>(dog).is_none());
        assert!(cast_mut::<dyn Canine, _>(dog).is_some());
        assert!(cast_mut::<dyn Rodent, _>(dog).is_none());

        let mut dog: Box<dyn Mammal> = Box::new(Dog::new("Rover"));
        let dog: &mut dyn Mammal = &mut *dog;

        assert!(cast_ref::<dyn Animal, _>(dog).is_some());
        assert!(cast_ref::<dyn Feline, _>(dog).is_none());
        assert!(cast_ref::<dyn Canine, _>(dog).is_some());
        assert!(cast_ref::<dyn Rodent, _>(dog).is_none());
        assert!(cast_mut::<dyn Animal, _>(dog).is_some());
        assert!(cast_mut::<dyn Feline, _>(dog).is_none());
        assert!(cast_mut::<dyn Canine, _>(dog).is_some());
        assert!(cast_mut::<dyn Rodent, _>(dog).is_none());

        let mut dog: Box<dyn Canine> = Box::new(Dog::new("Rover"));
        let dog: &mut dyn Canine = &mut *dog;

        assert!(cast_ref::<dyn Animal, _>(dog).is_some());
        assert!(cast_ref::<dyn Mammal, _>(dog).is_some());
        assert!(cast_mut::<dyn Animal, _>(dog).is_some());
        assert!(cast_mut::<dyn Mammal, _>(dog).is_some());

        let mut rat: Box<dyn Animal> = Box::new(Rat::new("Daisy"));
        let rat: &mut dyn Animal = &mut *rat;

        assert!(cast_ref::<dyn Mammal, _>(rat).is_some());
        assert!(cast_ref::<dyn Feline, _>(rat).is_none());
        assert!(cast_ref::<dyn Canine, _>(rat).is_none());
        assert!(cast_ref::<dyn Rodent, _>(rat).is_some());
        assert!(cast_mut::<dyn Mammal, _>(rat).is_some());
        assert!(cast_mut::<dyn Feline, _>(rat).is_none());
        assert!(cast_mut::<dyn Canine, _>(rat).is_none());
        assert!(cast_mut::<dyn Rodent, _>(rat).is_some());

        let mut rat: Box<dyn Mammal> = Box::new(Rat::new("Daisy"));
        let rat: &mut dyn Mammal = &mut *rat;

        assert!(cast_ref::<dyn Animal, _>(rat).is_some());
        assert!(cast_ref::<dyn Feline, _>(rat).is_none());
        assert!(cast_ref::<dyn Canine, _>(rat).is_none());
        assert!(cast_ref::<dyn Rodent, _>(rat).is_some());
        assert!(cast_mut::<dyn Animal, _>(rat).is_some());
        assert!(cast_mut::<dyn Feline, _>(rat).is_none());
        assert!(cast_mut::<dyn Canine, _>(rat).is_none());
        assert!(cast_mut::<dyn Rodent, _>(rat).is_some());

        let mut rat: Box<dyn Rodent> = Box::new(Rat::new("Daisy"));
        let rat: &mut dyn Rodent = &mut *rat;

        assert!(cast_ref::<dyn Animal, _>(rat).is_some());
        assert!(cast_ref::<dyn Mammal, _>(rat).is_some());
        assert!(cast_mut::<dyn Animal, _>(rat).is_some());
        assert!(cast_mut::<dyn Mammal, _>(rat).is_some());
    }
}
