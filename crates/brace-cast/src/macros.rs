pub use std::any::Any;
pub use std::option::Option;

#[macro_export]
macro_rules! register_cast_ref {
    (struct $from:path : $as:path) => {
        $crate::inventory::submit! {
            #![crate = $crate]
            $crate::registry::CastRefRecord::new::<$from, dyn $as>(
                |item| {
                    let item: &$from = $crate::macros::Any::downcast_ref(item)?;
                    let item: &dyn $as = item;

                    $crate::macros::Option::Some(item)
                }
            )
        }
    };
}

#[macro_export]
macro_rules! register_cast_mut {
    (struct $from:path : $as:path) => {
        $crate::inventory::submit! {
            #![crate = $crate]
            $crate::registry::CastMutRecord::new::<$from, dyn $as>(
                |item| {
                    let item: &mut $from = $crate::macros::Any::downcast_mut(item)?;
                    let item: &mut dyn $as = item;

                    $crate::macros::Option::Some(item)
                }
            )
        }
    };
}

#[macro_export]
macro_rules! impl_cast_as {
    (struct $from:path : $as:path) => {
        $crate::impl_cast_as_ref!(struct $from : $as);
        $crate::impl_cast_as_mut!(struct $from : $as);
    };

    (struct $from:path : $as:path $(, $also:path)+) => {
        $crate::impl_cast_as_ref!(struct $from : $as $(, $also)*);
        $crate::impl_cast_as_mut!(struct $from : $as $(, $also)*);
    };

    (trait $from:path : $as:path) => {
        $crate::impl_cast_as_ref!(trait $from : $as);
        $crate::impl_cast_as_mut!(trait $from : $as);
    };

    (trait $from:path : $as:path $(, $also:path)+) => {
        $crate::impl_cast_as_ref!(trait $from : $as $(, $also)*);
        $crate::impl_cast_as_mut!(trait $from : $as $(, $also)*);
    };
}

