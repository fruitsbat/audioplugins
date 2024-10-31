use nih_plug_iced::Font;

pub mod size;

pub const FIRA_BOLD: Font = Font::External {
    name: "Fira Sans Bold",
    bytes: include_bytes!("../../assets/FiraSans-Bold.ttf"),
};

pub const FIRA_REGULAR: Font = Font::External {
    name: "Fira Sans Regular",
    bytes: include_bytes!("../../assets/FiraSans-Regular.ttf"),
};