use std::cell::RefCell;

pub type ID = usize;

/// Unique ID Trait
/// Usage:
///   * Uniquely determine a rendering entity
///   * Debug string
///   * possibly used for entity quality checking

pub trait HasUniqueId {
    fn to_id(&self) -> ID;
}

impl<T: HasUniqueId> HasUniqueId for RefCell<T> {
    fn to_id(&self) -> ID {
        self.borrow().to_id()
    }
}

#[macro_export]
macro_rules! impl_id {
    ($static_name:ident, $struct_name:ident <$($g:ident$(,)*)+>) => {
        use core::sync::atomic::{AtomicUsize, Ordering};

        // Init Atomic Usize
        static $static_name: AtomicUsize = AtomicUsize::new(1);

        // Increment ID
        impl<$($g,)+> $struct_name <$($g,)+> {
            fn _next_id() -> usize {
                $static_name.fetch_add(1, Ordering::SeqCst)
            }
        }

        impl<$($g,)+> HasUniqueId for $struct_name <$($g,)+> {
            fn to_id(&self) -> usize {
                self.id
            }
        }

        impl<$($g,)+> HasUniqueId for Rc<$struct_name <$($g,)+>> {
            fn to_id(&self) -> usize {
                <$struct_name <$($g,)+>>::to_id(self)
            }
        }

        impl<$($g,)+> HasUniqueId for Rc<RefCell<$struct_name <$($g,)+>>> {
            fn to_id(&self) -> usize {
                self.borrow().to_id()
            }
        }

        impl<$($g,)+> AsEntityHashKey for Rc<$struct_name <$($g,)+>> {}
        impl<$($g,)+> AsEntityHashKey for Rc<RefCell<$struct_name <$($g,)+>>> {}
    };
}
