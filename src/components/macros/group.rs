#[macro_export]
macro_rules! group {
    ( $vis:vis $trait:ident ) => {
        $vis trait $trait: std::any::Any {
            fn as_any(&self) -> &dyn std::any::Any;
        }

        impl dyn $trait {
            $vis fn is<T: $trait>(&self) -> bool {
                self.as_any().is::<T>()
            }
        }
    };
}

/// # declare!
/// Creates an 'empty trait', which can be used with the 'declare!' macro,
/// to essentially say that the struct or enum is a part of the group or
/// specfic trait. This is useful for dyn traits
/// ```rust
/// pub struct If;
/// pub struct Click;
/// pub struct Loop;
/// pub struct While;
/// declare!([If, Click, Loop, While] as pub Unit of pub UnitTypes);
///
/// let mut vec = Vec::<Box<dyn Unit>>::new();
/// vec.push(Box::new(If));
/// vec.push(Box::new(Click));
/// vec.push(Box::new(Loop));
/// vec.push(Box::new(While));
///
/// for (i, unit) in vec.iter().enumerate() {
///     match unit.get_type() {
///         UnitTypes::Click => {
///             println!("{i}: Click");
///         }
///         UnitTypes::If => {
///             println!("{i}: If");
///         }
///         UnitTypes::Loop => {
///             println!("{i}: Loop");
///         }
///         UnitTypes::While => {
///             println!("{i}: While");
///         }
///         UnitTypes::None => {
///             println!("{i}: None");
///         }
///     }
/// }
/// ```
#[macro_export]
macro_rules! declare {
    ( $type:ident as $vis:vis $trait:ident ) => {
        group!($vis $trait);

        impl $trait for $type {
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
        }
    };
    ( [$( $type:ident ),* ] as $trait_vis:vis $trait:ident of $enum_vis:vis $enum:ident ) => {
        group!($trait_vis $trait);

        $enum_vis enum $enum {
            None,
            $(
                $type
            ),*
        }

        $(
            impl $trait for $type {
                fn as_any(&self) -> &dyn std::any::Any {
                    self
                }
            }
        )*
        impl dyn $trait {

            fn get_type(&self) -> $enum {
                $(
                    if self.is::<$type>(){
                        return $enum::$type;
                    }
                )*
                $enum::None
            }
        }
    };
}
