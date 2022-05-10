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
        pub use text_field::text_area::TextArea;
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

cfg_if! {
    if #[cfg(feature = "card")] {
        pub mod card;
        pub use card::Card;
        pub use card::PrimaryAction;
        pub use card::Media;
        pub use card::MediaStyle;
    }
}

cfg_if! {
    if #[cfg(feature = "dialog")] {
        pub mod dialog;
        pub use dialog::Dialog;
    }
}

cfg_if! {
    if #[cfg(feature = "snackbar")] {
        pub mod snackbar;
        pub use snackbar::Snackbar;
    }
}

cfg_if! {
    if #[cfg(feature = "switch")] {
        pub mod switch;
        pub use switch::Switch;
    }
}

cfg_if! {
    if #[cfg(feature = "image-list")] {
        pub mod image_list;
        pub use image_list::ImageList;
        pub use image_list::Item as ImageListItem;
        pub use image_list::Image as ImageListImage;
        // Not sure how to call this, I don't expect any other types
        // being called "Supporting" tho.
        pub use image_list::Supporting;
    }
}

cfg_if! {
    if #[cfg(feature = "tabs")] {
        pub mod tabs;
        pub use tabs::*;
    }
}

cfg_if! {
    if #[cfg(feature = "select")] {
        pub mod select;
        pub use select::Select;
        pub use select::Item as SelectItem;
    }
}

cfg_if! {
    if #[cfg(feature = "list")] {
        pub mod list;
        pub use list::List;
        pub use list::Item as ListItem;
    }
}
