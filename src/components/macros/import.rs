#[macro_export]
macro_rules! include {
    ( $module:ident > $item_vis:vis [ $( $item:tt ),* ] ) => {
        $item_vis use $module::{
            $( $item ),*
        };
    };
    ( $( $module:ident > $item_vis:vis [ $( $item:tt ),* ] ),* $(,)? ) => {
        $(
            crate::include!($module > $item_vis [ $( $item ),* ]);
        )*
    };
}

#[macro_export]
macro_rules! import {
    ( $module_vis:vis $module:ident > $item_vis:vis [ $( $item:tt ),* ]  ) => {
        $module_vis mod $module;
        $item_vis use $module::{
            $( $item ),*
        };
    };
    ( $( $module_vis:vis $module:ident > $item_vis:vis [ $( $item:tt ),* ] ),* $(,)? ) => {
        $(
            crate::import!($module_vis $module > $item_vis [ $( $item ),* ]);
        )*
    };
}
