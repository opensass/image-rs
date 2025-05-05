#![doc = include_str!("../LEPTOS.md")]

use crate::common::{
    CrossOrigin, Decoding, FetchPriority, Layout, Loading, ObjectFit, Position, ReferrerPolicy,
};
use gloo_net::http::Request;
use leptos::callback::Callback;
use leptos::task::spawn_local;
use leptos::{html::*, prelude::*, *};
use web_sys::IntersectionObserverEntry;
use web_sys::js_sys;
use web_sys::wasm_bindgen::JsCast;
use web_sys::wasm_bindgen::closure::Closure;
use web_sys::{IntersectionObserver, IntersectionObserverInit, RequestCache};

// Comment out aria attrs cause of: tachys-0.2.0/src/html/attribute/mod.rs:593:1:
// not yet implemented: adding more than 26 attributes is not supported
#[component]
pub fn Image(
    /// The source URL of the image.
    ///
    /// This is the primary image that will be rendered.
    #[prop(optional)]
    src: &'static str,

    /// The alternative text for the image.
    ///
    /// Used for accessibility and shown if the image cannot be displayed.
    #[prop(optional, default = "Image")]
    alt: &'static str,

    /// A fallback image URL if the main image fails to load.
    #[prop(optional)]
    fallback_src: &'static str,

    /// Width of the image (e.g., "100px", "auto").
    #[prop(optional)]
    width: &'static str,

    /// Height of the image (e.g., "100px", "auto").
    #[prop(optional)]
    height: &'static str,

    /// Inline styles applied to the image.
    #[prop(optional)]
    style: &'static str,

    /// CSS class name(s) to apply to the image.
    #[prop(optional)]
    class: &'static str,

    /// Image `sizes` attribute for responsive loading.
    #[prop(optional)]
    sizes: &'static str,

    // #[prop(optional)] quality: &'static str,
    /// Defines how the image is loaded. Defaults to lazy loading.
    #[prop(optional, default = Loading::Lazy)]
    loading: Loading,

    /// Placeholder content shown while the image loads.
    #[prop(optional, default = "empty")]
    placeholder: &'static str,

    /// Callback function fired when the image is successfully loaded.
    #[prop(optional)]
    on_load: Option<Callback<()>>,

    /// Specifies how the image should be resized to fit its container.
    #[prop(optional, default = ObjectFit::Fill)]
    object_fit: ObjectFit,

    /// Specifies the position of the image within its container.
    #[prop(optional, default = Position::Center)]
    object_position: Position,

    /// Callback function fired when the image fails to load.
    #[prop(optional)]
    on_error: Option<Callback<String>>,

    /// Specifies how the image should be decoded (auto, sync, async).
    #[prop(optional, default = Decoding::Auto)]
    decoding: Decoding,

    /// Base64-encoded blurred image shown before the main image loads.
    #[prop(optional)]
    blur_data_url: &'static str,

    // #[prop(optional, default = "100px")] lazy_boundary: &'static str,
    // #[prop(optional, default = false)] unoptimized: bool,
    /// Controls how the image is laid out inside its container.
    #[prop(optional, default = Layout::Responsive)]
    layout: Layout,

    /// Reference to the image DOM element.
    #[prop(optional)]
    node_ref: NodeRef<Img>,

    /// One or more image sources with descriptors (e.g., "img-1x.jpg 1x, img-2x.jpg 2x").
    #[prop(optional)]
    srcset: &'static str,

    /// CORS policy for fetching the image (none, anonymous, use-credentials).
    #[prop(optional, default = CrossOrigin::None)]
    crossorigin: CrossOrigin,

    /// Referrer policy when fetching the image.
    #[prop(optional, default = ReferrerPolicy::NoReferrer)]
    referrerpolicy: ReferrerPolicy,

    /// Associates the image with an image map.
    #[prop(optional)]
    usemap: &'static str,

    /// Indicates the image is part of a server-side image map.
    #[prop(optional, default = false)]
    ismap: bool,

    /// Fetch priority hint for the browser (auto, high, low).
    #[prop(optional, default = FetchPriority::Auto)]
    fetchpriority: FetchPriority,

    /// Identifier for performance element timing.
    #[prop(optional)]
    elementtiming: &'static str,
    /// Indicates the current item in a set for accessibility.
    // #[prop(optional)] aria_current: &'static str,
    /// ID reference to the element describing this image.
    // #[prop(optional)] aria_describedby: &'static str,
    /// Whether the associated content is expanded or collapsed.
    // #[prop(optional)] aria_expanded: &'static str,
    /// Whether the image is hidden from assistive technologies.
    /// #[prop(optional)] aria_hidden: &'static str,
    /// Indicates the pressed state of the image if it's used as a toggle.
    // #[prop(optional, default = AriaPressed::Undefined)] aria_pressed: AriaPressed,
    /// ID reference to the element this image controls.
    // #[prop(optional)] aria_controls: &'static str,
    /// ID reference to the element that labels this image.
    // #[prop(optional)] aria_labelledby: &'static str,
    /// Indicates whether updates to the image are live.
    // #[prop(optional, default = AriaLive::Off)] aria_live: AriaLive,
    /// URLs for Attribution Reporting (experimental feature).
    #[prop(optional)]
    attributionsrc: &'static str,
) -> impl IntoView {
    let (img_src, set_img_src) = signal(src);

    Effect::new(move || {
        let callback = Closure::wrap(Box::new(
            move |entries: js_sys::Array, _observer: IntersectionObserver| {
                if let Some(entry) = entries.get(0).dyn_ref::<IntersectionObserverEntry>() {
                    if entry.is_intersecting() {
                        if let Some(node) = node_ref.get() {
                            if let Some(img) = node.dyn_ref::<web_sys::HtmlImageElement>() {
                                img.set_src(src);
                                if let Some(cb) = on_load {
                                    cb.run(());
                                }
                            }
                        }
                    }
                }
            },
        )
            as Box<dyn FnMut(js_sys::Array, IntersectionObserver)>);

        let options = IntersectionObserverInit::new();
        options.set_threshold(&js_sys::Array::of1(&0.1.into()));

        let observer =
            IntersectionObserver::new_with_options(callback.as_ref().unchecked_ref(), &options)
                .expect("Failed to create IntersectionObserver");

        if let Some(element) = node_ref.get() {
            if let Ok(img) = element.clone().dyn_into::<web_sys::HtmlElement>() {
                observer.observe(&img);
            }
        }

        let observer_clone = observer.clone();
        let _cleanup = move || {
            observer_clone.disconnect();
        };

        callback.forget();
    });

    let onload = move |_| {
        if let Some(cb) = on_load {
            cb.run(());
        }
    };

    let onerror = {
        move |_| {
            spawn_local(async move {
                match Request::get(fallback_src)
                    .cache(RequestCache::Reload)
                    .send()
                    .await
                {
                    Ok(res) if res.status() == 200 => match res.json::<serde_json::Value>().await {
                        Ok(_) => {
                            set_img_src.set(fallback_src);
                            if let Some(cb) = on_load {
                                cb.run(());
                            }
                        }
                        Err(_) => {
                            if let Some(cb) = on_error {
                                cb.run("Image not found!".to_string());
                            }
                        }
                    },
                    Ok(res) => {
                        let body = res.text().await.unwrap_or_default();
                        if let Some(cb) = on_error {
                            cb.run(format!(
                                "Failed to load image. Status: {}, Body: {}",
                                res.status(),
                                body
                            ));
                        }
                    }
                    Err(e) => {
                        if let Some(cb) = on_error {
                            cb.run(format!("Network error: {e}"));
                        }
                    }
                }
            });
        }
    };

    let img_style = format!(
        "{} object-fit: {}; object-position: {};",
        style,
        object_fit.as_str(),
        object_position.as_str()
    );

    let blur_style = if placeholder == "blur" && !blur_data_url.is_empty() {
        format!(
            "background-size: {}; background-position: {}; filter: blur(20px); background-image: url('{}');",
            sizes,
            object_position.as_str(),
            blur_data_url
        )
    } else {
        "".into()
    };

    let full_style = format!("{blur_style} {img_style}");

    let layout_view = match layout {
        Layout::Fill => view! {
            <span style="display:block; position:absolute; top:0; left:0; right:0; bottom:0;">
                <img
                    node_ref=node_ref
                    src=move || img_src.get()
                    alt=alt
                    class=class
                    width=width
                    height=height
                    style=full_style.clone()
                    sizes=sizes
                    srcset=srcset
                    decoding=decoding.as_str()
                    crossorigin=crossorigin.as_str()
                    referrerpolicy=referrerpolicy.as_str()
                    loading=loading.as_str()
                    fetchpriority=fetchpriority.as_str()
                    aria_placeholder=placeholder
                    on:load=onload
                    on:error=onerror
                    role="img"
                    // aria-label=alt
                    // aria-labelledby=aria_labelledby
                    // aria-describedby=aria_describedby
                    // aria-hidden=aria_hidden
                    // aria-current=aria_current
                    // aria-expanded=aria_expanded
                    // aria-live=aria_live.as_str()
                    // aria-pressed=aria_pressed.as_str()
                    // aria-controls=aria_controls
                    usemap=usemap
                    ismap=ismap
                    elementtiming=elementtiming
                    attributionsrc=attributionsrc
                />
            </span>
        }
        .into_any(),

        Layout::Responsive => {
            let ratio = height.parse::<f64>().unwrap_or(1.0) / width.parse::<f64>().unwrap_or(1.0);
            let padding = format!("{}%", ratio * 100.0);
            view! {
                <span style="display:block; position:relative;">
                    <span style=format!("padding-top: {padding}")>
                        <img
                            node_ref=node_ref
                            src=move || img_src.get()
                            alt=alt
                            class=class
                            width=width
                            height=height
                            style=full_style.clone()
                            sizes=sizes
                            srcset=srcset
                            decoding=decoding.as_str()
                            crossorigin=crossorigin.as_str()
                            referrerpolicy=referrerpolicy.as_str()
                            loading=loading.as_str()
                            fetchpriority=fetchpriority.as_str()
                            aria_placeholder=placeholder
                            on:load=onload
                            on:error=onerror
                            role="img"
                            // aria-label=alt
                            // aria-labelledby=aria_labelledby
                            // aria-describedby=aria_describedby
                            // aria-hidden=aria_hidden
                            // aria-current=aria_current
                            // aria-expanded=aria_expanded
                            // aria-live=aria_live.as_str()
                            // aria-pressed=aria_pressed.as_str()
                            // aria-controls=aria_controls
                            usemap=usemap
                            ismap=ismap
                            elementtiming=elementtiming
                            attributionsrc=attributionsrc
                        />
                    </span>
                </span>
            }
            .into_any()
        }

        Layout::Intrinsic => view! {
            <span style="display:inline-block; position:relative; max-width:100%;">
                <span style="max-width:100%;">
                    <img
                        node_ref=node_ref
                        src=move || img_src.get()
                        alt=alt
                        class=class
                        width=width
                        height=height
                        style=full_style.clone()
                        sizes=sizes
                        srcset=srcset
                        decoding=decoding.as_str()
                        crossorigin=crossorigin.as_str()
                        referrerpolicy=referrerpolicy.as_str()
                        loading=loading.as_str()
                        fetchpriority=fetchpriority.as_str()
                        aria_placeholder=placeholder
                        on:load=onload
                        on:error=onerror
                        role="img"
                        // aria-label=alt
                        // aria-labelledby=aria_labelledby
                        // aria-describedby=aria_describedby
                        // aria-hidden=aria_hidden
                        // aria-current=aria_current
                        // aria-expanded=aria_expanded
                        // aria-live=aria_live.as_str()
                        // aria-pressed=aria_pressed.as_str()
                        // aria-controls=aria_controls
                        usemap=usemap
                        ismap=ismap
                        elementtiming=elementtiming
                        attributionsrc=attributionsrc
                    />
                </span>
                <img
                    src=blur_data_url
                    style="display:none;"
                    alt=alt
                    aria-hidden="true"
                />
            </span>
        }
        .into_any(),

        Layout::Fixed => view! {
            <span style="display:inline-block; position:relative;">
                <img
                    node_ref=node_ref
                    src=move || img_src.get()
                    alt=alt
                    class=class
                    width=width
                    height=height
                    style=full_style.clone()
                    sizes=sizes
                    srcset=srcset
                    decoding=decoding.as_str()
                    crossorigin=crossorigin.as_str()
                    referrerpolicy=referrerpolicy.as_str()
                    loading=loading.as_str()
                    fetchpriority=fetchpriority.as_str()
                    aria_placeholder=placeholder
                    on:load=onload
                    on:error=onerror
                    role="img"
                    // aria-label=alt
                    // aria-labelledby=aria_labelledby
                    // aria-describedby=aria_describedby
                    // aria-hidden=aria_hidden
                    // aria-current=aria_current
                    // aria-expanded=aria_expanded
                    // aria-live=aria_live.as_str()
                    // aria-pressed=aria_pressed.as_str()
                    // aria-controls=aria_controls
                    usemap=usemap
                    ismap=ismap
                    elementtiming=elementtiming
                    attributionsrc=attributionsrc
                />
            </span>
        }
        .into_any(),

        Layout::Auto => view! {
            <span style="display:inline-block; position:relative;">
                <img
                    node_ref=node_ref
                    src=move || img_src.get()
                    alt=alt
                    class=class
                    width=width
                    height=height
                    style=full_style.clone()
                    sizes=sizes
                    srcset=srcset
                    decoding=decoding.as_str()
                    crossorigin=crossorigin.as_str()
                    referrerpolicy=referrerpolicy.as_str()
                    loading=loading.as_str()
                    fetchpriority=fetchpriority.as_str()
                    aria_placeholder=placeholder
                    on:load=onload
                    on:error=onerror
                    role="img"
                    // aria-label=alt
                    // aria-labelledby=aria_labelledby
                    // aria-describedby=aria_describedby
                    // aria-hidden=aria_hidden
                    // aria-current=aria_current
                    // aria-expanded=aria_expanded
                    // aria-live=aria_live.as_str()
                    // aria-pressed=aria_pressed.as_str()
                    // aria-controls=aria_controls
                    usemap=usemap
                    ismap=ismap
                    elementtiming=elementtiming
                    attributionsrc=attributionsrc
                />
            </span>
        }
        .into_any(),

        Layout::Stretch => view! {
            <span style="display:block; width:100%; height:100%; position:relative;">
                <img
                    node_ref=node_ref
                    src=move || img_src.get()
                    alt=alt
                    class=class
                    width="100%"
                    height="100%"
                    style=full_style.clone()
                    sizes=sizes
                    srcset=srcset
                    decoding=decoding.as_str()
                    crossorigin=crossorigin.as_str()
                    referrerpolicy=referrerpolicy.as_str()
                    loading=loading.as_str()
                    fetchpriority=fetchpriority.as_str()
                    aria_placeholder=placeholder
                    on:load=onload
                    on:error=onerror
                    role="img"
                    // aria-label=alt
                    // aria-labelledby=aria_labelledby
                    // aria-describedby=aria_describedby
                    // aria-hidden=aria_hidden
                    // aria-current=aria_current
                    // aria-expanded=aria_expanded
                    // aria-live=aria_live.as_str()
                    // aria-pressed=aria_pressed.as_str()
                    // aria-controls=aria_controls
                    usemap=usemap
                    ismap=ismap
                    elementtiming=elementtiming
                    attributionsrc=attributionsrc
                />
            </span>
        }
        .into_any(),

        Layout::ScaleDown => view! {
            <span style="display:inline-block; position:relative; max-width:100%; max-height:100%;">
                <img
                    node_ref=node_ref
                    src=move || img_src.get()
                    alt=alt
                    class=class
                    width=width
                    height=height
                    style=full_style.clone()
                    sizes=sizes
                    srcset=srcset
                    decoding=decoding.as_str()
                    crossorigin=crossorigin.as_str()
                    referrerpolicy=referrerpolicy.as_str()
                    loading=loading.as_str()
                    fetchpriority=fetchpriority.as_str()
                    aria_placeholder=placeholder
                    on:load=onload
                    on:error=onerror
                    role="img"
                    // aria-label=alt
                    // aria-labelledby=aria_labelledby
                    // aria-describedby=aria_describedby
                    // aria-hidden=aria_hidden
                    // aria-current=aria_current
                    // aria-expanded=aria_expanded
                    // aria-live=aria_live.as_str()
                    // aria-pressed=aria_pressed.as_str()
                    // aria-controls=aria_controls
                    usemap=usemap
                    ismap=ismap
                    elementtiming=elementtiming
                    attributionsrc=attributionsrc
                />
            </span>
        }
        .into_any(),
    };

    view! {
        {layout_view}
    }
}
