#[macro_export]
macro_rules! markup {
    ($($tokens:tt)*) => {{
        let mut __markup_template = $crate::template::TemplateBuilder::new();
        $crate::__markup_nodes!(__markup_template; []; $($tokens)*);
        __markup_template.finish()
    }};
}

#[macro_export]
macro_rules! component {
    () => {};

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
        $vis:vis enum $name:ident {
            $($variant:ident),* $(,)?
        } {
            $(
                $arm_variant:ident => { $($body:tt)* }
            ),* $(,)?
        }

        $($rest:tt)*
    ) => {
        $crate::__markup_component_item! {
            $(#[$meta])*
            $vis enum $name {
                $($variant),*
            }

            {
                $(
                    $arm_variant => { $($body)* }
                ),*
            }
        }

        $crate::component! { $($rest)* }
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
macro_rules! __markup_component_item {
    (
        $(#[$meta:meta])*
        $vis:vis enum $name:ident {
            $($variant:ident),* $(,)?
        }

        {
            $(
                $arm_variant:ident => { $($body:tt)* }
            ),* $(,)?
        }
    ) => {
        $(#[$meta])*
        $vis enum $name {
            $($variant),*
        }

        impl $crate::Render for $name {
            fn render(&self, __markup_children: ::std::option::Option<$crate::Markup>) -> $crate::Markup {
                let _ = &__markup_children;
                match self {
                    $(
                        $name :: $arm_variant => {
                            $crate::__markup_component_markup!(__markup_children; $($body)*)
                        }
                    )*
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

    ($builder:ident; $ctx:tt; ( $($value:tt)+ ) $($rest:tt)*) => {{
        let __markup_markup = $crate::template::render(&({ $($value)+ }), ::std::option::Option::None);
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

    ($out:expr; [$($segment:ident),+] = $value:literal $($rest:tt)*) => {{
        $crate::template::push_attr_segments(&mut $out, &[$(stringify!($segment)),+], &$value);
        $crate::__markup_attrs!($out; $($rest)*);
    }};

    ($out:expr; [$($segment:ident),+] $($rest:tt)*) => {{
        $crate::template::push_bool_attr_segments(&mut $out, &[$(stringify!($segment)),+]);
        $crate::__markup_attrs!($out; $($rest)*);
    }};
}
