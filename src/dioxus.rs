#![doc = include_str!("../DIOXUS.md")]

use crate::common::{
    AriaLive, AriaPressed, CrossOrigin, Decoding, FetchPriority, Layout, Loading, ObjectFit,
    Position, ReferrerPolicy,
};
use dioxus::prelude::*;
use gloo_net::http::Request;
use web_sys::IntersectionObserverEntry;
use web_sys::js_sys;
use web_sys::wasm_bindgen::JsCast;
use web_sys::wasm_bindgen::prelude::*;
use web_sys::{IntersectionObserver, IntersectionObserverInit};

/// Properties for the `Image` component.
///
/// The `Image` component allows you to display an image with various customization options
/// for layout, styling, and behavior. It supports fallback images, lazy loading, and custom
/// callbacks for error handling and loading completion.
///
/// This component is highly flexible, providing support for multiple image layouts,
/// object-fit, object-position, ARIA attributes, and more.
///
/// # See Also
/// - [MDN img Element](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/img)
#[derive(Props, Clone, PartialEq)]
pub struct ImageProps {
    /// The source URL of the image.
    ///
    /// This is the URL of the image to be displayed. This property is required for loading
    /// an image. If not provided, the image will not be displayed.
    #[props(default = "")]
    pub src: &'static str,

    /// The alternative text for the image.
    ///
    /// This is the alt text for the image, which is used for accessibility purposes.
    /// If not provided, the alt text will be empty.
    #[props(default = "")]
    pub alt: &'static str,

    /// Optional fallback image.
    ///
    /// This image will be displayed if the main image fails to load. If not provided,
    /// the image will attempt to load without a fallback.
    #[props(default = "")]
    pub fallback_src: &'static str,

    /// The width of the image.
    ///
    /// Specifies the width of the image in pixels. It is typically used for responsive
    /// layouts. Defaults to an empty string if not provided.
    #[props(default = "")]
    pub width: &'static str,

    /// The height of the image.
    ///
    /// Specifies the height of the image in pixels. Like `width`, it is often used for
    /// responsive layouts. Defaults to an empty string if not provided.
    #[props(default = "")]
    pub height: &'static str,

    // Common props
    /// The style attribute for the image.
    ///
    /// Allows you to apply custom inline CSS styles to the image. Defaults to an empty string.
    #[props(default = "")]
    pub style: &'static str,

    /// The CSS class for the image.
    ///
    /// This can be used to apply custom CSS classes to the image for styling purposes.
    /// Defaults to an empty string if not provided.
    #[props(default = "")]
    pub class: &'static str,

    /// The sizes attribute for the image.
    ///
    /// This is used to define different image sizes for different viewport widths, helping
    /// with responsive images. Defaults to an empty string if not provided.
    #[props(default = "")]
    pub sizes: &'static str,

    /// The quality attribute for the image.
    ///
    /// Allows you to set the quality of the image (e.g., "low", "medium", "high"). Defaults
    /// to an empty string if not provided.
    #[props(default = "")]
    pub quality: &'static str,

    /// Indicates if the image should have priority loading.
    ///
    /// This controls whether the image should be loaded eagerly (immediately) or lazily
    #[props(default)]
    pub loading: Loading,

    /// The placeholder attribute for the image.
    ///
    /// Allows you to specify a placeholder image URL or data URL to show while the main
    /// image is loading. Defaults to an empty string.
    #[props(default = "")]
    pub placeholder: &'static str,

    /// Callback function for handling loading completion.
    ///
    /// This callback is triggered once the image has finished loading. This is useful for
    /// actions that should happen after the image has been fully loaded, such as hiding
    /// a loading spinner. Defaults to a no-op.
    #[props(default)]
    pub on_load: Callback<()>,

    // Advanced Props
    /// The object-fit attribute for the image.
    ///
    /// Determines how the image should be resized to fit its container. Common values include
    /// "contain", "cover", "fill", etc. Defaults to an empty string.
    #[props(default)]
    pub object_fit: ObjectFit,

    /// The object-position attribute for the image.
    ///
    /// Specifies how the image should be positioned within its container when `object-fit` is set.
    /// The available options are:
    /// - `Position::Center`: Centers the image within the container.
    /// - `Position::Top`: Aligns the image to the top of the container.
    /// - `Position::Bottom`: Aligns the image to the bottom of the container.
    /// - `Position::Left`: Aligns the image to the left of the container.
    /// - `Position::Right`: Aligns the image to the right of the container.
    /// - `Position::TopLeft`: Aligns the image to the top-left of the container.
    /// - `Position::TopRight`: Aligns the image to the top-right of the container.
    /// - `Position::BottomLeft`: Aligns the image to the bottom-left of the container.
    /// - `Position::BottomRight`: Aligns the image to the bottom-right of the container.
    ///
    /// Defaults to `Position::Center`.
    #[props(default)]
    pub object_position: Position,

