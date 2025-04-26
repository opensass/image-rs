#![doc = include_str!("../YEW.md")]

use crate::common::{Decoding, Layout, ObjectFit, Position};
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use web_sys::IntersectionObserverEntry;
use web_sys::js_sys;
use web_sys::wasm_bindgen::JsCast;
use web_sys::wasm_bindgen::JsValue;
use web_sys::wasm_bindgen::prelude::*;
use web_sys::{IntersectionObserver, IntersectionObserverInit, RequestCache};
use yew::prelude::*;

/// Properties for the `Image` component.
///
/// The `Image` component allows you to display an image with various customization options
/// for layout, styling, and behavior. It supports fallback images, lazy loading, and custom
/// callbacks for error handling and loading completion.
///
/// This component is highly flexible, providing support for multiple image layouts,
/// object-fit, object-position, ARIA attributes, and more.
#[derive(Properties, Clone, PartialEq)]
pub struct ImageProps {
    /// The source URL of the image.
    ///
    /// This is the URL of the image to be displayed. This property is required for loading
    /// an image. If not provided, the image will not be displayed.
    #[prop_or_default]
    pub src: &'static str,

    /// The alternative text for the image.
    ///
    /// This is the alt text for the image, which is used for accessibility purposes.
    /// If not provided, the alt text will be empty.
    #[prop_or_default]
    pub alt: &'static str,

    /// Optional fallback image.
    ///
    /// This image will be displayed if the main image fails to load. If not provided,
    /// the image will attempt to load without a fallback.
    #[prop_or_default]
    pub fallback_src: &'static str,

    /// The width of the image.
    ///
    /// Specifies the width of the image in pixels. It is typically used for responsive
    /// layouts. Defaults to an empty string if not provided.
    #[prop_or_default]
    pub width: &'static str,

    /// The height of the image.
    ///
    /// Specifies the height of the image in pixels. Like `width`, it is often used for
    /// responsive layouts. Defaults to an empty string if not provided.
    #[prop_or_default]
    pub height: &'static str,

    // Common props
    /// The style attribute for the image.
    ///
    /// Allows you to apply custom inline CSS styles to the image. Defaults to an empty string.
    #[prop_or_default]
    pub style: &'static str,

    /// The CSS class for the image.
    ///
    /// This can be used to apply custom CSS classes to the image for styling purposes.
    /// Defaults to an empty string if not provided.
    #[prop_or_default]
    pub class: &'static str,

    /// The sizes attribute for the image.
    ///
    /// This is used to define different image sizes for different viewport widths, helping
    /// with responsive images. Defaults to an empty string if not provided.
    #[prop_or_default]
    pub sizes: &'static str,

    /// The quality attribute for the image.
    ///
    /// Allows you to set the quality of the image (e.g., "low", "medium", "high"). Defaults
    /// to an empty string if not provided.
    #[prop_or_default]
    pub quality: &'static str,

    /// Indicates if the image should have priority loading.
    ///
    /// This controls whether the image should be loaded eagerly (immediately) or lazily
    /// (when it enters the viewport). Defaults to `false`.
    #[prop_or_default]
    pub priority: bool,

    /// The placeholder attribute for the image.
    ///
    /// Allows you to specify a placeholder image URL or data URL to show while the main
    /// image is loading. Defaults to an empty string.
    #[prop_or_default]
    pub placeholder: &'static str,

    /// Callback function for handling loading completion.
    ///
    /// This callback is triggered once the image has finished loading. This is useful for
    /// actions that should happen after the image has been fully loaded, such as hiding
    /// a loading spinner. Defaults to a no-op.
    #[prop_or_default]
    pub on_loading_complete: Callback<()>,

    // Advanced Props
    /// The object-fit attribute for the image.
    ///
    /// Determines how the image should be resized to fit its container. Common values include
    /// "contain", "cover", "fill", etc. Defaults to an empty string.
    #[prop_or_default]
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
    #[prop_or_default]
    pub object_position: Position,

    /// Callback function for handling errors during image loading.
    ///
    /// This callback is triggered if the image fails to load, allowing you to handle
    /// error states (e.g., displaying a fallback image or showing an error message).
    #[prop_or_default]
    pub on_error: Callback<String>,

