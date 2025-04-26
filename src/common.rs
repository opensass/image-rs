use std::str::FromStr;

/// Enum representing the layout of an image.
///
/// Specifies how an image should be positioned or sized within its container.
/// The default layout is `Layout::Auto`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Layout {
    /// The image should stretch to fill the container, possibly distorting the aspect ratio.
    Fill,

    /// The image will scale the size of the container while maintaining the aspect ratio.
    Responsive,

    /// The image will be displayed at its intrinsic size (its natural width and height).
    Intrinsic,

    /// The image will have a fixed width and height, regardless of the container size.
    Fixed,

    /// Automatically adjusts the layout based on other properties (default behavior).
    #[default]
    Auto,

    /// Stretches the image to fill the container, similar to `Fill`, but respects the aspect ratio more strictly.
    Stretch,

    /// The image is scaled down to fit the container but does not scale up beyond its original size.
    ScaleDown,
}

impl Layout {
    /// Converts the `Layout` enum variant to its corresponding string representation.
    ///
    /// Returns a string that represents the layout type, useful for passing as a CSS property.
    pub fn as_str(&self) -> &'static str {
        match self {
            Layout::Fill => "fill",
            Layout::Responsive => "responsive",
            Layout::Intrinsic => "intrinsic",
            Layout::Fixed => "fixed",
            Layout::Auto => "auto",
            Layout::Stretch => "stretch",
            Layout::ScaleDown => "scale-down",
        }
    }
}

impl FromStr for Layout {
    type Err = ();

    /// Parses a string into a `Layout` enum variant.
    ///
    /// This method converts a string representation of a layout into the corresponding `Layout` enum variant.
    /// If the string doesn't match any valid layout value, it returns an error.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "fill" => Ok(Layout::Fill),
            "responsive" => Ok(Layout::Responsive),
            "intrinsic" => Ok(Layout::Intrinsic),
            "fixed" => Ok(Layout::Fixed),
            "auto" => Ok(Layout::Auto),
            "stretch" => Ok(Layout::Stretch),
            "scale-down" => Ok(Layout::ScaleDown),
            _ => Err(()),
        }
    }
}

/// Enum for the `decoding` attribute of the `Image` component.
///
/// Specifies how the image should be decoded. This controls when the browser decodes the image
/// relative to its loading behavior. It ensures that only valid decoding options are used,
/// improving type safety and preventing mistakes.
///
/// The default behavior is `Auto`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Decoding {
    /// Let the browser automatically decide the best decoding method.
    #[default]
    Auto,

    /// Force synchronous decoding (may block other operations).
    Sync,

    /// Allow asynchronous decoding (non-blocking).
    Async,
}

impl Decoding {
    /// Returns the string value associated with the `Decoding` option.
    ///
    /// Useful for setting HTML attributes.
    pub fn as_str(&self) -> &'static str {
        match self {
            Decoding::Auto => "auto",
            Decoding::Sync => "sync",
            Decoding::Async => "async",
        }
    }
}

impl FromStr for Decoding {
    type Err = ();

    /// Parses a string into a `Decoding` enum variant.
    ///
    /// Accepts case-insensitive inputs like `"auto"`, `"sync"`, or `"async"`.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "auto" => Ok(Decoding::Auto),
            "sync" => Ok(Decoding::Sync),
            "async" => Ok(Decoding::Async),
            _ => Err(()),
        }
    }
}

/// Enum representing possible values for the `object-position` attribute of the `Image` component.
///
/// Controls how the image is positioned inside its container when using `object-fit`.
/// This provides predefined common positions while ensuring type safety.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Position {
    /// Center the image both horizontally and vertically (default).
    #[default]
    Center,

    /// Align the image to the top.
    Top,

    /// Align the image to the bottom.
    Bottom,

    /// Align the image to the left.
    Left,

    /// Align the image to the right.
    Right,

    /// Align the image to the top-left corner.
    TopLeft,

    /// Align the image to the top-right corner.
    TopRight,

    /// Align the image to the bottom-left corner.
    BottomLeft,

    /// Align the image to the bottom-right corner.
    BottomRight,
}

impl Position {
    /// Returns the string value associated with the `Position` option.
    ///
    /// Useful for setting the `object-position` CSS property.
    pub fn as_str(&self) -> &'static str {
        match self {
            Position::Center => "center",
            Position::Top => "top",
            Position::Bottom => "bottom",
            Position::Left => "left",
            Position::Right => "right",
            Position::TopLeft => "top left",
            Position::TopRight => "top right",
            Position::BottomLeft => "bottom left",
            Position::BottomRight => "bottom right",
        }
    }
}

/// Enum representing possible values for the `object-fit` attribute of the `Image` component.
///
/// Defines how the image should be resized to fit its container.
/// Provides predefined options while ensuring type safety.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ObjectFit {
    /// Fill the container without preserving the aspect ratio.
    Fill,

    /// Contain the image within the container while preserving aspect ratio.
    #[default]
    Contain,

    /// Cover the container while preserving aspect ratio (may crop).
    Cover,

    /// Scale down the image only if necessary.
    ScaleDown,

    /// Do not resize the image.
    None,
}

impl ObjectFit {
    /// Returns the string value associated with the `ObjectFit` option.
    ///
    /// Useful for setting the `object-fit` CSS property.
    pub fn as_str(&self) -> &'static str {
        match self {
            ObjectFit::Fill => "fill",
            ObjectFit::Contain => "contain",
            ObjectFit::Cover => "cover",
            ObjectFit::ScaleDown => "scale-down",
            ObjectFit::None => "none",
        }
    }
}