    /// Callback function for handling errors during image loading.
    ///
    /// This callback is triggered if the image fails to load, allowing you to handle
    /// error states (e.g., displaying a fallback image or showing an error message).
    #[props(default)]
    pub on_error: Callback<String>,

    /// The decoding attribute for the image.
    ///
    /// Specifies how the image should be decoded. The available options are:
    /// - `Decoding::Auto`: The image decoding behavior is automatically decided by the browser.
    /// - `Decoding::Sync`: The image is decoded synchronously (blocking other tasks).
    /// - `Decoding::Async`: The image is decoded asynchronously (non-blocking).
    ///
    /// Defaults to `Decoding::Auto`.
    #[props(default)]
    pub decoding: Decoding,

    /// The blur data URL for placeholder image.
    ///
    /// This is used to display a low-quality blurred version of the image while the full
    /// image is loading. Defaults to an empty string.
    #[props(default = "")]
    pub blur_data_url: &'static str,

    /// The lazy boundary for lazy loading.
    ///
    /// Defines the distance (in pixels) from the viewport at which the image should start
    /// loading. Defaults to an empty string.
    #[props(default = "")]
    pub lazy_boundary: &'static str,

    /// Indicates if the image should be unoptimized.
    ///
    /// If set to `true`, the image will be loaded without any optimization applied (e.g.,
    /// no resizing or compression). Defaults to `false`.
    #[props(default = false)]
    pub unoptimized: bool,

    /// Image layout.
    ///
    /// Specifies how the image should be laid out within its container. Possible values
    /// include `Layout::Fill`, `Layout::Responsive`, `Layout::Intrinsic`, `Layout::Fixed`,
    /// `Layout::Auto`, `Layout::Stretch`, and `Layout::ScaleDown`. Defaults to `Layout::Responsive`.
    #[props(default)]
    pub layout: Layout,

    // /// Reference to the DOM node.
    // ///
    // /// This is used to create a reference to the actual DOM element of the image. It is
    // /// useful for directly manipulating the image element via JavaScript if needed.
    // // TODO: Figure out how to pass a node ref
    // #[prop_or_default]
    // pub node_ref: Node,
    /// A list of one or more image sources for responsive loading.
    ///
    /// Defines multiple image resources for the browser to choose from, depending on screen size, resolution,
    /// and other factors. Each source can include width (`w`) or pixel density (`x`) descriptors.
    #[props(default)]
    pub srcset: &'static str,

    /// Cross-origin policy to use when fetching the image.
    ///
    /// Determines whether the image should be fetched with CORS enabled. Useful when the image needs to be accessed
    /// in a `<canvas>` element. Accepts `anonymous` or `use-credentials`.
    #[props(default)]
    pub crossorigin: CrossOrigin,

    /// Referrer policy to apply when fetching the image.
    ///
    /// Controls how much referrer information should be included with requests made for the image resource.
    /// Common values include `no-referrer`, `origin`, `strict-origin-when-cross-origin`, etc.
    #[props(default)]
    pub referrerpolicy: ReferrerPolicy,

    /// The fragment identifier of the image map to use.
    ///
    /// Associates the image with a `<map>` element, enabling clickable regions within the image. The value
    /// should begin with `#` and match the `name` of the corresponding map element.
    #[props(default)]
    pub usemap: &'static str,

    /// Indicates that the image is part of a server-side image map.
    ///
    /// When set, clicking the image will send the click coordinates to the server. Only allowed when the image
    /// is inside an `<a>` element with a valid `href`.
    #[props(default)]
    pub ismap: bool,

    /// Hints the browser about the priority of fetching this image.
    ///
    /// Helps the browser prioritize network resource loading. Accepts `high`, `low`, or `auto` (default).
    /// See `HTMLImageElement.fetchPriority` for more.
    #[props(default)]
    pub fetchpriority: FetchPriority,

    /// Identifier for tracking image performance timing.
    ///
    /// Registers the image with the `PerformanceElementTiming` API using the given string as its ID. Useful for
    /// performance monitoring and analytics.
    #[props(default)]
    pub elementtiming: &'static str,