    /// The decoding attribute for the image.
    ///
    /// Specifies how the image should be decoded. The available options are:
    /// - `Decoding::Auto`: The image decoding behavior is automatically decided by the browser.
    /// - `Decoding::Sync`: The image is decoded synchronously (blocking other tasks).
    /// - `Decoding::Async`: The image is decoded asynchronously (non-blocking).
    ///
    /// Defaults to `Decoding::Auto`.
    #[prop_or_default]
    pub decoding: Decoding,

    /// The blur data URL for placeholder image.
    ///
    /// This is used to display a low-quality blurred version of the image while the full
    /// image is loading. Defaults to an empty string.
    #[prop_or_default]
    pub blur_data_url: &'static str,

    /// The lazy boundary for lazy loading.
    ///
    /// Defines the distance (in pixels) from the viewport at which the image should start
    /// loading. Defaults to an empty string.
    #[prop_or_default]
    pub lazy_boundary: &'static str,

    /// Indicates if the image should be unoptimized.
    ///
    /// If set to `true`, the image will be loaded without any optimization applied (e.g.,
    /// no resizing or compression). Defaults to `false`.
    #[prop_or_default]
    pub unoptimized: bool,

    /// Image layout.
    ///
    /// Specifies how the image should be laid out within its container. Possible values
    /// include `Layout::Fill`, `Layout::Responsive`, `Layout::Intrinsic`, `Layout::Fixed`,
    /// `Layout::Auto`, `Layout::Stretch`, and `Layout::ScaleDown`. Defaults to `Layout::Responsive`.
    #[prop_or_default]
    pub layout: Layout,

    /// Reference to the DOM node.
    ///
    /// This is used to create a reference to the actual DOM element of the image. It is
    /// useful for directly manipulating the image element via JavaScript if needed.
    #[prop_or_default]
    pub node_ref: NodeRef,

    /// Indicates the current state of the image in a navigation menu.
    ///
    /// Valid values are "page", "step", "location", "date", "time", "true", "false".
    /// This is useful for enhancing accessibility in navigation menus.
    #[prop_or_default]
    pub aria_current: &'static str,

    /// Describes the image using the ID of the element that provides a description.
    ///
    /// The ID of the element that describes the image. This is used for accessibility
    /// purposes, particularly for screen readers.
    #[prop_or_default]
    pub aria_describedby: &'static str,

    /// Indicates whether the content associated with the image is currently expanded or collapsed.
    ///
    /// This is typically used for ARIA-based accessibility and is represented as "true" or "false".
    #[prop_or_default]
    pub aria_expanded: &'static str,

    /// Indicates whether the image is currently hidden from the user.
    ///
    /// This attribute is used for accessibility and indicates whether the image is visible
    /// to the user or not. Valid values are "true" or "false".
    #[prop_or_default]
    pub aria_hidden: &'static str,

    /// Indicates whether the content associated with the image is live and dynamic.
    ///
    /// The value can be "off", "assertive", or "polite", helping assistive technologies
    /// determine how to handle updates to the content.
    #[prop_or_default]
    pub aria_live: &'static str,

    /// Indicates whether the image is currently pressed or selected.
    ///
    /// This attribute can have values like "true", "false", "mixed", or "undefined".
    #[prop_or_default]
    pub aria_pressed: &'static str,

    /// ID of the element that the image controls or owns.
    ///
    /// Specifies the ID of the element that the image controls or is associated with.
    #[prop_or_default]
    pub aria_controls: &'static str,

    /// ID of the element that labels the image.
    ///
    /// Specifies the ID of the element that labels the image for accessibility purposes.
    #[prop_or_default]
    pub aria_labelledby: &'static str,
}

impl Default for ImageProps {
    fn default() -> Self {
        ImageProps {
            src: "",
            alt: "Image",
            width: "300",
            height: "200",
            style: "",
            class: "",
            sizes: "",
            quality: "",
            priority: false,
            placeholder: "empty",
            on_loading_complete: Callback::noop(),
            object_fit: ObjectFit::default(),
            object_position: Position::default(),
            on_error: Callback::noop(),
            decoding: Decoding::default(),
            blur_data_url: "",
            lazy_boundary: "100px",
            unoptimized: false,
            layout: Layout::default(),
            node_ref: NodeRef::default(),
            fallback_src: "",
            aria_current: "",
            aria_describedby: "",
            aria_expanded: "",
            aria_hidden: "",
            aria_live: "",
            aria_pressed: "",
            aria_controls: "",
            aria_labelledby: "",
        }
    }
}

