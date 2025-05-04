use dioxus::prelude::*;
use dioxus_logger::tracing;
use image_rs::dioxus::Image;
use image_rs::{AriaLive, AriaPressed, Decoding, Layout, Loading, ObjectFit, Position};

fn main() {
    dioxus_logger::init(tracing::Level::INFO).expect("failed to init logger");
    tracing::info!("starting app");
    launch(app);
}

#[component]
fn app() -> Element {
    rsx! {
        document::Stylesheet { href: "https://unpkg.com/tailwindcss@2.2.19/dist/tailwind.min.css" }
        document::Stylesheet { href: asset!("assets/main.css") }
        LandingPage {}
    }
}

#[component]
pub fn Example1() -> Element {
    rsx!(Image {
        src: "https://placehold.co/300x200",
        alt: "Basic Image"
    })
}

#[component]
pub fn Example2() -> Element {
    rsx!(Image {
        src: "https://placehold.co/300x200",
        alt: "Fixed Layout",
        layout: Layout::Fixed,
        width: "300",
        height: "200",
    })
}

#[component]
pub fn Example3() -> Element {
    rsx!(Image {
        src: "https://placehold.co/600x400",
        alt: "Responsive Layout",
        layout: Layout::Responsive,
        width: "600",
        height: "400",
    })
}

#[component]
pub fn Example4() -> Element {
    rsx!(Image {
        src: "https://placehold.co/600x400",
        alt: "Blurred Image",
        layout: Layout::Responsive,
        width: "600",
        height: "400",
        placeholder: "blur",
        blur_data_url: "https://placehold.co/10x10",
    })
}

#[component]
pub fn Example5() -> Element {
    rsx!(Image {
        src: "https://placehold.co/600x400",
        alt: "Cover Fit",
        layout: Layout::Responsive,
        width: "600",
        height: "400",
        object_fit: ObjectFit::Cover,
    })
}

#[component]
pub fn Example6() -> Element {
    let on_error = |_| {
        tracing::error!("Failed to load image");
    };

    rsx!(Image {
        src: "https://invalid.url",
        fallback_src: "https://jsonplaceholder.typicode.com/posts/1",
        alt: "Broken Image",
        on_error: on_error,
    })
}

#[component]
pub fn Example7() -> Element {
    rsx!(Image {
        src: "https://placehold.co/400x300",
        alt: "Priority Image",
        loading: Loading::Eager,
    })
}

#[component]
pub fn Example8() -> Element {
    rsx!(Image {
        src: "https://placehold.co/400x300",
        alt: "Async Decoding",
        decoding: Decoding::Async,
    })
}

#[component]
pub fn Example9() -> Element {
    rsx!(Image {
        src: "https://placehold.co/400x300",
        alt: "High Quality",
        quality: "90",
    })
}

#[component]
pub fn Example10() -> Element {
    rsx!(Image {
        src: "https://placehold.co/500x300",
        alt: "Accessible Image",
        aria_labelledby: "imageLabel",
        aria_describedby: "imageDescription",
        aria_hidden: "false",
        aria_live: AriaLive::Polite,
        aria_current: "page",
        aria_expanded: "false",
        aria_pressed: AriaPressed::False,
        aria_controls: "imageControl",
        decoding: Decoding::Sync,
        layout: Layout::Intrinsic,
        width: "500",
        height: "300",
    })
}

#[component]
pub fn Example11() -> Element {
    rsx!(Image {
        src: "https://placehold.co/400x300",
        alt: "Custom Style",
        style: "border-radius: 12px; border: 2px solid black;",
    })
}

#[component]
pub fn Example12() -> Element {
    rsx!(Image {
        src: "https://placehold.co/300x200",
        alt: "Custom Class",
        class: "rounded-lg shadow-lg",
    })
}

#[component]
pub fn Example13() -> Element {
    rsx!(Image {
        src: "https://placehold.co/500x300",
        alt: "Lazy Boundary Example",
        layout: Layout::Responsive,
        width: "500",
        height: "300",
        lazy_boundary: "500px",
    })
}

#[component]
pub fn Example14() -> Element {
    rsx!(Image {
        src: "https://placehold.co/300x300",
        alt: "Unoptimized Image",
        unoptimized: true,
    })
}

#[component]
pub fn Example15() -> Element {
    rsx!(Image {
        src: "https://placehold.co/600x400",
        alt: "Custom Object Position",
        object_fit: ObjectFit::Cover,
        object_position: Position::TopRight,
        layout: Layout::Responsive,
        width: "600",
        height: "400",
    })
}

