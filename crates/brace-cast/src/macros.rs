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
}

#[macro_export]
macro_rules! impl_cast_as_ref {
    (struct $from:path : $as:path) => {
        impl $crate::CastAsRef<dyn $as> for $from
        where
            $from: $as,
        {
            fn cast_as_ref(&self) -> std::option::Option<&(dyn $as + 'static)> {
                std::option::Option::Some(self as &dyn $as)
            }
        }
    };

    (struct $from:path : $as:path $(, $also:path)+) => {
        $crate::impl_cast_as_ref!(struct $from : $as);
        $(
            $crate::impl_cast_as_ref!(struct $from : $also);
        )*
    };
}

#[macro_export]
macro_rules! impl_cast_as_mut {
    (struct $from:path : $as:path) => {
        impl $crate::CastAsMut<dyn $as> for $from
        where
            $from: $as,
        {
            fn cast_as_mut(&mut self) -> std::option::Option<&mut (dyn $as + 'static)> {
                std::option::Option::Some(self as &mut dyn $as)
            }
        }
    };

    (struct $from:path : $as:path $(, $also:path)+) => {
        $crate::impl_cast_as_mut!(struct $from : $as);
        $(
            $crate::impl_cast_as_mut!(struct $from : $also);
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
}

#[macro_export]
macro_rules! impl_cast_from_ref {
    (struct $from:path : $as:path) => {
        impl $crate::CastFromRef<$from> for dyn $as
        where
            $from: $as,
        {
            fn cast_from_ref(from: &$from) -> std::option::Option<&Self> {
                std::option::Option::Some(from as &dyn $as)
            }
        }
    };

    (struct $from:path : $as:path $(, $also:path)+) => {
        $crate::impl_cast_from_ref!(struct $from : $as);
        $(
            $crate::impl_cast_from_ref!(struct $from : $also);
        )*
    };
}

#[macro_export]
macro_rules! impl_cast_from_mut {
    (struct $from:path : $as:path) => {
        impl $crate::CastFromMut<$from> for dyn $as
        where
            $from: $as,
        {
            fn cast_from_mut(from: &mut $from) -> std::option::Option<&mut Self> {
                std::option::Option::Some(from as &mut dyn $as)
            }
        }
    };

    (struct $from:path : $as:path $(, $also:path)+) => {
        $crate::impl_cast_from_mut!(struct $from : $as);
        $(
            $crate::impl_cast_from_mut!(struct $from : $also);
        )*
    };
}
