#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! set_hook {
    ($($_:tt)*) => {};
}

#[cfg(debug_assertions)]
#[macro_export]
macro_rules! set_hook {
    ($obj:expr, $ev:ident, $f:expr) => {
        $obj._hooks.$ev = Some(Box::new($f));
    };
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! emit_hook {
    ($($_:tt)*) => {};
}

#[cfg(debug_assertions)]
#[macro_export]
macro_rules! emit_hook {
    (@rtnval [$hook:expr, $ev:ident] [($($t:expr),+)]) => {
        if let Some(f) = &mut $hook.$ev {
            f($($t,)+);
        }
    };
    (@block [$hook:expr, $ev:ident] [] $val:tt) => {
        emit_hook!(@rtnval [$hook, $ev] [$val]);
    };
    (@block [$hook:expr, $ev:ident] [$($store:tt)*] ; $($rest:tt)+) => {
        $($store)*;
        emit_hook!(@block [$hook, $ev] [] $($rest)+);
    };
    (@block [$hook:expr, $ev:ident] [$($store:tt)*] $head:tt $($rest:tt)+) => {
        emit_hook!(@block [$hook, $ev] [$($store)* $head] $($rest)+);
    };
    ($hook:expr, $ev:ident, {$($t:tt)*}) => {
        emit_hook!(@block [$hook, $ev] [] $($t)*);
    };
    ($hook:expr, $ev:ident $(,$args:expr)*) => {
        if let Some(f) = &mut $hook.$ev {
            f($($args,)*);
        }
    };
}

macro_rules! def_hooks {
    ( $($name:ident($($typ:ty),*),)*) => {
        pub struct DebugHooks<T: Float> {
            $(pub $name: Option<Box<dyn FnMut($($typ),*)>>,)*
            _f: PhantomData<T>,
        }

        impl<T: Float> DebugHooks<T> {
            pub fn new() -> Self {
                Self {
                    _f: PhantomData,
                    $($name: None,)*
                }
            }
        }
    };
}