#[component]
pub fn LandingPage() -> Element {
    rsx! {
        div {
            class: "m-6 min-h-screen flex flex-col items-center justify-center",
            h1 { class: "text-3xl font-bold mb-8 text-white", "Image RS Dioxus Examples" }
            div {
                class: "grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-8",

                div {
                    class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md",
                    h2 { class: "text-xl font-bold mb-2", "Basic Image" }
                    pre {
                        class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto",
                        r##"Image {{
    src: "https://placehold.co/300x200",
    alt: "Basic Image"
}}"##
                    }
                    Example1 {}
                }

                div {
                    class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md",
                    h2 { class: "text-xl font-bold mb-2", "Fixed Layout" }
                    pre {
                        class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto",
                        r##"Image {{
    src: "https://placehold.co/300x200",
    alt: "Fixed Layout",
    layout: Layout::Fixed,
    width: "300",
    height: "200"
}}"##
                    }
                    Example2 {}
                }

                div {
                    class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md",
                    h2 { class: "text-xl font-bold mb-2", "Responsive Layout" }
                    pre {
                        class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto",
                        r##"Image {{
    src: "https://placehold.co/600x400",
    alt: "Responsive Layout",
    layout: Layout::Responsive,
    width: "600",
    height: "400"
}}"##
                    }
                    Example3 {}
                }

                div {
                    class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md",
                    h2 { class: "text-xl font-bold mb-2", "Blur Placeholder" }
                    pre {
                        class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto",
                        r##"Image {{
    src: "https://placehold.co/600x400",
    alt: "Blurred Image",
    layout: Layout::Responsive,
    width: "600",
    height: "400",
    placeholder: "blur",
    blur_data_url: "https://placehold.co/10x10"
}}"##
                    }
                    Example4 {}
                }

                div {
                    class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md",
                    h2 { class: "text-xl font-bold mb-2", "Object Fit: Cover" }
                    pre {
                        class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto",
                        r##"Image {{
    src: "https://placehold.co/600x400",
    alt: "Cover Fit",
    layout: Layout::Responsive,
    width: "600",
    height: "400",
    object_fit: ObjectFit::Cover
}}"##
                    }
                    Example5 {}
                }

                div {
                    class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md",
                    h2 { class: "text-xl font-bold mb-2", "Error Handling (Press F12)" }
                    pre {
                        class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto",
                        r##"Image {{
    src: "https://invalid.url",
    fallback_src: "https://jsonplaceholder.typicode.com/posts/1",
    alt: "Broken Image",
    on_error: |_| tracing::error!("Failed to load image")
}}"##
                    }
                    Example6 {}
                }
                div {
                    class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md",
                    h2 { class: "text-xl font-bold mb-2", "Priority Image" }
                    pre {
                        class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto",
                        r##"Image {{
    src: "https://placehold.co/400x300",
    alt: "Priority Image",
    loading: Loading::Eager
}}"##
                    }
                    Example7 {}
                }

                div {
                    class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md",
                    h2 { class: "text-xl font-bold mb-2", "Async Decoding" }
                    pre {
                        class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto",
                        r##"Image {{
    src: "https://placehold.co/400x300",
    alt: "Async Decoding",
    decoding: Decoding::Async
}}"##
                    }
                    Example8 {}
                }

                div {
                    class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md",
                    h2 { class: "text-xl font-bold mb-2", "High Quality" }
                    pre {
                        class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto",
                        r##"Image {{
    src: "https://placehold.co/400x300",
    alt: "High Quality",
    quality: "90"
}}"##
                    }
                    Example9 {}
                }

                div {
                    class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md",
                    h2 { class: "text-xl font-bold mb-2", "Accessible Image" }
                    pre {
                        class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto",
                        r##"Image {{
    src: "https://placehold.co/500x300",
    alt: "Accessible Image",
    aria_labelledby: "imageLabel",
    aria_describedby: "imageDescription",
    aria_hidden: "false",
    aria_live: AriaLive::Polite,
    aria_current: "page",
    aria_expanded: "false",
    aria_pressed: AriaPressed::False,
    aria_controls: "imageControl",
    decoding: Decoding::Sync,
    layout: Layout::Intrinsic,
    width: "500",
    height: "300"
}}"##
                    }
                    Example10 {}
                }

                div {
                    class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md",
                    h2 { class: "text-xl font-bold mb-2", "Custom Style" }
                    pre {
                        class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto",
                        r##"Image {{
    src: "https://placehold.co/400x300",
    alt: "Custom Style",
    style: "border-radius: 12px; border: 2px solid black;"
}}"##
                    }
                    Example11 {}
                }

                div {
                    class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md",
                    h2 { class: "text-xl font-bold mb-2", "Custom Class" }
                    pre {
                        class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto",
                        r##"Image {{
    src: "https://placehold.co/300x200",
    alt: "Custom Class",
    class: "rounded-lg shadow-lg"
}}"##
                    }
                    Example12 {}
                }

                div {
                    class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md",
                    h2 { class: "text-xl font-bold mb-2", "Lazy Boundary Example" }
                    pre {
                        class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto",
                        r##"Image {{
    src: "https://placehold.co/500x300",
    alt: "Lazy Boundary Example",
    layout: Layout::Responsive,
    width: "500",
    height: "300",
    lazy_boundary: "500px"
}}"##
                    }
                    Example13 {}
                }

                div {
                    class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md",
                    h2 { class: "text-xl font-bold mb-2", "Unoptimized Image" }
                    pre {
                        class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto",
                        r##"Image {{
    src: "https://placehold.co/300x300",
    alt: "Unoptimized Image",
    unoptimized: true
}}"##
                    }
                    Example14 {}
                }

                div {
                    class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md",
                    h2 { class: "text-xl font-bold mb-2", "Custom Object Position" }
                    pre {
                        class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto",
                        r##"Image {{
    src: "https://placehold.co/600x400",
    alt: "Custom Object Position",
    object_fit: ObjectFit::Cover,
    object_position: Position::TopRight,
    layout: Layout::Responsive,
    width: "600",
    height: "400"
}}"##
                    }
                    Example15 {}
                }
            }
        }
    }
}