#[macro_export]
macro_rules! impl_cast_as_ref {
    (struct $from:path : $as:path) => {
        $crate::register_cast_ref!(struct $from : $as);

        impl $crate::CastAsRef<dyn $as> for $from
        where
            $from: $as,
        {
            fn cast_as_ref(&self) -> $crate::macros::Option<&(dyn $as + 'static)> {
                $crate::macros::Option::Some(self as &dyn $as)
            }
        }

        impl $crate::CastFromRef<dyn $as> for $from
        where
            $from: $as + $crate::Cast,
        {
            fn cast_from_ref<'a>(from: &'a (dyn $as + 'static)) -> $crate::macros::Option<&'a Self> {
                $crate::macros::Any::downcast_ref($crate::CastAsAny::cast_as_any_ref(from))
            }
        }
    };

    (struct $from:path : $as:path $(, $also:path)+) => {
        $crate::impl_cast_as_ref!(struct $from : $as);
        $(
            $crate::impl_cast_as_ref!(struct $from : $also);
        )*
    };

    (trait $from:path : $as:path) => {
        impl $crate::CastAsRef<dyn $as> for dyn $from {
            fn cast_as_ref(&self) -> $crate::macros::Option<&(dyn $as + 'static)> {
                $crate::registry::cast_from_ref::<dyn $from, dyn $as>(self)
            }
        }

        impl $crate::CastFromRef<dyn $as> for dyn $from {
            fn cast_from_ref<'a>(from: &'a (dyn $as + 'static)) -> $crate::macros::Option<&'a Self> {
                $crate::registry::cast_from_ref::<dyn $as, dyn $from>(from)
            }
        }
    };

    (trait $from:path : $as:path $(, $also:path)+) => {
        $crate::impl_cast_as_ref!(trait $from : $as);
        $(
            $crate::impl_cast_as_ref!(trait $from : $also);
        )*
    };
}

#[macro_export]
macro_rules! impl_cast_as_mut {
    (struct $from:path : $as:path) => {
        $crate::register_cast_mut!(struct $from : $as);

        impl $crate::CastAsMut<dyn $as> for $from
        where
            $from: $as,
        {
            fn cast_as_mut(&mut self) -> $crate::macros::Option<&mut (dyn $as + 'static)> {
                $crate::macros::Option::Some(self as &mut dyn $as)
            }
        }

        impl $crate::CastFromMut<dyn $as> for $from
        where
            $from: $as + $crate::Cast,
        {
            fn cast_from_mut<'a>(from: &'a mut (dyn $as + 'static)) -> $crate::macros::Option<&'a mut Self> {
                $crate::macros::Any::downcast_mut($crate::CastAsAny::cast_as_any_mut(from))
            }
        }
    };

    (struct $from:path : $as:path $(, $also:path)+) => {
        $crate::impl_cast_as_mut!(struct $from : $as);
        $(
            $crate::impl_cast_as_mut!(struct $from : $also);
        )*
    };

    (trait $from:path : $as:path) => {
        impl $crate::CastAsMut<dyn $as> for dyn $from {
            fn cast_as_mut(&mut self) -> $crate::macros::Option<&mut (dyn $as + 'static)> {
                $crate::registry::cast_from_mut::<dyn $from, dyn $as>(self)
            }
        }

        impl $crate::CastFromMut<dyn $as> for dyn $from {
            fn cast_from_mut<'a>(from: &'a mut (dyn $as + 'static)) -> $crate::macros::Option<&'a mut Self> {
                $crate::registry::cast_from_mut::<dyn $as, dyn $from>(from)
            }
        }
    };

    (trait $from:path : $as:path $(, $also:path)+) => {
        $crate::impl_cast_as_mut!(trait $from : $as);
        $(
            $crate::impl_cast_as_mut!(trait $from : $also);
        )*
    };
}

#[macro_export]
macro_rules! impl_cast_from {
    (struct $from:path : $as:path) => {
        $crate::impl_cast_from_ref!(struct $from : $as);
        $crate::impl_cast_from_mut!(struct $from : $as);
    };

    (struct $from:path : $as:path $(, $also:path)+) => {
        $crate::impl_cast_from_ref!(struct $from : $as $(, $also)*);
        $crate::impl_cast_from_mut!(struct $from : $as $(, $also)*);
    };

    (trait $from:path : $as:path) => {
        $crate::impl_cast_from_ref!(trait $from : $as);
        $crate::impl_cast_from_mut!(trait $from : $as);
    };

    (trait $from:path : $as:path $(, $also:path)+) => {
        $crate::impl_cast_from_ref!(trait $from : $as $(, $also)*);
        $crate::impl_cast_from_mut!(trait $from : $as $(, $also)*);
    };
}

#[macro_export]
macro_rules! impl_cast_from_ref {
    (struct $from:path : $as:path) => {
        $crate::register_cast_ref!(struct $from : $as);

        impl $crate::CastFromRef<$from> for dyn $as
        where
            $from: $as,
        {
            fn cast_from_ref(from: &$from) -> $crate::macros::Option<&Self> {
                $crate::macros::Option::Some(from as &dyn $as)
            }
        }

        impl $crate::CastAsRef<$from> for dyn $as
        where
            $from: $as + $crate::Cast,
        {
            fn cast_as_ref(&self) -> $crate::macros::Option<&$from> {
                $crate::macros::Any::downcast_ref($crate::CastAsAny::cast_as_any_ref(self))
            }
        }
    };

    (struct $from:path : $as:path $(, $also:path)+) => {
        $crate::impl_cast_from_ref!(struct $from : $as);
        $(
            $crate::impl_cast_from_ref!(struct $from : $also);
        )*
    };

    (trait $from:path : $as:path) => {
        impl $crate::CastFromRef<dyn $from> for dyn $as {
            fn cast_from_ref<'a>(from: &'a (dyn $from + 'static)) -> $crate::macros::Option<&'a Self> {
                $crate::registry::cast_from_ref::<dyn $from, dyn $as>(from)
            }
        }

        impl $crate::CastAsRef<dyn $from> for dyn $as {
            fn cast_as_ref(&self) -> $crate::macros::Option<&(dyn $from + 'static)> {
                $crate::registry::cast_from_ref::<dyn $as, dyn $from>(self)
            }
        }
    };

    (trait $from:path : $as:path $(, $also:path)+) => {
        $crate::impl_cast_from_ref!(trait $from : $as);
        $(
            $crate::impl_cast_from_ref!(trait $from : $also);
        )*
    };
}

#[macro_export]
macro_rules! impl_cast_from_mut {
    (struct $from:path : $as:path) => {
        $crate::register_cast_mut!(struct $from : $as);

        impl $crate::CastFromMut<$from> for dyn $as
        where
            $from: $as,
        {
            fn cast_from_mut(from: &mut $from) -> $crate::macros::Option<&mut Self> {
                $crate::macros::Option::Some(from as &mut dyn $as)
            }
        }

        impl $crate::CastAsMut<$from> for dyn $as
        where
            $from: $as + $crate::Cast,
        {
            fn cast_as_mut(&mut self) -> $crate::macros::Option<&mut $from> {
                $crate::macros::Any::downcast_mut($crate::CastAsAny::cast_as_any_mut(self))
            }
        }
    };

    (struct $from:path : $as:path $(, $also:path)+) => {
        $crate::impl_cast_from_mut!(struct $from : $as);
        $(
            $crate::impl_cast_from_mut!(struct $from : $also);
        )*
    };

    (trait $from:path : $as:path) => {
        impl $crate::CastFromMut<dyn $from> for dyn $as {
            fn cast_from_mut<'a>(from: &'a mut (dyn $from + 'static)) -> $crate::macros::Option<&'a mut Self> {
                $crate::registry::cast_from_mut::<dyn $from, dyn $as>(from)
            }
        }

        impl $crate::CastAsMut<dyn $from> for dyn $as {
            fn cast_as_mut(&mut self) -> $crate::macros::Option<&mut (dyn $from + 'static)> {
                $crate::registry::cast_from_mut::<dyn $as, dyn $from>(self)
            }
        }
    };

    (trait $from:path : $as:path $(, $also:path)+) => {
        $crate::impl_cast_from_mut!(trait $from : $as);
        $(
            $crate::impl_cast_from_mut!(trait $from : $also);
        )*
    };
}