    /// URL(s) to send Attribution Reporting requests for the image.
    ///
    /// Indicates that the browser should send an `Attribution-Reporting-Eligible` header with the image request.
    /// Can be a boolean or a list of URLs for attribution registration on specified servers. Experimental feature.
    #[props(default)]
    pub attributionsrc: &'static str,

    /// Indicates the current state of the image in a navigation menu.
    ///
    /// Valid values are "page", "step", "location", "date", "time", "true", "false".
    /// This is useful for enhancing accessibility in navigation menus.
    #[props(default = "")]
    pub aria_current: &'static str,

    /// Describes the image using the ID of the element that provides a description.
    ///
    /// The ID of the element that describes the image. This is used for accessibility
    /// purposes, particularly for screen readers.
    #[props(default = "")]
    pub aria_describedby: &'static str,

    /// Indicates whether the content associated with the image is currently expanded or collapsed.
    ///
    /// This is typically used for ARIA-based accessibility and is represented as "true" or "false".
    #[props(default = "")]
    pub aria_expanded: &'static str,

    /// Indicates whether the image is currently hidden from the user.
    ///
    /// This attribute is used for accessibility and indicates whether the image is visible
    /// to the user or not. Valid values are "true" or "false".
    #[props(default = "")]
    pub aria_hidden: &'static str,

    /// Indicates whether the content associated with the image is live and dynamic.
    ///
    /// The value can be "off", "assertive", or "polite", helping assistive technologies
    /// determine how to handle updates to the content.
    #[props(default)]
    pub aria_live: AriaLive,

    /// Indicates whether the image is currently pressed or selected.
    ///
    /// This attribute can have values like "true", "false", "mixed", or "undefined".
    #[props(default)]
    pub aria_pressed: AriaPressed,

    /// ID of the element that the image controls or owns.
    ///
    /// Specifies the ID of the element that the image controls or is associated with.
    #[props(default = "")]
    pub aria_controls: &'static str,

    /// ID of the element that labels the image.
    ///
    /// Specifies the ID of the element that labels the image for accessibility purposes.
    #[props(default = "")]
    pub aria_labelledby: &'static str,
}

impl Default for ImageProps {
    fn default() -> Self {
        ImageProps {
            src: "",
            alt: "Image",
            width: "",
            height: "",
            style: "",
            class: "",
            sizes: "",
            quality: "",
            placeholder: "empty",
            on_load: Callback::default(),
            object_fit: ObjectFit::default(),
            object_position: Position::default(),
            on_error: Callback::default(),
            decoding: Decoding::default(),
            blur_data_url: "",
            lazy_boundary: "100px",
            unoptimized: false,
            layout: Layout::default(),
            fallback_src: "",
            srcset: "",
            crossorigin: CrossOrigin::default(),
            loading: Loading::default(),
            referrerpolicy: ReferrerPolicy::default(),
            usemap: "",
            ismap: false,
            fetchpriority: FetchPriority::default(),
            elementtiming: "",
            attributionsrc: "",
            aria_current: "",
            aria_describedby: "",
            aria_expanded: "",
            aria_hidden: "",
            aria_live: AriaLive::default(),
            aria_pressed: AriaPressed::default(),
            aria_controls: "",
            aria_labelledby: "",
        }
    }
}