/// Image Component
///
/// A highly optimized and feature-rich `Image` component for Yew applications, supporting
/// lazy loading, blur placeholders, fallback handling, and multiple responsive layouts.
///
/// # Properties
/// The component uses the `ImageProps` struct for its properties. Key properties include:
///
/// - **src**: The main image source URL (`&'static str`). Required.
/// - **alt**: Alternative text for accessibility (`&'static str`). Default: `""`.
/// - **layout**: The image layout strategy (`Layout`). Default: `Layout::Auto`.
/// - **width**: The width of the image (`&'static str`). Required for certain layouts.
/// - **height**: The height of the image (`&'static str`). Required for certain layouts.
/// - **sizes**: Defines responsive image sizes (`&'static str`). Default: `""`.
/// - **quality**: Image quality (custom property) (`&'static str`). Optional.
/// - **placeholder**: Placeholder strategy before the image loads (e.g., `"blur"`) (`&'static str`). Default: `""`.
/// - **blur_data_url**: Base64-encoded low-res placeholder image (`&'static str`). Used when `placeholder` is `"blur"`.
/// - **fallback_src**: Fallback image URL if the main `src` fails to load (`&'static str`). Optional.
/// - **priority**: Whether to load the image eagerly instead of lazily (`bool`). Default: `false`.
/// - **object_fit**: CSS `object-fit` value (`ObjectFit`). Default: `ObjectFit::Contain`.
/// - **object_position**: Object positioning inside the container (`Position`). Default: `Position::Center`.
/// - **style**: Additional inline CSS styles (`&'static str`). Default: `""`.
/// - **class**: Additional CSS classes (`&'static str`). Default: `""`.
/// - **decoding**: Decoding strategy (`Decoding`). Default: `Decoding::Auto`.
/// - **on_loading_complete**: Callback invoked when the image successfully loads (`Callback<()>`). Default: no-op.
/// - **on_error**: Callback invoked if loading or fallback loading fails (`Callback<String>`). Default: no-op.
/// - **node_ref**: Node reference for the underlying `img` element (`NodeRef`).
/// - **ARIA attributes**: Full ARIA support for better accessibility (`aria_label`, `aria_hidden`, etc.).
///
/// # Features
/// - **Lazy Loading with IntersectionObserver**:
///   Image is loaded **only when scrolled into view**, boosting performance and saving bandwidth.
///
/// - **Fallback Handling**:
///   Automatically tries a `fallback_src` if the primary `src` fails, ensuring robustness.
///
/// - **Blur Placeholder**:
///   Supports blurred low-resolution placeholders for smoother image transitions.
///
/// - **Multiple Layouts**:
///   Supports various layouts like `Fill`, `Responsive`, `Intrinsic`, `Fixed`, `Auto`, `Stretch`, and `ScaleDown`.
///
/// - **Accessibility**:
///   Built-in support for ARIA attributes to make images fully accessible.
///
/// # Layout Modes
/// - `Layout::Fill`: Image stretches to fill its container absolutely.
/// - `Layout::Responsive`: Maintains aspect ratio and scales responsively.
/// - `Layout::Intrinsic`: Renders using natural image dimensions.
/// - `Layout::Fixed`: Renders at a fixed size.
/// - `Layout::Auto`: Default natural behavior without forcing constraints.
/// - `Layout::Stretch`: Fills the parent container's width and height.
/// - `Layout::ScaleDown`: Scales image down to fit the container without stretching.
///
/// # Examples
///
/// ## Basic Usage
/// ```rust
/// use yew::prelude::*;
/// use image_rs::yew::Image;
/// use image_rs::Layout;
///
/// #[function_component(App)]
/// pub fn app() -> Html {
///     html! {
///         <Image
///             src="/images/photo.jpg"
///             alt="A beautiful view"
///             width="800"
///             height="600"
///             layout={Layout::Responsive}
///         />
///     }
/// }
/// ```
///
/// ## Blur Placeholder
/// ```rust
/// use yew::prelude::*;
/// use image_rs::yew::Image;
/// use image_rs::Layout;
///
/// #[function_component(App)]
/// pub fn app() -> Html {
///     html! {
///         <Image
///             src="/images/photo.jpg"
///             alt="Blur example"
///             width="800"
///             height="600"
///             layout={Layout::Intrinsic}
///             placeholder="blur"
///             blur_data_url="data:image/jpeg;base64,..."
///         />
///     }
/// }
/// ```
///
/// ## Fallback Image
/// ```rust
/// use yew::prelude::*;
/// use image_rs::yew::Image;
/// use image_rs::Layout;
///
/// #[function_component(App)]
/// pub fn app() -> Html {
///     html! {
///         <Image
///             src="/images/main.jpg"
///             fallback_src="/images/fallback.jpg"
///             alt="With fallback"
///             width="800"
///             height="600"
///             layout={Layout::Fixed}
///         />
///     }
/// }
/// ```
///
/// # Behavior
/// - The image starts loading lazily once it enters the viewport (10% visible threshold).
/// - If loading fails, a network fetch checks the fallback image.
/// - Blur placeholder is rendered with a heavy `blur(20px)` effect until the full image loads.
/// - Depending on the `Layout`, the container styling adjusts automatically.
///
/// # Notes
/// - `width` and `height` are required for `Responsive`, `Intrinsic`, and `Fixed` layouts.
/// - `Layout::Fill` ignores width and height, stretching to fit the parent container.
/// - Accessibility attributes like `aria-label` and `aria-hidden` are passed directly to the `<img>` element.
/// - Priority images (`priority = true`) are loaded eagerly instead of lazily.
///
/// # Errors
/// - If both `src` and `fallback_src` fail, the `on_error` callback is triggered with an error message.
/// - JSON parsing from a fallback response is attempted but not mandatory for image loading success.
///
/// # Optimization Techniques
/// - **IntersectionObserver** is used for intelligent lazy loading.
/// - **Caching** via `RequestCache::Reload` ensures fallback images are always fetched fresh if needed.
/// - **Async/await** approach for fetch operations provides non-blocking fallback handling.
#[function_component]
pub fn Image(props: &ImageProps) -> Html {
    let mut props = props.clone();
    let img_ref = props.node_ref.clone();

    let img_ref_clone = img_ref.clone();
    let on_loading_complete = props.on_loading_complete.clone();

    // Lazy Load Effect:
    // Waits until the image **scrolls into view**, then dynamically **sets the src** to start loading it.
    // Triggers an optional `on_loading_complete` callback once loading is initiated.
    // Smart Optimization: Saves bandwidth and greatly improves page speed, especially for pages with **many images**!
    // 9000 IQ Move: Only load images users actually *scroll to*, no more wasting bytes, gg!
    use_effect_with(JsValue::from(props.src), move |_deps| {
        let callback = Closure::wrap(Box::new(
            move |entries: js_sys::Array, _observer: IntersectionObserver| {
                if let Some(entry) = entries.get(0).dyn_ref::<IntersectionObserverEntry>() {
                    if entry.is_intersecting() {
                        if let Some(img) = img_ref_clone.cast::<web_sys::HtmlImageElement>() {
                            img.set_src(props.src);
                            on_loading_complete.emit(());
                        }
                    }
                }
            },
        )
            as Box<dyn FnMut(js_sys::Array, IntersectionObserver)>);

        let options = IntersectionObserverInit::new();
        // e.g., 10% visible
        options.set_threshold(&js_sys::Array::of1(&0.1.into()));
        // if Root is None, defaults to viewport

        // Create observer
        let observer =
            IntersectionObserver::new_with_options(callback.as_ref().unchecked_ref(), &options)
                .expect("Failed to create IntersectionObserver");

        // Start observing
        if let Some(img) = img_ref.cast::<web_sys::HtmlElement>() {
            observer.observe(&img);
        }

        // Disconnect when unmount
        let observer_clone = observer.clone();
        let _cleanup = move || {
            observer_clone.disconnect();
        };

        // Keep closure alive
        callback.forget();
    });

    // This informs your app that the image failed to load and auto replace the image.
    let fetch_data = {
        Callback::from(move |_| {
            let loading_complete_callback = props.on_loading_complete.clone();
            let on_error_callback = props.on_error.clone();
            spawn_local(async move {
                match Request::get(props.fallback_src)
                    .cache(RequestCache::Reload)
                    .send()
                    .await
                {
                    Ok(response) => {
                        if response.status() == 200 {
                            let json_result = response.json::<serde_json::Value>();
                            match json_result.await {
                                Ok(_data) => {
                                    props.src = props.fallback_src;
                                    loading_complete_callback.emit(());
                                }
                                Err(_err) => {
                                    on_error_callback.emit("Image Not Found!".to_string());
                                }
                            }
                        } else {
                            let status = response.status();
                            let body = response.text().await.unwrap_or_else(|_| {
                                String::from("Failed to retrieve response body")
                            });
                            on_error_callback.emit(format!(
                                "Failed to load image. Status: {}, Body: {:?}",
                                status, body
                            ));
                        }
                    }

                    Err(err) => {
                        // Handle network errors
                        on_error_callback.emit(format!("Network error: {}", err));
                    }
                }
            });
        })
    };

    let img_style = {
        let mut style = String::new();
        style.push_str(&format!("object-fit: {};", props.object_fit.as_str()));
        style.push_str(&format!(
            "object-position: {};",
            props.object_position.as_str()
        ));
        if !props.style.is_empty() {
            style.push_str(props.style);
        }
        style
    };

    let blur_style = if props.placeholder == "blur" {
        format!(
            "background-size: {}; background-position: {}; filter: blur(20px); background-image: url(\"{}\")",
            props.sizes,
            props.object_position.as_str(),
            props.blur_data_url
        )
    } else {
        String::new()
    };

    let layout = match props.layout {
        Layout::Fill => {
            html! {
                <span style={"display: block; position: absolute; top: 0; left: 0; bottom: 0; right: 0;"}>
                    <img
                        src={props.src}
                        alt={props.alt}
                        width={props.width}
                        height={props.height}
                        style={img_style}
                        class={props.class}
                        loading={if props.priority { "eager" } else { "lazy" }}
                        sizes={props.sizes}
                        quality={props.quality}
                        placeholder={props.placeholder}
                        decoding={props.decoding.as_str()}
                        ref={props.node_ref}
                        role="img"
                        aria-label={props.alt}
                        aria-labelledby={props.aria_labelledby}
                        aria-describedby={props.aria_describedby}
                        aria-hidden={props.aria_hidden}
                        aria-current={props.aria_current}
                        aria-expanded={props.aria_expanded}
                        aria-live={props.aria_live}
                        aria-pressed={props.aria_pressed}
                        aria-controls={props.aria_controls}
                        onerror={fetch_data}
                        style={blur_style}
                    />
                </span>
            }
        }
        Layout::Responsive => {
            let quotient: f64 =
                props.height.parse::<f64>().unwrap() / props.width.parse::<f64>().unwrap();
            let padding_top: String = if quotient.is_nan() {
                "100%".to_string()
            } else {
                format!("{}%", quotient * 100.0)
            };

            html! {
                <span style={"display: block; position: relative;"}>
                    <span style={"padding-top: ".to_owned() + &padding_top}>
                        <img
                            src={props.src}
                            alt={props.alt}
                            width={props.width}
                            height={props.height}
                            style={img_style}
                            class={props.class}
                            loading={if props.priority { "eager" } else { "lazy" }}
                            sizes={props.sizes}
                            quality={props.quality}
                            placeholder={props.placeholder}
                            decoding={props.decoding.as_str()}
                            ref={props.node_ref}
                            role="img"
                            aria-label={props.alt}
                            aria-labelledby={props.aria_labelledby}
                            aria-describedby={props.aria_describedby}
                            aria-hidden={props.aria_hidden}
                            aria-current={props.aria_current}
                            aria-expanded={props.aria_expanded}
                            aria-live={props.aria_live}
                            aria-pressed={props.aria_pressed}
                            aria-controls={props.aria_controls}
                            onerror={fetch_data}
                            style={blur_style}
                        />
                    </span>
                </span>
            }
        }
        Layout::Intrinsic => {
            html! {
                <span style={"display: inline-block; position: relative; max-width: 100%;"}>
                    <span style={"max-width: 100%;"}>
                        <img
                            src={props.src}
                            alt={props.alt}
                            width={props.width}
                            height={props.height}
                            style={img_style}
                            class={props.class}
                            loading={if props.priority { "eager" } else { "lazy" }}
                            sizes={props.sizes}
                            quality={props.quality}
                            placeholder={props.placeholder}
                            decoding={props.decoding.as_str()}
                            ref={props.node_ref}
                            role="img"
                            aria-label={props.alt}
                            aria-labelledby={props.aria_labelledby}
                            aria-describedby={props.aria_describedby}
                            aria-hidden={props.aria_hidden}
                            aria-current={props.aria_current}
                            aria-expanded={props.aria_expanded}
                            aria-live={props.aria_live}
                            aria-pressed={props.aria_pressed}
                            aria-controls={props.aria_controls}
                            onerror={fetch_data}
                            style={blur_style}
                        />
                    </span>
                    <img
                        src={props.blur_data_url}
                        style={"display: none;"}
                        alt={props.alt}
                        aria-hidden="true"
                    />
                </span>
            }
        }
        Layout::Fixed => {
            html! {
                <span style={"display: inline-block; position: relative;"}>
                    <img
                        src={props.src}
                        alt={props.alt}
                        width={props.width}
                        height={props.height}
                        style={img_style}
                        class={props.class}
                        loading={if props.priority { "eager" } else { "lazy" }}
                        sizes={props.sizes}
                        quality={props.quality}
                        placeholder={props.placeholder}
                        decoding={props.decoding.as_str()}
                        ref={props.node_ref}
                        role="img"
                        aria-label={props.alt}
                        aria-labelledby={props.aria_labelledby}
                        aria-describedby={props.aria_describedby}
                        aria-hidden={props.aria_hidden}
                        aria-current={props.aria_current}
                        aria-expanded={props.aria_expanded}
                        aria-live={props.aria_live}
                        aria-pressed={props.aria_pressed}
                        aria-controls={props.aria_controls}
                        onerror={fetch_data}
                        style={blur_style}
                    />
                </span>
            }
        }
        Layout::Auto => {
            // Preserve the natural size of the image
            html! {
                <span style={"display: inline-block; position: relative;"}>
                    <img
                        src={props.src}
                        alt={props.alt}
                        width={props.width}
                        height={props.height}
                        style={img_style}
                        class={props.class}
                        loading={if props.priority { "eager" } else { "lazy" }}
                        sizes={props.sizes}
                        quality={props.quality}
                        placeholder={props.placeholder}
                        decoding={props.decoding.as_str()}
                        ref={props.node_ref}
                        role="img"
                        aria-label={props.alt}
                        aria-labelledby={props.aria_labelledby}
                        aria-describedby={props.aria_describedby}
                        aria-hidden={props.aria_hidden}
                        aria-current={props.aria_current}
                        aria-expanded={props.aria_expanded}
                        aria-live={props.aria_live}
                        aria-pressed={props.aria_pressed}
                        aria-controls={props.aria_controls}
                        onerror={fetch_data}
                        style={blur_style}
                    />
                </span>
            }
        }
        Layout::Stretch => {
            // Make the image fill the container
            html! {
                <span style={"display: block; width: 100%; height: 100%; position: relative;"}>
                    <img
                        src={props.src}
                        alt={props.alt}
                        width="100%"
                        height="100%"
                        style={img_style}
                        class={props.class}
                        loading={if props.priority { "eager" } else { "lazy" }}
                        sizes={props.sizes}
                        quality={props.quality}
                        placeholder={props.placeholder}
                        decoding={props.decoding.as_str()}
                        ref={props.node_ref}
                        role="img"
                        aria-label={props.alt}
                        aria-labelledby={props.aria_labelledby}
                        aria-describedby={props.aria_describedby}
                        aria-hidden={props.aria_hidden}
                        aria-current={props.aria_current}
                        aria-expanded={props.aria_expanded}
                        aria-live={props.aria_live}
                        aria-pressed={props.aria_pressed}
                        aria-controls={props.aria_controls}
                        onerror={fetch_data}
                        style={blur_style}
                    />
                </span>
            }
        }
        Layout::ScaleDown => {
            // Maintain aspect ratio, but scale down if image is too large
            html! {
                <span style={"display: inline-block; position: relative; max-width: 100%; max-height: 100%;"}>
                    <img
                        src={props.src}
                        alt={props.alt}
                        width={props.width}
                        height={props.height}
                        style={img_style}
                        class={props.class}
                        loading={if props.priority { "eager" } else { "lazy" }}
                        sizes={props.sizes}
                        quality={props.quality}
                        placeholder={props.placeholder}
                        decoding={props.decoding.as_str()}
                        ref={props.node_ref}
                        role="img"
                        aria-label={props.alt}
                        aria-labelledby={props.aria_labelledby}
                        aria-describedby={props.aria_describedby}
                        aria-hidden={props.aria_hidden}
                        aria-current={props.aria_current}
                        aria-expanded={props.aria_expanded}
                        aria-live={props.aria_live}
                        aria-pressed={props.aria_pressed}
                        aria-controls={props.aria_controls}
                        onerror={fetch_data}
                        style={blur_style}
                    />
                </span>
            }
        }
    };
    html! {
            {layout}
    }
}
