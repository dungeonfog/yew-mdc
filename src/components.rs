use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "button")] {
        pub mod button;
        pub use button::Button;
    }
}

cfg_if! {
    if #[cfg(feature = "icon-button")] {
        pub mod icon_button;
        pub use icon_button::IconButton;
    }
}

cfg_if! {
    if #[cfg(feature = "fab")] {
        pub mod fab;
        pub use fab::FAB;
    }
}

cfg_if! {
    if #[cfg(feature = "text-field")] {
        pub mod text_field;
        pub use text_field::HelperLine as TextFieldHelperLine;
        pub use text_field::TextField;
    }
}

cfg_if! {
    if #[cfg(feature = "menu")] {
        pub mod menu;
        pub use menu::Item as MenuItem;
        pub use menu::Menu;
    }
}

cfg_if! {
    if #[cfg(feature = "drawer")] {
        pub mod drawer;
        pub use drawer::Content as DrawerContent;
        pub use drawer::Drawer;
        pub use drawer::Header as DrawerHeader;
    }
}

cfg_if! {
    if #[cfg(feature = "top-app-bar")] {
        pub mod top_app_bar;
        pub use top_app_bar::Section as TopAppBarSection;
        pub use top_app_bar::TopAppBar;
    }
}