#[component]
pub fn Image(props: ImageProps) -> Element {
    // TODO: Figure out how to create a node in dioxus
    let node_ref = Some(5);
    let mut src = use_signal(|| props.src);
    let on_load = props.on_load;
    let on_error_callback = props.on_error;

    // Intersection Observer effect
    use_effect(move || {
        // TODO: el.cast::<HtmlImageElement>()
        let node = node_ref.as_ref();
        if let Some(_img) = node {
            let closure = Closure::wrap(Box::new(
                move |entries: js_sys::Array, _: IntersectionObserver| {
                    if let Some(entry) = entries.get(0).dyn_ref::<IntersectionObserverEntry>() {
                        if entry.is_intersecting() {
                            // img.set_src(props.src);
                            on_load.call(());
                        }
                    }
                },
            )
                as Box<dyn FnMut(js_sys::Array, IntersectionObserver)>);

            let options = IntersectionObserverInit::new();
            options.set_threshold(&js_sys::Array::of1(&0.1.into()));
            options.set_root_margin(props.lazy_boundary);

            if let Ok(observer) =
                IntersectionObserver::new_with_options(closure.as_ref().unchecked_ref(), &options)
            {
                // observer.observe(&img);
                closure.forget();
                {
                    observer.disconnect();
                }
            }
        }
    });

    // On error handler
    let on_error = move |_| {
        let fallback_src = props.fallback_src;

        if fallback_src.is_empty() {
            on_error_callback.call("Image failed to load and no fallback provided.".to_string());
            return;
        }

        spawn(async move {
            match Request::get(fallback_src).send().await {
                Ok(resp) if resp.ok() => {
                    src.set(fallback_src);
                    on_load.call(());
                }
                Ok(resp) => {
                    let status = resp.status();
                    let body = resp.text().await.unwrap_or_default();
                    on_error_callback.call(format!(
                        "Fallback image load failed: status {}, body {}",
                        status, body
                    ));
                }
                Err(e) => {
                    on_error_callback.call(format!("Network error while loading fallback: {}", e));
                }
            }
        });
    };

    let img_style = format!(
        "object-fit: {:?}; object-position: {:?}; {};",
        props.object_fit, props.object_position, props.style
    );

    let blur_style = if props.placeholder == "blur" {
        format!(
            "background-size: {}; background-position: {:?}; filter: blur(20px); background-image: url('{}');",
            props.sizes, props.object_position, props.blur_data_url
        )
    } else {
        "".to_string()
    };

    let full_style = format!("{img_style} {blur_style}");

    let onload = move |_| {
        props.on_load.call(());
    };

    let img_element = rsx! {
        img {
            src: "{src()}",
            alt: "{props.alt}",
            width: "{props.width}",
            height: "{props.height}",
            class: "{props.class}",
            // TODO: Till Dioxus support this attribute
            // sizes: "{props.sizes}",
            // decoding: "{props.decoding}",
            // TODO:
            // loading: "{props.loading}",
            // TODO
            // node_ref: node_ref,
            style: "{full_style}",
            onerror: on_error,
            aria_current: "{props.aria_current}",
            aria_describedby: "{props.aria_describedby}",
            aria_expanded: "{props.aria_expanded}",
            aria_hidden: "{props.aria_hidden}",
            aria_live: "{props.aria_live.as_str()}",
            aria_pressed: "{props.aria_pressed.as_str()}",
            aria_controls: "{props.aria_controls}",
            aria_labelledby: "{props.aria_labelledby}",
            role: "img",
            style: "{blur_style}",
            crossorigin: props.crossorigin.as_str(),
            referrerpolicy: props.referrerpolicy.as_str(),
            // TODO
            // fetchpriority: "{props.fetchpriority.as_str()}",
            // TODO
            // attributionsrc: "{props.attributionsrc}",
            onload: onload,
            // TODO
            // elementtiming: "{props.elementtiming}",
            srcset: "{props.srcset}",
            ismap: "{props.ismap}",
            usemap: "{props.usemap}"
        }
    };

    match props.layout {
        Layout::Fill => rsx! {
            span {
                style: "display: block; position: absolute; top: 0; left: 0; bottom: 0; right: 0;",
                {img_element},
            }
        },
        Layout::Responsive => {
            let quotient = props.height.parse::<f64>().unwrap_or(1.0)
                / props.width.parse::<f64>().unwrap_or(1.0);
            let padding_top = if quotient.is_finite() {
                format!("{}%", quotient * 100.0)
            } else {
                "100%".to_string()
            };
            rsx! {
                span {
                    style: "display: block; position: relative;",
                    span {
                        style: "padding-top: {padding_top};",
                    },
                    {img_element},
                },
            }
        }
        Layout::Intrinsic => rsx! {
            span {
                style: "display: inline-block; position: relative; max-width: 100%;",
                span {
                    style: "max-width: 100%;",
                    {img_element},
                },
                img {
                    src: "{props.blur_data_url}",
                    style: "display: none;",
                    alt: "{props.alt}",
                    aria_hidden: "true",
                },
            }
        },
        Layout::Fixed => rsx! {
            span {
                style: "display: inline-block; position: relative;",
                {img_element},
            }
        },
        Layout::Auto => rsx! {
            span {
                style: "display: inline-block; position: relative;",
                {img_element},
            }
        },
        Layout::Stretch => rsx! {
            span {
                style: "display: block; width: 100%; height: 100%; position: relative;",
                {img_element},
            }
        },
        Layout::ScaleDown => rsx! {
            span {
                style: "display: inline-block; position: relative; max-width: 100%; max-height: 100%;",
                {img_element},
            }
        },
    }
}
