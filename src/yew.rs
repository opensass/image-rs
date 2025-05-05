#![doc = include_str!("../YEW.md")]

use crate::common::{
    AriaLive, AriaPressed, CrossOrigin, Decoding, FetchPriority, Layout, Loading, ObjectFit,
    Position, ReferrerPolicy,
};
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
///
/// # See Also
/// - [MDN img Element](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/img)
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
    pub loading: Loading,

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
    pub on_load: Callback<()>,

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

    /// A list of one or more image sources for responsive loading.
    ///
    /// Defines multiple image resources for the browser to choose from, depending on screen size, resolution,
    /// and other factors. Each source can include width (`w`) or pixel density (`x`) descriptors.
    #[prop_or_default]
    pub srcset: &'static str,

    /// Cross-origin policy to use when fetching the image.
    ///
    /// Determines whether the image should be fetched with CORS enabled. Useful when the image needs to be accessed
    /// in a `<canvas>` element. Accepts `anonymous` or `use-credentials`.
    #[prop_or_default]
    pub crossorigin: CrossOrigin,

    /// Referrer policy to apply when fetching the image.
    ///
    /// Controls how much referrer information should be included with requests made for the image resource.
    /// Common values include `no-referrer`, `origin`, `strict-origin-when-cross-origin`, etc.
    #[prop_or_default]
    pub referrerpolicy: ReferrerPolicy,

    /// The fragment identifier of the image map to use.
    ///
    /// Associates the image with a `<map>` element, enabling clickable regions within the image. The value
    /// should begin with `#` and match the `name` of the corresponding map element.
    #[prop_or_default]
    pub usemap: &'static str,

    /// Indicates that the image is part of a server-side image map.
    ///
    /// When set, clicking the image will send the click coordinates to the server. Only allowed when the image
    /// is inside an `<a>` element with a valid `href`.
    #[prop_or_default]
    pub ismap: bool,

    /// Hints the browser about the priority of fetching this image.
    ///
    /// Helps the browser prioritize network resource loading. Accepts `high`, `low`, or `auto` (default).
    /// See `HTMLImageElement.fetchPriority` for more.
    #[prop_or_default]
    pub fetchpriority: FetchPriority,

    /// Identifier for tracking image performance timing.
    ///
    /// Registers the image with the `PerformanceElementTiming` API using the given string as its ID. Useful for
    /// performance monitoring and analytics.
    #[prop_or_default]
    pub elementtiming: &'static str,

    /// URL(s) to send Attribution Reporting requests for the image.
    ///
    /// Indicates that the browser should send an `Attribution-Reporting-Eligible` header with the image request.
    /// Can be a boolean or a list of URLs for attribution registration on specified servers. Experimental feature.
    #[prop_or_default]
    pub attributionsrc: &'static str,

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
    /// This is typically used for ARIA-based accessibility and is represented as "true", "false", or "undefined".
    #[prop_or_default]
    pub aria_expanded: &'static str,

    /// Indicates whether the image is currently hidden from the user.
    ///
    /// This attribute is used for accessibility and indicates whether the image is visible
    /// to the user or not. Valid values are "true", "false", or "undefined".
    #[prop_or_default]
    pub aria_hidden: &'static str,

    /// Indicates whether the content associated with the image is live and dynamic.
    ///
    /// The value can be "off", "assertive", or "polite", helping assistive technologies
    /// determine how to handle updates to the content.
    #[prop_or_default]
    pub aria_live: AriaLive,

    /// Indicates whether the image is currently pressed or selected.
    ///
    /// This attribute can have values like "true", "false", "mixed", or "undefined".
    #[prop_or_default]
    pub aria_pressed: AriaPressed,

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
            width: "",
            height: "",
            style: "",
            class: "",
            sizes: "",
            quality: "",
            placeholder: "empty",
            on_load: Callback::noop(),
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
/// - **on_load**: Callback invoked when the image successfully loads (`Callback<()>`). Default: no-op.
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
///
/// # See Also
/// - [MDN img Element](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/img)
#[function_component]
pub fn Image(props: &ImageProps) -> Html {
    let mut props = props.clone();
    let img_ref = props.node_ref.clone();

    let img_ref_clone = img_ref.clone();
    let on_load = props.on_load.clone();
    let on_load_call = props.on_load.clone();

    // Lazy Load Effect:
    // Waits until the image **scrolls into view**, then dynamically **sets the src** to start loading it.
    // Triggers an optional `on_load` callback once loading is initiated.
    // Smart Optimization: Saves bandwidth and greatly improves page speed, especially for pages with **many images**!
    // 9000 IQ Move: Only load images users actually *scroll to*, no more wasting bytes, gg!
    use_effect_with(JsValue::from(props.src), move |_deps| {
        let callback = Closure::wrap(Box::new(
            move |entries: js_sys::Array, _observer: IntersectionObserver| {
                if let Some(entry) = entries.get(0).dyn_ref::<IntersectionObserverEntry>() {
                    if entry.is_intersecting() {
                        if let Some(img) = img_ref_clone.cast::<web_sys::HtmlImageElement>() {
                            img.set_src(props.src);
                            on_load.emit(());
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
            let loading_complete_callback = props.on_load.clone();
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

    let onload = {
        Callback::from(move |_| {
            on_load_call.emit(());
        })
    };

    let full_style = format!("{} {}", blur_style, img_style);

    let layout = match props.layout {
        Layout::Fill => {
            html! {
                <span style={"display: block; position: absolute; top: 0; left: 0; bottom: 0; right: 0;"}>
                    <img
                        src={props.src}
                        alt={props.alt}
                        width={props.width}
                        height={props.height}
                        style={full_style}
                        class={props.class}
                        loading={props.loading.as_str()}
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
                        aria-live={props.aria_live.as_str()}
                        aria-pressed={props.aria_pressed.as_str()}
                        aria-controls={props.aria_controls}
                        onerror={fetch_data}
                        crossorigin={props.crossorigin.as_str()}
                        referrerpolicy={props.referrerpolicy.as_str()}
                        fetchpriority={props.fetchpriority.as_str()}
                        attributionsrc={props.attributionsrc}
                        onload={onload}
                        elementtiming={props.elementtiming}
                        srcset={props.srcset}
                        ismap={props.ismap}
                        usemap={props.usemap}
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
                            style={full_style}
                            class={props.class}
                            sizes={props.sizes}
                            quality={props.quality}
                            loading={props.loading.as_str()}
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
                            aria-live={props.aria_live.as_str()}
                            aria-pressed={props.aria_pressed.as_str()}
                            aria-controls={props.aria_controls}
                            onerror={fetch_data}
                            crossorigin={props.crossorigin.as_str()}
                            referrerpolicy={props.referrerpolicy.as_str()}
                            fetchpriority={props.fetchpriority.as_str()}
                            attributionsrc={props.attributionsrc}
                            onload={onload}
                            elementtiming={props.elementtiming}
                            srcset={props.srcset}
                            ismap={props.ismap}
                            usemap={props.usemap}
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
                            style={full_style}
                            class={props.class}
                            sizes={props.sizes}
                            loading={props.loading.as_str()}
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
                            aria-live={props.aria_live.as_str()}
                            aria-pressed={props.aria_pressed.as_str()}
                            aria-controls={props.aria_controls}
                            onerror={fetch_data}
                            crossorigin={props.crossorigin.as_str()}
                            referrerpolicy={props.referrerpolicy.as_str()}
                            fetchpriority={props.fetchpriority.as_str()}
                            attributionsrc={props.attributionsrc}
                            onload={onload}
                            elementtiming={props.elementtiming}
                            srcset={props.srcset}
                            ismap={props.ismap}
                            usemap={props.usemap}
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
                        style={full_style}
                        class={props.class}
                        sizes={props.sizes}
                        quality={props.quality}
                        loading={props.loading.as_str()}
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
                        aria-live={props.aria_live.as_str()}
                        aria-pressed={props.aria_pressed.as_str()}
                        aria-controls={props.aria_controls}
                        onerror={fetch_data}
                        crossorigin={props.crossorigin.as_str()}
                        referrerpolicy={props.referrerpolicy.as_str()}
                        fetchpriority={props.fetchpriority.as_str()}
                        attributionsrc={props.attributionsrc}
                        onload={onload}
                        elementtiming={props.elementtiming}
                        srcset={props.srcset}
                        ismap={props.ismap}
                        usemap={props.usemap}
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
                        style={full_style}
                        class={props.class}
                        sizes={props.sizes}
                        quality={props.quality}
                        placeholder={props.placeholder}
                        loading={props.loading.as_str()}
                        decoding={props.decoding.as_str()}
                        ref={props.node_ref}
                        role="img"
                        aria-label={props.alt}
                        aria-labelledby={props.aria_labelledby}
                        aria-describedby={props.aria_describedby}
                        aria-hidden={props.aria_hidden}
                        aria-current={props.aria_current}
                        aria-expanded={props.aria_expanded}
                        aria-live={props.aria_live.as_str()}
                        aria-pressed={props.aria_pressed.as_str()}
                        aria-controls={props.aria_controls}
                        onerror={fetch_data}
                        crossorigin={props.crossorigin.as_str()}
                        referrerpolicy={props.referrerpolicy.as_str()}
                        fetchpriority={props.fetchpriority.as_str()}
                        attributionsrc={props.attributionsrc}
                        onload={onload}
                        elementtiming={props.elementtiming}
                        srcset={props.srcset}
                        ismap={props.ismap}
                        usemap={props.usemap}
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
                        style={full_style}
                        class={props.class}
                        loading={props.loading.as_str()}
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
                        aria-live={props.aria_live.as_str()}
                        aria-pressed={props.aria_pressed.as_str()}
                        aria-controls={props.aria_controls}
                        onerror={fetch_data}
                        crossorigin={props.crossorigin.as_str()}
                        referrerpolicy={props.referrerpolicy.as_str()}
                        fetchpriority={props.fetchpriority.as_str()}
                        attributionsrc={props.attributionsrc}
                        onload={onload}
                        elementtiming={props.elementtiming}
                        srcset={props.srcset}
                        ismap={props.ismap}
                        usemap={props.usemap}
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
                        style={full_style}
                        class={props.class}
                        loading={props.loading.as_str()}
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
                        aria-live={props.aria_live.as_str()}
                        aria-pressed={props.aria_pressed.as_str()}
                        aria-controls={props.aria_controls}
                        onerror={fetch_data}
                        crossorigin={props.crossorigin.as_str()}
                        referrerpolicy={props.referrerpolicy.as_str()}
                        fetchpriority={props.fetchpriority.as_str()}
                        attributionsrc={props.attributionsrc}
                        onload={onload}
                        elementtiming={props.elementtiming}
                        srcset={props.srcset}
                        ismap={props.ismap}
                        usemap={props.usemap}
                    />
                </span>
            }
        }
    };
    html! {
            {layout}
    }
}
