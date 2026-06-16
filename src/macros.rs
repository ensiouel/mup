/// Builds escaped HTML markup from the `mup` DSL.
///
/// Static text and ordinary Rust values are escaped by default. Use
/// [`Markup::raw`](crate::Markup::raw) only for trusted HTML.
#[macro_export]
macro_rules! markup {
    ($($tokens:tt)*) => {{
        let mut __markup_template = $crate::template::TemplateBuilder::new();
        $crate::__markup_nodes!(__markup_template; []; $($tokens)*);
        __markup_template.finish()
    }};
}

/// Declares lightweight renderable components.
///
/// The macro generates the requested struct or enum plus a [`Render`](crate::Render)
/// implementation whose body is written with [`markup!`](crate::markup).
#[macro_export]
macro_rules! component {
    () => {};

    (
        $(#[$meta:meta])*
        $vis:vis struct $name:ident < $($tail:tt)*
    ) => {
        $crate::__markup_component_collect_generics! {
            struct
            [[$(#[$meta])*] [$vis] [$name]]
            []
            [@]
            $($tail)*
        }
    };

    (
        $(#[$meta:meta])*
        $vis:vis struct $name:ident {
            $($field_vis:vis $field:ident : $ty:ty),* $(,)?
        } {
            $($body:tt)*
        }

        $($rest:tt)*
    ) => {
        $crate::__markup_component_item! {
            $(#[$meta])*
            $vis struct $name {
                $($field_vis $field: $ty),*
            }

            $($body)*
        }

        $crate::component! { $($rest)* }
    };

    (
        $(#[$meta:meta])*
        $vis:vis enum $name:ident < $($tail:tt)*
    ) => {
        $crate::__markup_component_collect_generics! {
            enum
            [[$(#[$meta])*] [$vis] [$name]]
            []
            [@]
            $($tail)*
        }
    };

    (
        $(#[$meta:meta])*
        $vis:vis enum $name:ident {
            $($variant:tt)*
        } {
            $($arm:tt)*
        }

        $($rest:tt)*
    ) => {
        $crate::__markup_component_item! {
            $(#[$meta])*
            $vis enum $name {
                $($variant)*
            }

            {
                $($arm)*
            }
        }

        $crate::component! { $($rest)* }
    };

    (
        impl < $($tail:tt)*
    ) => {
        $crate::__markup_component_collect_generics! {
            impl_generics
            []
            []
            [@]
            $($tail)*
        }
    };

    (
        impl $name:ident < $($tail:tt)*
    ) => {
        $crate::__markup_component_collect_generics! {
            impl_ty_generics
            [[$name]]
            []
            [@]
            $($tail)*
        }
    };

    (
        impl $name:ident {
            $($impl_body:tt)*
        }

        $($rest:tt)*
    ) => {
        impl $name {
            $($impl_body)*
        }

        $crate::component! { $($rest)* }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __markup_component_collect_generics {
    (
        $mode:ident
        [$($state:tt)*]
        [$($generics:tt)+]
        [@]
        >
        $($tail:tt)*
    ) => {
        $crate::__markup_component_after_generics! {
            $mode
            [$($state)*]
            [$($generics)+]
            $($tail)*
        }
    };

    (
        $mode:ident
        [$($state:tt)*]
        [$($generics:tt)*]
        [@ @]
        >>
        $($tail:tt)*
    ) => {
        $crate::__markup_component_after_generics! {
            $mode
            [$($state)*]
            [$($generics)* >]
            $($tail)*
        }
    };

    (
        $mode:ident
        [$($state:tt)*]
        [$($generics:tt)*]
        [@ @ $($depth:tt)+]
        >>
        $($tail:tt)*
    ) => {
        $crate::__markup_component_collect_generics! {
            $mode
            [$($state)*]
            [$($generics)* > >]
            [$($depth)+]
            $($tail)*
        }
    };

    (
        $mode:ident
        [$($state:tt)*]
        [$($generics:tt)*]
        [@ @ @]
        >>>
        $($tail:tt)*
    ) => {
        $crate::__markup_component_after_generics! {
            $mode
            [$($state)*]
            [$($generics)* > >]
            $($tail)*
        }
    };

    (
        $mode:ident
        [$($state:tt)*]
        [$($generics:tt)*]
        [@ @ @ $($depth:tt)+]
        >>>
        $($tail:tt)*
    ) => {
        $crate::__markup_component_collect_generics! {
            $mode
            [$($state)*]
            [$($generics)* > > >]
            [$($depth)+]
            $($tail)*
        }
    };

    (
        $mode:ident
        [$($state:tt)*]
        [$($generics:tt)*]
        [@ $($depth:tt)+]
        >
        $($tail:tt)*
    ) => {
        $crate::__markup_component_collect_generics! {
            $mode
            [$($state)*]
            [$($generics)* >]
            [$($depth)+]
            $($tail)*
        }
    };

    (
        $mode:ident
        [$($state:tt)*]
        [$($generics:tt)*]
        [$($depth:tt)*]
        <
        $($tail:tt)*
    ) => {
        $crate::__markup_component_collect_generics! {
            $mode
            [$($state)*]
            [$($generics)* <]
            [@ $($depth)*]
            $($tail)*
        }
    };

    (
        $mode:ident
        [$($state:tt)*]
        [$($generics:tt)*]
        [$($depth:tt)*]
        $next:tt
        $($tail:tt)*
    ) => {
        $crate::__markup_component_collect_generics! {
            $mode
            [$($state)*]
            [$($generics)* $next]
            [$($depth)*]
            $($tail)*
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __markup_component_after_generics {
    (
        struct
        [[$(#[$meta:meta])*] [$vis:vis] [$name:ident]]
        [$($generics:tt)+]
        {
            $($field_vis:vis $field:ident : $ty:ty),* $(,)?
        } {
            $($body:tt)*
        }

        $($rest:tt)*
    ) => {
        $crate::__markup_component_generic_args! {
            @parse
            struct
            [
                [$(#[$meta])*]
                [$vis]
                [$name]
                [$($generics)+]
                []
                [$($field_vis $field : $ty),*]
                [$($body)*]
            ]
            []
            []
            $($generics)+
        }

        $crate::component! { $($rest)* }
    };

    (
        struct
        [[$($meta:tt)*] [$vis:vis] [$name:ident]]
        [$($generics:tt)+]
        $next:tt
        $($tail:tt)*
    ) => {
        $crate::__markup_component_after_struct_where! {
            [[$($meta)*] [$vis] [$name]]
            [$($generics)+]
            [$next]
            $($tail)*
        }
    };

    (
        enum
        [[$(#[$meta:meta])*] [$vis:vis] [$name:ident]]
        [$($generics:tt)+]
        {
            $($variant:tt)*
        } {
            $($arm:tt)*
        }

        $($rest:tt)*
    ) => {
        $crate::__markup_component_generic_args! {
            @parse
            enum
            [
                [$(#[$meta])*]
                [$vis]
                [$name]
                [$($generics)+]
                []
                [$($variant)*]
                [$($arm)*]
            ]
            []
            []
            $($generics)+
        }

        $crate::component! { $($rest)* }
    };

    (
        enum
        [[$($meta:tt)*] [$vis:vis] [$name:ident]]
        [$($generics:tt)+]
        $next:tt
        $($tail:tt)*
    ) => {
        $crate::__markup_component_after_enum_where! {
            [[$($meta)*] [$vis] [$name]]
            [$($generics)+]
            [$next]
            $($tail)*
        }
    };

    (
        impl_generics
        []
        [$($impl_generics:tt)+]
        $name:ident < $($tail:tt)*
    ) => {
        $crate::__markup_component_collect_generics! {
            impl_ty_generics
            [[< $($impl_generics)+ >] [$name]]
            []
            [@]
            $($tail)*
        }
    };

    (
        impl_generics
        []
        [$($impl_generics:tt)+]
        $name:ident
        $($tail:tt)*
    ) => {
        $crate::__markup_component_parse_impl! {
            [< $($impl_generics)+ > $name]
            []
            $($tail)*
        }
    };

    (
        impl_ty_generics
        [[$($impl_generics:tt)+] [$name:ident]]
        [$($ty_generics:tt)+]
        $($tail:tt)*
    ) => {
        $crate::__markup_component_parse_impl! {
            [$($impl_generics)+ $name < $($ty_generics)+ >]
            []
            $($tail)*
        }
    };

    (
        impl_ty_generics
        [[$name:ident]]
        [$($ty_generics:tt)+]
        $($tail:tt)*
    ) => {
        $crate::__markup_component_parse_impl! {
            [$name < $($ty_generics)+ >]
            []
            $($tail)*
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __markup_component_after_struct_where {
    (
        [[$(#[$meta:meta])*] [$vis:vis] [$name:ident]]
        [$($generics:tt)+]
        [$($where_clause:tt)*]
        {
            $($field_vis:vis $field:ident : $ty:ty),* $(,)?
        } {
            $($body:tt)*
        }

        $($rest:tt)*
    ) => {
        $crate::__markup_component_generic_args! {
            @parse
            struct
            [
                [$(#[$meta])*]
                [$vis]
                [$name]
                [$($generics)+]
                [$($where_clause)*]
                [$($field_vis $field : $ty),*]
                [$($body)*]
            ]
            []
            []
            $($generics)+
        }

        $crate::component! { $($rest)* }
    };

    (
        [[$($meta:tt)*] [$vis:vis] [$name:ident]]
        [$($generics:tt)+]
        [$($where_clause:tt)*]
        $next:tt
        $($tail:tt)*
    ) => {
        $crate::__markup_component_after_struct_where! {
            [[$($meta)*] [$vis] [$name]]
            [$($generics)+]
            [$($where_clause)* $next]
            $($tail)*
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __markup_component_after_enum_where {
    (
        [[$(#[$meta:meta])*] [$vis:vis] [$name:ident]]
        [$($generics:tt)+]
        [$($where_clause:tt)*]
        {
            $($variant:tt)*
        } {
            $($arm:tt)*
        }

        $($rest:tt)*
    ) => {
        $crate::__markup_component_generic_args! {
            @parse
            enum
            [
                [$(#[$meta])*]
                [$vis]
                [$name]
                [$($generics)+]
                [$($where_clause)*]
                [$($variant)*]
                [$($arm)*]
            ]
            []
            []
            $($generics)+
        }

        $crate::component! { $($rest)* }
    };

    (
        [[$($meta:tt)*] [$vis:vis] [$name:ident]]
        [$($generics:tt)+]
        [$($where_clause:tt)*]
        $next:tt
        $($tail:tt)*
    ) => {
        $crate::__markup_component_after_enum_where! {
            [[$($meta)*] [$vis] [$name]]
            [$($generics)+]
            [$($where_clause)* $next]
            $($tail)*
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __markup_component_parse_impl {
    (
        [$($header:tt)+]
        [$($where_clause:tt)*]
        {
            $($impl_body:tt)*
        }

        $($rest:tt)*
    ) => {
        impl $($header)+ $($where_clause)* {
            $($impl_body)*
        }

        $crate::component! { $($rest)* }
    };

    (
        [$($header:tt)+]
        [$($where_clause:tt)*]
        $next:tt
        $($tail:tt)*
    ) => {
        $crate::__markup_component_parse_impl! {
            [$($header)+]
            [$($where_clause)* $next]
            $($tail)*
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __markup_component_generic_args {
    (@parse $mode:ident [$($state:tt)*] [$($args:tt)*] [$($impl_params:tt)*]) => {
        $crate::__markup_component_generic_args_finish! {
            $mode
            [$($args)*]
            [$($impl_params)*]
            $($state)*
        }
    };

    (@parse $mode:ident [$($state:tt)*] [$($args:tt)*] [$($impl_params:tt)*] , $($tail:tt)*) => {
        $crate::__markup_component_generic_args! {
            @parse
            $mode
            [$($state)*]
            [$($args)*]
            [$($impl_params)*]
            $($tail)*
        }
    };

    (@parse $mode:ident [$($state:tt)*] [$($args:tt)*] [$($impl_params:tt)*] const $name:ident $($tail:tt)*) => {
        $crate::__markup_component_generic_args! {
            @copy_param
            $mode
            [$($state)*]
            [$($args)* $name,]
            [$($impl_params)*]
            [const $name]
            $($tail)*
        }
    };

    (@parse $mode:ident [$($state:tt)*] [$($args:tt)*] [$($impl_params:tt)*] $lifetime:lifetime $($tail:tt)*) => {
        $crate::__markup_component_generic_args! {
            @copy_param
            $mode
            [$($state)*]
            [$($args)* $lifetime,]
            [$($impl_params)*]
            [$lifetime]
            $($tail)*
        }
    };

    (@parse $mode:ident [$($state:tt)*] [$($args:tt)*] [$($impl_params:tt)*] $name:ident $($tail:tt)*) => {
        $crate::__markup_component_generic_args! {
            @copy_param
            $mode
            [$($state)*]
            [$($args)* $name,]
            [$($impl_params)*]
            [$name]
            $($tail)*
        }
    };

    (@copy_param $mode:ident [$($state:tt)*] [$($args:tt)*] [$($impl_params:tt)*] [$($param:tt)*] , $($tail:tt)*) => {
        $crate::__markup_component_generic_args! {
            @parse
            $mode
            [$($state)*]
            [$($args)*]
            [$($impl_params)* $($param)*,]
            $($tail)*
        }
    };

    (@copy_param $mode:ident [$($state:tt)*] [$($args:tt)*] [$($impl_params:tt)*] [$($param:tt)*] = $($tail:tt)*) => {
        $crate::__markup_component_generic_args! {
            @skip_default
            $mode
            [$($state)*]
            [$($args)*]
            [$($impl_params)*]
            [$($param)*]
            []
            $($tail)*
        }
    };

    (@copy_param $mode:ident [$($state:tt)*] [$($args:tt)*] [$($impl_params:tt)*] [$($param:tt)*]) => {
        $crate::__markup_component_generic_args! {
            @parse
            $mode
            [$($state)*]
            [$($args)*]
            [$($impl_params)* $($param)*]
        }
    };

    (@copy_param $mode:ident [$($state:tt)*] [$($args:tt)*] [$($impl_params:tt)*] [$($param:tt)*] < $($tail:tt)*) => {
        $crate::__markup_component_generic_args! {
            @copy_angle
            $mode
            [$($state)*]
            [$($args)*]
            [$($impl_params)*]
            [$($param)* <]
            [@]
            $($tail)*
        }
    };

    (@copy_param $mode:ident [$($state:tt)*] [$($args:tt)*] [$($impl_params:tt)*] [$($param:tt)*] $next:tt $($tail:tt)*) => {
        $crate::__markup_component_generic_args! {
            @copy_param
            $mode
            [$($state)*]
            [$($args)*]
            [$($impl_params)*]
            [$($param)* $next]
            $($tail)*
        }
    };

    (@copy_angle $mode:ident [$($state:tt)*] [$($args:tt)*] [$($impl_params:tt)*] [$($param:tt)*] [@] > $($tail:tt)*) => {
        $crate::__markup_component_generic_args! {
            @copy_param
            $mode
            [$($state)*]
            [$($args)*]
            [$($impl_params)*]
            [$($param)* >]
            $($tail)*
        }
    };

    (@copy_angle $mode:ident [$($state:tt)*] [$($args:tt)*] [$($impl_params:tt)*] [$($param:tt)*] [@ $($depth:tt)+] > $($tail:tt)*) => {
        $crate::__markup_component_generic_args! {
            @copy_angle
            $mode
            [$($state)*]
            [$($args)*]
            [$($impl_params)*]
            [$($param)* >]
            [$($depth)+]
            $($tail)*
        }
    };

    (@copy_angle $mode:ident [$($state:tt)*] [$($args:tt)*] [$($impl_params:tt)*] [$($param:tt)*] [$($depth:tt)*] < $($tail:tt)*) => {
        $crate::__markup_component_generic_args! {
            @copy_angle
            $mode
            [$($state)*]
            [$($args)*]
            [$($impl_params)*]
            [$($param)* <]
            [@ $($depth)*]
            $($tail)*
        }
    };

    (@copy_angle $mode:ident [$($state:tt)*] [$($args:tt)*] [$($impl_params:tt)*] [$($param:tt)*] [$($depth:tt)*] $next:tt $($tail:tt)*) => {
        $crate::__markup_component_generic_args! {
            @copy_angle
            $mode
            [$($state)*]
            [$($args)*]
            [$($impl_params)*]
            [$($param)* $next]
            [$($depth)*]
            $($tail)*
        }
    };

    (@skip_default $mode:ident [$($state:tt)*] [$($args:tt)*] [$($impl_params:tt)*] [$($param:tt)*] [] , $($tail:tt)*) => {
        $crate::__markup_component_generic_args! {
            @parse
            $mode
            [$($state)*]
            [$($args)*]
            [$($impl_params)* $($param)*,]
            $($tail)*
        }
    };

    (@skip_default $mode:ident [$($state:tt)*] [$($args:tt)*] [$($impl_params:tt)*] [$($param:tt)*] []) => {
        $crate::__markup_component_generic_args! {
            @parse
            $mode
            [$($state)*]
            [$($args)*]
            [$($impl_params)* $($param)*]
        }
    };

    (@skip_default $mode:ident [$($state:tt)*] [$($args:tt)*] [$($impl_params:tt)*] [$($param:tt)*] [$($depth:tt)*] < $($tail:tt)*) => {
        $crate::__markup_component_generic_args! {
            @skip_default
            $mode
            [$($state)*]
            [$($args)*]
            [$($impl_params)*]
            [$($param)*]
            [@ $($depth)*]
            $($tail)*
        }
    };

    (@skip_default $mode:ident [$($state:tt)*] [$($args:tt)*] [$($impl_params:tt)*] [$($param:tt)*] [@] > $($tail:tt)*) => {
        $crate::__markup_component_generic_args! {
            @skip_default
            $mode
            [$($state)*]
            [$($args)*]
            [$($impl_params)*]
            [$($param)*]
            []
            $($tail)*
        }
    };

    (@skip_default $mode:ident [$($state:tt)*] [$($args:tt)*] [$($impl_params:tt)*] [$($param:tt)*] [@ $($depth:tt)+] > $($tail:tt)*) => {
        $crate::__markup_component_generic_args! {
            @skip_default
            $mode
            [$($state)*]
            [$($args)*]
            [$($impl_params)*]
            [$($param)*]
            [$($depth)+]
            $($tail)*
        }
    };

    (@skip_default $mode:ident [$($state:tt)*] [$($args:tt)*] [$($impl_params:tt)*] [$($param:tt)*] [$($depth:tt)*] $next:tt $($tail:tt)*) => {
        $crate::__markup_component_generic_args! {
            @skip_default
            $mode
            [$($state)*]
            [$($args)*]
            [$($impl_params)*]
            [$($param)*]
            [$($depth)*]
            $($tail)*
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __markup_component_generic_args_finish {
    (
        struct
        [$($args:tt)*]
        [$($impl_params:tt)*]
        [$(#[$meta:meta])*]
        [$vis:vis]
        [$name:ident]
        [$($decl_generics:tt)+]
        [$($where_clause:tt)*]
        [$($field_vis:vis $field:ident : $ty:ty),*]
        [$($body:tt)*]
    ) => {
        $(#[$meta])*
        $vis struct $name < $($decl_generics)+ > $($where_clause)* {
            $($field_vis $field: $ty),*
        }

        impl < $($impl_params)* > $crate::Render for $name < $($args)* > $($where_clause)* {
            fn render(&self, __markup_children: ::std::option::Option<$crate::Markup>) -> $crate::Markup {
                let _ = &__markup_children;
                $(
                    let $field = &self.$field;
                )*

                $crate::__markup_component_markup!(__markup_children; $($body)*)
            }
        }
    };

    (
        enum
        [$($args:tt)*]
        [$($impl_params:tt)*]
        [$(#[$meta:meta])*]
        [$vis:vis]
        [$name:ident]
        [$($decl_generics:tt)+]
        [$($where_clause:tt)*]
        [$($variant:tt)*]
        [$($arm:tt)*]
    ) => {
        $(#[$meta])*
        $vis enum $name < $($decl_generics)+ > $($where_clause)* {
            $($variant)*
        }

        impl < $($impl_params)* > $crate::Render for $name < $($args)* > $($where_clause)* {
            fn render(&self, __markup_children: ::std::option::Option<$crate::Markup>) -> $crate::Markup {
                let _ = &__markup_children;
                $crate::__markup_component_enum_render! {
                    __markup_children;
                    self;
                    $name;
                    []
                    $($arm)*
                }
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __markup_component_item {
    (
        $(#[$meta:meta])*
        $vis:vis enum $name:ident {
            $($variant:tt)*
        }

        {
            $($arm:tt)*
        }
    ) => {
        $(#[$meta])*
        $vis enum $name {
            $($variant)*
        }

        impl $crate::Render for $name {
            fn render(&self, __markup_children: ::std::option::Option<$crate::Markup>) -> $crate::Markup {
                let _ = &__markup_children;
                $crate::__markup_component_enum_render! {
                    __markup_children;
                    self;
                    $name;
                    []
                    $($arm)*
                }
            }
        }
    };

    (
        $(#[$meta:meta])*
        $vis:vis struct $name:ident {
            $($field_vis:vis $field:ident : $ty:ty),* $(,)?
        }

        $($body:tt)*
    ) => {
        $(#[$meta])*
        $vis struct $name {
            $($field_vis $field: $ty),*
        }

        impl $crate::Render for $name {
            fn render(&self, __markup_children: ::std::option::Option<$crate::Markup>) -> $crate::Markup {
                let _ = &__markup_children;
                $(
                    let $field = &self.$field;
                )*

                $crate::__markup_component_markup!(__markup_children; $($body)*)
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __markup_component_enum_render {
    ($children:ident; $value:expr; $name:ident; [$($arms:tt)*]) => {
        match $value {
            $($arms)*
        }
    };

    ($children:ident; $value:expr; $name:ident; [$($arms:tt)*] $variant:ident => { $($body:tt)* } , $($rest:tt)*) => {
        $crate::__markup_component_enum_render! {
            $children;
            $value;
            $name;
            [
                $($arms)*
                $name :: $variant => {
                    $crate::__markup_component_markup!($children; $($body)*)
                },
            ]
            $($rest)*
        }
    };

    ($children:ident; $value:expr; $name:ident; [$($arms:tt)*] $variant:ident => { $($body:tt)* }) => {
        $crate::__markup_component_enum_render! {
            $children;
            $value;
            $name;
            [
                $($arms)*
                $name :: $variant => {
                    $crate::__markup_component_markup!($children; $($body)*)
                },
            ]
        }
    };

    ($children:ident; $value:expr; $name:ident; [$($arms:tt)*] $variant:ident ( $($pattern:tt)* ) => { $($body:tt)* } , $($rest:tt)*) => {
        $crate::__markup_component_enum_render! {
            $children;
            $value;
            $name;
            [
                $($arms)*
                $name :: $variant ( $($pattern)* ) => {
                    $crate::__markup_component_markup!($children; $($body)*)
                },
            ]
            $($rest)*
        }
    };

    ($children:ident; $value:expr; $name:ident; [$($arms:tt)*] $variant:ident ( $($pattern:tt)* ) => { $($body:tt)* }) => {
        $crate::__markup_component_enum_render! {
            $children;
            $value;
            $name;
            [
                $($arms)*
                $name :: $variant ( $($pattern)* ) => {
                    $crate::__markup_component_markup!($children; $($body)*)
                },
            ]
        }
    };

    ($children:ident; $value:expr; $name:ident; [$($arms:tt)*] $variant:ident { $($pattern:tt)* } => { $($body:tt)* } , $($rest:tt)*) => {
        $crate::__markup_component_enum_render! {
            $children;
            $value;
            $name;
            [
                $($arms)*
                $name :: $variant { $($pattern)* } => {
                    $crate::__markup_component_markup!($children; $($body)*)
                },
            ]
            $($rest)*
        }
    };

    ($children:ident; $value:expr; $name:ident; [$($arms:tt)*] $variant:ident { $($pattern:tt)* } => { $($body:tt)* }) => {
        $crate::__markup_component_enum_render! {
            $children;
            $value;
            $name;
            [
                $($arms)*
                $name :: $variant { $($pattern)* } => {
                    $crate::__markup_component_markup!($children; $($body)*)
                },
            ]
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __markup_component_markup {
    ($children:ident; $($tokens:tt)*) => {
        $crate::__markup_children_markup!([$children]; $($tokens)*)
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __markup_children_markup {
    ($ctx:tt; $($tokens:tt)*) => {{
        let mut __markup_template = $crate::template::TemplateBuilder::new();
        $crate::__markup_nodes!(__markup_template; $ctx; $($tokens)*);
        __markup_template.finish()
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __markup_nodes {
    ($builder:ident; $ctx:tt;) => {};

    ($builder:ident; [$children:ident]; @ children $($rest:tt)*) => {{
        if let ::std::option::Option::Some(__markup_children) = $children.as_ref() {
            $builder.push_markup(__markup_children);
        }
        $crate::__markup_nodes!($builder; [$children]; $($rest)*);
    }};

    ($builder:ident; [$children:ident]; @ Markup :: slot ( ) $($rest:tt)*) => {{
        $crate::__markup_nodes!($builder; [$children]; @ children $($rest)*);
    }};

    ($builder:ident; $ctx:tt; @ let $($tail:tt)*) => {
        $crate::__markup_let!($builder; $ctx; [] $($tail)*);
    };

    ($builder:ident; $ctx:tt; @ fn $name:ident ( $($args:tt)* ) -> $ret:ty { $($body:tt)* } $($rest:tt)*) => {{
        fn $name($($args)*) -> $ret {
            $($body)*
        }
        $crate::__markup_nodes!($builder; $ctx; $($rest)*);
    }};

    ($builder:ident; $ctx:tt; @ fn $name:ident ( $($args:tt)* ) { $($body:tt)* } $($rest:tt)*) => {{
        fn $name($($args)*) {
            $($body)*
        }
        $crate::__markup_nodes!($builder; $ctx; $($rest)*);
    }};

    ($builder:ident; $ctx:tt; @ if $($tail:tt)*) => {
        $crate::__markup_if!($builder; $ctx; [] $($tail)*);
    };

    ($builder:ident; $ctx:tt; @ for $($tail:tt)*) => {
        $crate::__markup_for!($builder; $ctx; [] $($tail)*);
    };

    ($builder:ident; $ctx:tt; @ match $($tail:tt)*) => {
        $crate::__markup_match!($builder; $ctx; [] $($tail)*);
    };

    ($builder:ident; $ctx:tt; @ $macro:ident ! ( $($args:tt)* ) $($rest:tt)*) => {{
        let __markup_markup =
            $crate::template::render(&$macro!($($args)*), ::std::option::Option::None);
        $builder.push_markup(&__markup_markup);
        $crate::__markup_nodes!($builder; $ctx; $($rest)*);
    }};

    ($builder:ident; $ctx:tt; @ $head:ident $(:: $segment:ident)+ ( $($args:tt)* ) { $($component_children:tt)* } $($rest:tt)*) => {{
        let __markup_value = $head $(:: $segment)+ ( $($args)* );
        let __markup_children = $crate::__markup_children_markup!($ctx; $($component_children)*);
        let __markup_markup = $crate::template::render(&__markup_value, ::std::option::Option::Some(__markup_children));
        $builder.push_markup(&__markup_markup);
        $crate::__markup_nodes!($builder; $ctx; $($rest)*);
    }};

    ($builder:ident; $ctx:tt; @ $head:ident $(:: $segment:ident)+ ( $($args:tt)* ) $($rest:tt)*) => {{
        let __markup_markup = $crate::template::render(&$head $(:: $segment)+ ( $($args)* ), ::std::option::Option::None);
        $builder.push_markup(&__markup_markup);
        $crate::__markup_nodes!($builder; $ctx; $($rest)*);
    }};

    ($builder:ident; $ctx:tt; @ $function:ident ( $($args:tt)* ) { $($component_children:tt)* } $($rest:tt)*) => {{
        let __markup_value = $function($($args)*);
        let __markup_children = $crate::__markup_children_markup!($ctx; $($component_children)*);
        let __markup_markup = $crate::template::render(&__markup_value, ::std::option::Option::Some(__markup_children));
        $builder.push_markup(&__markup_markup);
        $crate::__markup_nodes!($builder; $ctx; $($rest)*);
    }};

    ($builder:ident; $ctx:tt; @ $function:ident ( $($args:tt)* ) $($rest:tt)*) => {{
        let __markup_markup =
            $crate::template::render(&$function($($args)*), ::std::option::Option::None);
        $builder.push_markup(&__markup_markup);
        $crate::__markup_nodes!($builder; $ctx; $($rest)*);
    }};

    ($builder:ident; $ctx:tt; @ $component:ident { $field:ident : $($props:tt)* } { $($component_children:tt)* } $($rest:tt)*) => {{
        let __markup_component = $component { $field: $($props)* };
        let __markup_children = $crate::__markup_children_markup!($ctx; $($component_children)*);
        let __markup_markup = $crate::template::render(&__markup_component, ::std::option::Option::Some(__markup_children));
        $builder.push_markup(&__markup_markup);
        $crate::__markup_nodes!($builder; $ctx; $($rest)*);
    }};

    ($builder:ident; $ctx:tt; @ $component:ident { $field:ident : $($props:tt)* } $($rest:tt)*) => {{
        let __markup_component = $component { $field: $($props)* };
        let __markup_markup = $crate::template::render(&__markup_component, ::std::option::Option::None);
        $builder.push_markup(&__markup_markup);
        $crate::__markup_nodes!($builder; $ctx; $($rest)*);
    }};

    ($builder:ident; $ctx:tt; @ $component:ident { $($component_children:tt)* } $($rest:tt)*) => {{
        let __markup_children = $crate::__markup_children_markup!($ctx; $($component_children)*);
        let __markup_markup = $crate::template::render(&$component, ::std::option::Option::Some(__markup_children));
        $builder.push_markup(&__markup_markup);
        $crate::__markup_nodes!($builder; $ctx; $($rest)*);
    }};

    ($builder:ident; $ctx:tt; @ $base:ident $($tail:tt)*) => {
        $crate::__markup_rust_value!($builder; $ctx; [$base] $($tail)*);
    };

    ($builder:ident; $ctx:tt; $text:literal $($rest:tt)*) => {{
        let __markup_markup = $crate::template::render(&$text, ::std::option::Option::None);
        $builder.push_markup(&__markup_markup);
        $crate::__markup_nodes!($builder; $ctx; $($rest)*);
    }};

    ($builder:ident; $ctx:tt; . $($tail:tt)*) => {{
        $crate::__markup_element!($builder; $ctx; "div"; [] []; []; . $($tail)*);
    }};

    ($builder:ident; $ctx:tt; # $($tail:tt)*) => {{
        $crate::__markup_element!($builder; $ctx; "div"; [] []; []; # $($tail)*);
    }};

    ($builder:ident; $ctx:tt; ( $tag:expr ) { $($body:tt)* } $($rest:tt)*) => {{
        $crate::__markup_dynamic_element!($builder; $ctx; $tag; { $($body)* } $($rest)*);
    }};

    ($builder:ident; $ctx:tt; ( $tag:expr ) . $($tail:tt)*) => {{
        $crate::__markup_dynamic_element!($builder; $ctx; $tag; . $($tail)*);
    }};

    ($builder:ident; $ctx:tt; ( $tag:expr ) # $($tail:tt)*) => {{
        $crate::__markup_dynamic_element!($builder; $ctx; $tag; # $($tail)*);
    }};

    ($builder:ident; $ctx:tt; ( $tag:expr ) .. $($tail:tt)*) => {{
        $crate::__markup_dynamic_element!($builder; $ctx; $tag; .. $($tail)*);
    }};

    ($builder:ident; $ctx:tt; ( $tag:expr ) ( $($attrs:tt)* ) $($tail:tt)*) => {{
        $crate::__markup_dynamic_element!($builder; $ctx; $tag; ( $($attrs)* ) $($tail)*);
    }};

    ($builder:ident; $ctx:tt; ( $tag:expr ) $attr:ident $($tail:tt)*) => {{
        $crate::__markup_dynamic_element!($builder; $ctx; $tag; $attr $($tail)*);
    }};

    ($builder:ident; $ctx:tt; ( $value:expr ) $($rest:tt)*) => {{
        let __markup_markup = $crate::template::render(&$value, ::std::option::Option::None);
        $builder.push_markup(&__markup_markup);
        $crate::__markup_nodes!($builder; $ctx; $($rest)*);
    }};

    ($builder:ident; $ctx:tt; $tag:ident - $next:ident $($tail:tt)*) => {{
        $crate::__markup_static_tag!($builder; $ctx; [$tag, $next] $($tail)*);
    }};

    ($builder:ident; $ctx:tt; $tag:ident $($tail:tt)*) => {{
        $crate::__markup_element!($builder; $ctx; stringify!($tag); [] []; []; $($tail)*);
    }};

    ($builder:ident; $ctx:tt; $unexpected:tt $($rest:tt)*) => {
        compile_error!(concat!("unexpected token in markup: ", stringify!($unexpected)));
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __markup_static_tag {
    ($builder:ident; $ctx:tt; [$($segment:ident),+] - $next:ident $($tail:tt)*) => {
        $crate::__markup_static_tag!($builder; $ctx; [$($segment,)+ $next] $($tail)*);
    };

    ($builder:ident; $ctx:tt; [$($segment:ident),+] $($tail:tt)*) => {{
        $crate::__markup_element!(
            $builder;
            $ctx;
            $crate::__markup_join_name!($($segment),+);
            [] [];
            [];
            $($tail)*
        );
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __markup_join_name {
    ($first:ident $(, $segment:ident)*) => {
        concat!(stringify!($first) $(, "-", stringify!($segment))*)
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __markup_dynamic_element {
    ($builder:ident; $ctx:tt; $tag:expr; $($tail:tt)*) => {{
        let __markup_tag = &$tag;
        $crate::__markup_element!($builder; $ctx; __markup_tag; [] []; []; $($tail)*);
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __markup_let {
    ($builder:ident; $ctx:tt; [$($statement:tt)+] ; $($rest:tt)*) => {{
        let $($statement)+;
        $crate::__markup_nodes!($builder; $ctx; $($rest)*);
    }};

    ($builder:ident; $ctx:tt; [$($statement:tt)*] $next:tt $($tail:tt)*) => {
        $crate::__markup_let!($builder; $ctx; [$($statement)* $next] $($tail)*);
    };

    ($builder:ident; $ctx:tt; [$($statement:tt)*]) => {
        compile_error!("expected `;` after @let statement");
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __markup_if {
    ($builder:ident; $ctx:tt; [$($condition:tt)+] { $($body:tt)* } @ else if $($tail:tt)*) => {{
        let __markup_if_matched = ::std::cell::Cell::new(false);
        if $($condition)+ {
            __markup_if_matched.set(true);
            $crate::__markup_nodes!($builder; $ctx; $($body)*);
        }
        $crate::__markup_if_chain!($builder; $ctx; __markup_if_matched; if $($tail)*);
    }};

    ($builder:ident; $ctx:tt; [$($condition:tt)+] { $($body:tt)* } @ else @ if $($tail:tt)*) => {{
        let __markup_if_matched = ::std::cell::Cell::new(false);
        if $($condition)+ {
            __markup_if_matched.set(true);
            $crate::__markup_nodes!($builder; $ctx; $($body)*);
        }
        $crate::__markup_if_chain!($builder; $ctx; __markup_if_matched; if $($tail)*);
    }};

    ($builder:ident; $ctx:tt; [$($condition:tt)+] { $($body:tt)* } @ else { $($else_body:tt)* } $($rest:tt)*) => {{
        if $($condition)+ {
            $crate::__markup_nodes!($builder; $ctx; $($body)*);
        } else {
            $crate::__markup_nodes!($builder; $ctx; $($else_body)*);
        }
        $crate::__markup_nodes!($builder; $ctx; $($rest)*);
    }};

    ($builder:ident; $ctx:tt; [$($condition:tt)+] { $($body:tt)* } $($rest:tt)*) => {{
        if $($condition)+ {
            $crate::__markup_nodes!($builder; $ctx; $($body)*);
        }
        $crate::__markup_nodes!($builder; $ctx; $($rest)*);
    }};

    ($builder:ident; $ctx:tt; [$($condition:tt)*] $next:tt $($tail:tt)*) => {
        $crate::__markup_if!($builder; $ctx; [$($condition)* $next] $($tail)*);
    };

    ($builder:ident; $ctx:tt; [$($condition:tt)*]) => {
        compile_error!("expected `{ ... }` after @if condition");
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __markup_if_chain {
    ($builder:ident; $ctx:tt; $matched:ident; if $($tail:tt)*) => {
        $crate::__markup_if_chain_condition!($builder; $ctx; $matched; [] $($tail)*);
    };

    ($builder:ident; $ctx:tt; $matched:ident; @ if $($tail:tt)*) => {
        $crate::__markup_if_chain_condition!($builder; $ctx; $matched; [] $($tail)*);
    };

    ($builder:ident; $ctx:tt; $matched:ident; @ else if $($tail:tt)*) => {
        $crate::__markup_if_chain_condition!($builder; $ctx; $matched; [] $($tail)*);
    };

    ($builder:ident; $ctx:tt; $matched:ident; @ else @ if $($tail:tt)*) => {
        $crate::__markup_if_chain_condition!($builder; $ctx; $matched; [] $($tail)*);
    };

    ($builder:ident; $ctx:tt; $matched:ident; @ else { $($body:tt)* } $($rest:tt)*) => {{
        if !$matched.get() {
            $crate::__markup_nodes!($builder; $ctx; $($body)*);
        }
        $crate::__markup_nodes!($builder; $ctx; $($rest)*);
    }};

    ($builder:ident; $ctx:tt; $matched:ident; $($rest:tt)*) => {
        $crate::__markup_nodes!($builder; $ctx; $($rest)*);
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __markup_if_chain_condition {
    ($builder:ident; $ctx:tt; $matched:ident; [$($condition:tt)+] { $($body:tt)* } $($tail:tt)*) => {{
        if !$matched.get() {
            if $($condition)+ {
                $matched.set(true);
                $crate::__markup_nodes!($builder; $ctx; $($body)*);
            }
        }
        $crate::__markup_if_chain!($builder; $ctx; $matched; $($tail)*);
    }};

    ($builder:ident; $ctx:tt; $matched:ident; [$($condition:tt)*] $next:tt $($tail:tt)*) => {
        $crate::__markup_if_chain_condition!($builder; $ctx; $matched; [$($condition)* $next] $($tail)*);
    };

    ($builder:ident; $ctx:tt; $matched:ident; [$($condition:tt)*]) => {
        compile_error!("expected `{ ... }` after @else if condition");
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __markup_for {
    ($builder:ident; $ctx:tt; [$($head:tt)+] { $($body:tt)* } $($rest:tt)*) => {{
        for $($head)+ {
            $crate::__markup_nodes!($builder; $ctx; $($body)*);
        }
        $crate::__markup_nodes!($builder; $ctx; $($rest)*);
    }};

    ($builder:ident; $ctx:tt; [$($head:tt)*] $next:tt $($tail:tt)*) => {
        $crate::__markup_for!($builder; $ctx; [$($head)* $next] $($tail)*);
    };

    ($builder:ident; $ctx:tt; [$($head:tt)*]) => {
        compile_error!("expected `{ ... }` after @for iterator");
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __markup_match {
    (
        $builder:ident;
        $ctx:tt;
        [$($value:tt)+]
        {
            $(
                $pattern:pat_param $(| $alt_pattern:pat_param)* $(if $guard:expr)? => { $($body:tt)* } $(,)?
            )*
        }
        $($rest:tt)*
    ) => {{
        match $($value)+ {
            $(
                $pattern $(| $alt_pattern)* $(if $guard)? => {
                    $crate::__markup_nodes!($builder; $ctx; $($body)*);
                }
            )*
        }
        $crate::__markup_nodes!($builder; $ctx; $($rest)*);
    }};

    ($builder:ident; $ctx:tt; [$($value:tt)*] $next:tt $($tail:tt)*) => {
        $crate::__markup_match!($builder; $ctx; [$($value)* $next] $($tail)*);
    };

    ($builder:ident; $ctx:tt; [$($value:tt)*]) => {
        compile_error!("expected `{ ... }` after @match value");
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __markup_rust_value {
    ($builder:ident; $ctx:tt; [$($value:tt)+] . $field:ident $($tail:tt)*) => {
        $crate::__markup_rust_value!($builder; $ctx; [$($value)+ . $field] $($tail)*);
    };

    ($builder:ident; $ctx:tt; [$($value:tt)+] $($rest:tt)*) => {{
        let __markup_markup = $crate::template::render(&$($value)+, ::std::option::Option::None);
        $builder.push_markup(&__markup_markup);
        $crate::__markup_nodes!($builder; $ctx; $($rest)*);
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __markup_element {
    ($builder:ident; $ctx:tt; $tag:expr; [$($class:expr,)*] [$($id:expr)?]; [$($attrs:tt)*]; . ( $class_value:expr ) $($tail:tt)*) => {
        $crate::__markup_element!(
            $builder;
            $ctx;
            $tag;
            [$($class,)* $class_value,]
            [$($id)?];
            [$($attrs)*];
            $($tail)*
        );
    };

    ($builder:ident; $ctx:tt; $tag:expr; [$($class:expr,)*] [$($id:expr)?]; [$($attrs:tt)*]; . $class_name:ident $($tail:tt)*) => {
        $crate::__markup_element!(
            $builder;
            $ctx;
            $tag;
            [$($class,)* stringify!($class_name),]
            [$($id)?];
            [$($attrs)*];
            $($tail)*
        );
    };

    ($builder:ident; $ctx:tt; $tag:expr; [$($class:expr,)*] [$($id:expr)?]; [$($attrs:tt)*]; # ( $id_value:expr ) $($tail:tt)*) => {
        $crate::__markup_element!(
            $builder;
            $ctx;
            $tag;
            [$($class,)*]
            [$id_value];
            [$($attrs)*];
            $($tail)*
        );
    };

    ($builder:ident; $ctx:tt; $tag:expr; [$($class:expr,)*] [$($id:expr)?]; [$($attrs:tt)*]; # $id_name:ident $($tail:tt)*) => {
        $crate::__markup_element!(
            $builder;
            $ctx;
            $tag;
            [$($class,)*]
            [stringify!($id_name)];
            [$($attrs)*];
            $($tail)*
        );
    };

    ($builder:ident; $ctx:tt; $tag:expr; [$($class:expr,)*] [$($id:expr)?]; [$($attrs:tt)*]; ; $($rest:tt)*) => {{
        $crate::template::push_start_tag(&mut $builder.current, $tag);
        $crate::__markup_classes_attr!($builder.current; $($class,)*);
        $(
            $crate::template::push_attr(&mut $builder.current, "id", &$id);
        )?
        $crate::__markup_attrs!($builder.current; $($attrs)*);
        $crate::template::finish_void_tag(&mut $builder.current, $tag);
        $crate::__markup_nodes!($builder; $ctx; $($rest)*);
    }};

    ($builder:ident; $ctx:tt; $tag:expr; [$($class:expr,)*] [$($id:expr)?]; [$($attrs:tt)*]; { $($body:tt)* } $($rest:tt)*) => {{
        $crate::template::push_start_tag(&mut $builder.current, $tag);
        $crate::__markup_classes_attr!($builder.current; $($class,)*);
        $(
            $crate::template::push_attr(&mut $builder.current, "id", &$id);
        )?
        $crate::__markup_attrs!($builder.current; $($attrs)*);
        $crate::template::finish_start_tag(&mut $builder.current);
        $crate::__markup_nodes!($builder; $ctx; $($body)*);
        $crate::template::push_end_tag(&mut $builder.current, $tag);
        $crate::__markup_nodes!($builder; $ctx; $($rest)*);
    }};

    ($builder:ident; $ctx:tt; $tag:expr; [$($class:expr,)*] [$($id:expr)?]; [$($attrs:tt)*]; = $base:ident . $field:ident $($tail:tt)*) => {
        $crate::__markup_element_attr_field_value!(
            $builder;
            $ctx;
            $tag;
            [$($class,)*]
            [$($id)?];
            [$($attrs)*];
            [$base . $field]
            $($tail)*
        );
    };

    ($builder:ident; $ctx:tt; $tag:expr; [$($class:expr,)*] [$($id:expr)?]; [$($attrs:tt)*]; $next:tt $($tail:tt)*) => {
        $crate::__markup_element!(
            $builder;
            $ctx;
            $tag;
            [$($class,)*]
            [$($id)?];
            [$($attrs)* $next];
            $($tail)*
        );
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __markup_element_attr_field_value {
    ($builder:ident; $ctx:tt; $tag:expr; [$($class:expr,)*] [$($id:expr)?]; [$($attrs:tt)*]; [$($value:tt)+] . $field:ident $($tail:tt)*) => {
        $crate::__markup_element_attr_field_value!(
            $builder;
            $ctx;
            $tag;
            [$($class,)*]
            [$($id)?];
            [$($attrs)*];
            [$($value)+ . $field]
            $($tail)*
        );
    };

    ($builder:ident; $ctx:tt; $tag:expr; [$($class:expr,)*] [$($id:expr)?]; [$($attrs:tt)*]; [$($value:tt)+] $($tail:tt)*) => {
        $crate::__markup_element!(
            $builder;
            $ctx;
            $tag;
            [$($class,)*]
            [$($id)?];
            [$($attrs)* = $($value)+];
            $($tail)*
        );
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __markup_classes_attr {
    ($out:expr;) => {};

    ($out:expr; $($class:expr,)+) => {{
        let mut __markup_classes = ::std::string::String::new();
        $(
            $crate::template::push_class_value(&mut __markup_classes, &$class);
        )+
        $crate::template::push_class_attr(&mut $out, &__markup_classes);
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __markup_attrs {
    ($out:expr;) => {};

    ($out:expr; .. $attrs:ident $($rest:tt)*) => {{
        $crate::template::push_attrs(&mut $out, &$attrs);
        $crate::__markup_attrs!($out; $($rest)*);
    }};

    ($out:expr; .. [$($attrs:tt)*] $($rest:tt)*) => {{
        $crate::template::push_attrs(&mut $out, &[$($attrs)*]);
        $crate::__markup_attrs!($out; $($rest)*);
    }};

    ($out:expr; .. ($attrs:expr) $($rest:tt)*) => {{
        $crate::template::push_attrs(&mut $out, &$attrs);
        $crate::__markup_attrs!($out; $($rest)*);
    }};

    ($out:expr; (.. $attrs:ident) $($rest:tt)*) => {{
        $crate::template::push_attrs(&mut $out, &$attrs);
        $crate::__markup_attrs!($out; $($rest)*);
    }};

    ($out:expr; (.. [$($attrs:tt)*]) $($rest:tt)*) => {{
        $crate::template::push_attrs(&mut $out, &[$($attrs)*]);
        $crate::__markup_attrs!($out; $($rest)*);
    }};

    ($out:expr; (.. $attrs:expr) $($rest:tt)*) => {{
        $crate::template::push_attrs(&mut $out, &$attrs);
        $crate::__markup_attrs!($out; $($rest)*);
    }};

    ($out:expr; ($attrs:ident ...) $($rest:tt)*) => {
        compile_error!("attribute spread syntax changed: use `..attrs` instead of `(attrs...)`");
    };

    ($out:expr; ([$($attrs:tt)*] ...) $($rest:tt)*) => {
        compile_error!("attribute spread syntax changed: use `..[attrs]` instead of `([attrs]...)`");
    };

    ($out:expr; ($name:expr) = ($value:expr) $($rest:tt)*) => {{
        $crate::template::push_attr(&mut $out, &$name, &$value);
        $crate::__markup_attrs!($out; $($rest)*);
    }};

    ($out:expr; ($name:expr) = $function:ident $(:: $segment:ident)* ( $($args:tt)* ) $($rest:tt)*) => {{
        $crate::template::push_attr(&mut $out, &$name, &$function $(:: $segment)* ($($args)*));
        $crate::__markup_attrs!($out; $($rest)*);
    }};

    ($out:expr; ($name:expr) = $base:ident . $field:ident $($rest:tt)*) => {
        $crate::__markup_attr_field_value!($out; name ($name); [$base . $field] $($rest)*);
    };

    ($out:expr; ($name:expr) = $value:literal $($rest:tt)*) => {{
        $crate::template::push_attr(&mut $out, &$name, &$value);
        $crate::__markup_attrs!($out; $($rest)*);
    }};

    ($out:expr; $name:literal = ($value:expr) $($rest:tt)*) => {{
        $crate::template::push_attr(&mut $out, &$name, &$value);
        $crate::__markup_attrs!($out; $($rest)*);
    }};

    ($out:expr; $name:literal = $function:ident $(:: $segment:ident)* ( $($args:tt)* ) $($rest:tt)*) => {{
        $crate::template::push_attr(&mut $out, &$name, &$function $(:: $segment)* ($($args)*));
        $crate::__markup_attrs!($out; $($rest)*);
    }};

    ($out:expr; $name:literal = $base:ident . $field:ident $($rest:tt)*) => {
        $crate::__markup_attr_field_value!($out; name ($name); [$base . $field] $($rest)*);
    };

    ($out:expr; $name:literal = $value:literal $($rest:tt)*) => {{
        $crate::template::push_attr(&mut $out, &$name, &$value);
        $crate::__markup_attrs!($out; $($rest)*);
    }};

    ($out:expr; $name:ident $($tail:tt)*) => {
        $crate::__markup_attr_name!($out; [$name] $($tail)*);
    };

    ($out:expr; $unexpected:tt $($rest:tt)*) => {
        compile_error!(concat!("unexpected token in attributes: ", stringify!($unexpected)));
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __markup_attr_field_value {
    ($out:expr; name ($name:expr); [$($value:tt)+] . $field:ident $($rest:tt)*) => {
        $crate::__markup_attr_field_value!($out; name ($name); [$($value)+ . $field] $($rest)*);
    };

    ($out:expr; name ($name:expr); [$($value:tt)+] $($rest:tt)*) => {{
        $crate::template::push_attr(&mut $out, &$name, &$($value)+);
        $crate::__markup_attrs!($out; $($rest)*);
    }};

    ($out:expr; segments [$($segment:ident),+]; [$($value:tt)+] . $field:ident $($rest:tt)*) => {
        $crate::__markup_attr_field_value!($out; segments [$($segment),+]; [$($value)+ . $field] $($rest)*);
    };

    ($out:expr; segments [$($segment:ident),+]; [$($value:tt)+] $($rest:tt)*) => {{
        $crate::template::push_attr_segments(&mut $out, &[$(stringify!($segment)),+], &$($value)+);
        $crate::__markup_attrs!($out; $($rest)*);
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __markup_attr_name {
    ($out:expr; [$($segment:ident),+] - $next:ident $($tail:tt)*) => {
        $crate::__markup_attr_name!($out; [$($segment,)+ $next] $($tail)*);
    };

    ($out:expr; [$($segment:ident),+] = ($value:expr) $($rest:tt)*) => {{
        $crate::template::push_attr_segments(&mut $out, &[$(stringify!($segment)),+], &$value);
        $crate::__markup_attrs!($out; $($rest)*);
    }};

    ($out:expr; [$($segment:ident),+] = $function:ident $(:: $path_segment:ident)* ( $($args:tt)* ) $($rest:tt)*) => {{
        $crate::template::push_attr_segments(&mut $out, &[$(stringify!($segment)),+], &$function $(:: $path_segment)* ($($args)*));
        $crate::__markup_attrs!($out; $($rest)*);
    }};

    ($out:expr; [$($segment:ident),+] = $base:ident . $field:ident $($rest:tt)*) => {
        $crate::__markup_attr_field_value!($out; segments [$($segment),+]; [$base . $field] $($rest)*);
    };

    ($out:expr; [$($segment:ident),+] = $value:literal $($rest:tt)*) => {{
        $crate::template::push_attr_segments(&mut $out, &[$(stringify!($segment)),+], &$value);
        $crate::__markup_attrs!($out; $($rest)*);
    }};

    ($out:expr; [$($segment:ident),+] $($rest:tt)*) => {{
        $crate::template::push_bool_attr_segments(&mut $out, &[$(stringify!($segment)),+]);
        $crate::__markup_attrs!($out; $($rest)*);
    }};
}
