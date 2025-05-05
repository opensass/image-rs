use image_rs::leptos::Image;
use image_rs::{Decoding, Layout, ObjectFit, Position, Loading};
use leptos::{*, prelude::*};
use leptos::logging::log;

#[component]
pub fn Example1() -> impl IntoView {
    view! { <Image src="https://placehold.co/300x200" alt="Basic Image" /> }
}

#[component]
pub fn Example2() -> impl IntoView {
    view! {
        <Image
            src="https://placehold.co/300x200"
            alt="Fixed Layout"
            layout={Layout::Fixed}
            width="300"
            height="200"
        />
    }
}

#[component]
pub fn Example3() -> impl IntoView {
    view! {
        <Image
            src="https://placehold.co/600x400"
            alt="Responsive Layout"
            layout={Layout::Responsive}
            width="600"
            height="400"
        />
    }
}

#[component]
pub fn Example4() -> impl IntoView {
    view! {
        <Image
            src="https://placehold.co/600x400"
            alt="Blurred Image"
            layout={Layout::Responsive}
            width="600"
            height="400"
            placeholder="blur"
            blur_data_url="https://placehold.co/10x10"
        />
    }
}

#[component]
pub fn Example5() -> impl IntoView {
    view! {
        <Image
            src="https://placehold.co/600x400"
            alt="Cover Fit"
            layout={Layout::Responsive}
            width="600"
            height="400"
            object_fit={ObjectFit::Cover}
        />
    }
}

#[component]
pub fn Example6() -> impl IntoView {
    view! {
        <Image
            src="https://invalid.url"
            fallback_src="https://jsonplaceholder.typicode.com/posts/1"
            alt="Broken Image"
            on_error={Callback::new(|e| log!("Failed to load: {}", e))}
        />
    }
}

#[component]
pub fn Example7() -> impl IntoView {
    view! { <Image src="https://placehold.co/400x300" alt="Priority Image" loading={Loading::Eager} /> }
}

#[component]
pub fn Example8() -> impl IntoView {
    view! {
        <Image src="https://placehold.co/400x300" alt="Async Decoding" decoding={Decoding::Async} />
    }
}

#[component]
pub fn Example9() -> impl IntoView {
    view! {
        <Image
            src="https://placehold.co/400x300"
            alt="High Quality"
            // //quality="90"
        />
    }
}
// #[component]
// pub fn Example10() -> impl IntoView {
//     view! {
//         <Image
//             src="https://placehold.co/500x300"
//             alt="Accessible Image"
//             aria_labelledby="imageLabel"
//             aria_describedby="imageDescription"
//             aria_hidden="false"
//             aria_live={AriaLive::Polite}
//             aria_current="page"
//             aria_expanded="false"
//             aria_pressed={AriaPressed::False}
//             aria_controls="imageControl"
//             decoding={Decoding::Sync}
//             layout={Layout::Intrinsic}
//             width="500"
//             height="300"
//         />
//     }
// }
#[component]
pub fn Example11() -> impl IntoView {
    view! {
        <Image
            src="https://placehold.co/400x300"
            alt="Custom Style"
            style="border-radius: 12px; border: 2px solid black;"
        />
    }
}
#[component]
pub fn Example12() -> impl IntoView {
    view! {
        <Image src="https://placehold.co/300x200" alt="Custom Class" class="rounded-lg shadow-lg" />
    }
}

#[component]
pub fn Example13() -> impl IntoView {
    view! {
        <Image
            src="https://placehold.co/500x300"
            alt="Lazy Boundary Example"
            layout={Layout::Responsive}
            width="500"
            height="300"
            // lazy_boundary="500px"
        />
    }
}

#[component]
pub fn Example14() -> impl IntoView {
    view! {
        <Image
            src="https://placehold.co/300x300"
            alt="Unoptimized Image"
            //unoptimized=true
        />
    }
}

#[component]
pub fn Example15() -> impl IntoView {
    view! {
        <Image
            src="https://placehold.co/600x400"
            alt="Custom Object Position"
            object_fit={ObjectFit::Cover}
            object_position={Position::TopRight}
            layout={Layout::Responsive}
            width="600"
            height="400"
        />
    }
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        <div class="m-6 min-h-screen flex flex-col items-center justify-center">
            <h1 class="text-3xl font-bold mb-8 text-white">{ "Image RS Leptos Examples" }</h1>
            <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-8">
                <div class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md">
                    <h2 class="text-xl font-bold mb-2">{ "Basic Image" }</h2>
                    <pre
                        class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto"
                    >
                        { r#"use image_rs::leptos::Image;
use leptos::{*, prelude::*};

#[component]
pub fn Example1() -> impl IntoView {
    view! {
        <Image
            src="https://placehold.co/300x200"
            alt="Basic Image"
        />
    }
}"# }
                    </pre>
                    <Example1 />
                </div>
                <div class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md">
                    <h2 class="text-xl font-bold mb-2">{ "Fixed Layout" }</h2>
                    <pre
                        class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto"
                    >
                        { r#"use image_rs::leptos::Image;
use image_rs::Layout;
use leptos::{*, prelude::*};

#[component]
pub fn Example2() -> impl IntoView {
    view! {
        <Image
            src="https://placehold.co/300x200"
            alt="Fixed Layout"
            layout={Layout::Fixed}
            width="300"
            height="200"
        />
    }
}"# }
                    </pre>
                    <Example2 />
                </div>
                <div class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md">
                    <h2 class="text-xl font-bold mb-2">{ "Responsive Layout" }</h2>
                    <pre
                        class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto"
                    >
                        { r#"use image_rs::leptos::Image;
use image_rs::Layout;
use leptos::{*, prelude::*};

#[component]
pub fn Example3() -> impl IntoView {
    view! {
        <Image
            src="https://placehold.co/600x400"
            alt="Responsive Layout"
            layout={Layout::Responsive}
            width="600"
            height="400"
        />
    }
}"# }
                    </pre>
                    <Example3 />
                </div>
                <div class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md">
                    <h2 class="text-xl font-bold mb-2">{ "Blur Placeholder" }</h2>
                    <pre
                        class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto"
                    >
                        { r#"use image_rs::leptos::Image;
use image_rs::Layout;
use leptos::{*, prelude::*};

#[component]
pub fn Example4() -> impl IntoView {
    view! {
        <Image
            src="https://placehold.co/600x400"
            alt="Blurred Image"
            layout={Layout::Responsive}
            width="600"
            height="400"
            placeholder="blur"
            blur_data_url="https://placehold.co/10x10"
        />
    }
}"# }
                    </pre>
                    <Example4 />
                </div>
                <div class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md">
                    <h2 class="text-xl font-bold mb-2">{ "Object Fit: Cover" }</h2>
                    <pre
                        class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto"
                    >
                        { r#"use image_rs::leptos::Image;
use image_rs::{Layout, ObjectFit};
use leptos::{*, prelude::*};

#[component]
pub fn Example5() -> impl IntoView {
    view! {
        <Image
            src="https://placehold.co/600x400"
            alt="Cover Fit"
            layout={Layout::Responsive}
            width="600"
            height="400"
            object_fit={ObjectFit::Cover}
        />
    }
}"# }
                    </pre>
                    <Example5 />
                </div>
                <div class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md">
                    <h2 class="text-xl font-bold mb-2">{ "Error Handling (Press F12)" }</h2>
                    <pre
                        class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto"
                    >
                        { r#"
use image_rs::leptos::Image;
use leptos::{*, prelude::*};

#[component]
pub fn Example6() -> impl IntoView {
    view! {
        <Image
            src="https://invalid.url"
            fallback_src="https://jsonplaceholder.typicode.com/posts/1"
            alt="Broken Image"
            on_error={Callback::new(|e| log!("Failed to load: {}", e))}
        />
    }
}"# }
                    </pre>
                    <Example6 />
                </div>
                <div class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md">
                    <h2 class="text-xl font-bold mb-2">{ "Priority Loading" }</h2>
                    <pre
                        class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto"
                    >
                        { r#"
use image_rs::leptos::Image;
use image_rs::Loading;
use leptos::{*, prelude::*};

#[component]
pub fn Example7() -> impl IntoView {
    view! {
        <Image
            src="https://placehold.co/400x300"
            alt="Priority Image"
            loading={Loading::Eager}
        />
    }
}"# }
                    </pre>
                    <Example7 />
                </div>
                <div class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md">
                    <h2 class="text-xl font-bold mb-2">{ "Async Decoding" }</h2>
                    <pre
                        class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto"
                    >
                        { r#"use image_rs::leptos::Image;
use image_rs::Decoding;
use leptos::{*, prelude::*};

#[component]
pub fn Example8() -> impl IntoView {
    view! {
        <Image
            src="https://placehold.co/400x300"
            alt="Async Decoding"
            decoding={Decoding::Async}
        />
    }
}"# }
                    </pre>
                    <Example8 />
                </div>
                <div class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md">
                    <h2 class="text-xl font-bold mb-2">{ "Quality Setting" }</h2>
                    <pre
                        class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto"
                    >
                        { r#"use image_rs::leptos::Image;
use leptos::{*, prelude::*};

#[component]
pub fn Example9() -> impl IntoView {
    view! {
        <Image
            src="https://placehold.co/400x300"
            alt="High Quality"
            quality="90"
        />
    }
}"# }
                    </pre>
                    <Example9 />
                </div>
//                 <div class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md">
//                     <h2 class="text-xl font-bold mb-2">{ "ARIA Attributes" }</h2>
//                     <pre
//                         class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto"
//                     >
//                         { r#"use image_rs::leptos::Image;
// use image_rs::{Layout, Decoding, AriaLive, AriaPressed};
// use leptos::{*, prelude::*};

// #[component]
// pub fn Example10() -> impl IntoView {
//     view! {
//         <Image
//             src="https://placehold.co/500x300"
//             alt="Accessible Image"
//             aria_labelledby="imageLabel"
//             aria_describedby="imageDescription"
//             aria_hidden="false"
//             aria_live={AriaLive::Polite}
//             aria_current="page"
//             aria_expanded="false"
//             aria_pressed={AriaPressed::False}
//             aria_controls="imageControl"
//             decoding={Decoding::Sync}
//             layout={Layout::Intrinsic}
//             width="500"
//             height="300"
//         />
//     }
// }"# }
//                     </pre>
//                     <Example10 />
//                 </div>
                <div class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md">
                    <h2 class="text-xl font-bold mb-2">{ "Custom Style" }</h2>
                    <pre
                        class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto"
                    >
                        { r#"use image_rs::leptos::Image;
use leptos::{*, prelude::*};
            
#[component]
pub fn Example11() -> impl IntoView {
    view! {
        <Image
            src="https://placehold.co/400x300"
            alt="Custom Style"
            style="border-radius: 12px; border: 2px solid black;"
        />
    }
}"# }
                    </pre>
                    <Example11 />
                </div>
                <div class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md">
                    <h2 class="text-xl font-bold mb-2">{ "Custom Class" }</h2>
                    <pre
                        class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto"
                    >
                        { r#"use image_rs::leptos::Image;
use leptos::{*, prelude::*};
            
#[component]
pub fn Example12() -> impl IntoView {
    view! {
        <Image
            src="https://placehold.co/300x200"
            alt="Custom Class"
            class="rounded-lg shadow-lg"
        />
    }
}"# }
                    </pre>
                    <Example12 />
                </div>
                <div class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md">
                    <h2 class="text-xl font-bold mb-2">{ "Lazy Boundary" }</h2>
                    <pre
                        class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto"
                    >
                        { r#"use image_rs::leptos::Image;
use image_rs::Layout;
use leptos::{*, prelude::*};
            
#[component]
pub fn Example13() -> impl IntoView {
    view! {
        <Image
            src="https://placehold.co/500x300"
            alt="Lazy Boundary Example"
            layout={Layout::Responsive}
            width="500"
            height="300"
            lazy_boundary="500px"
        />
    }
}"# }
                    </pre>
                    <Example13 />
                </div>
                <div class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md">
                    <h2 class="text-xl font-bold mb-2">{ "Unoptimized" }</h2>
                    <pre
                        class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto"
                    >
                        { r#"use image_rs::leptos::Image;
use leptos::{*, prelude::*};
            
#[component]
pub fn Example14() -> impl IntoView {
    view! {
        <Image
            src="https://placehold.co/300x300"
            alt="Unoptimized Image"
            unoptimized={true}
        />
    }
}"# }
                    </pre>
                    <Example14 />
                </div>
                <div class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md">
                    <h2 class="text-xl font-bold mb-2">{ "Object Position" }</h2>
                    <pre
                        class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto"
                    >
                        { r#"use image_rs::leptos::Image;
use image_rs::{Layout, Position, ObjectFit};
use leptos::{*, prelude::*};
            
#[component]
pub fn Example15() -> impl IntoView {
    view! {
        <Image
            src="https://placehold.co/600x400"
            alt="Custom Object Position"
            object_fit={ObjectFit::Cover}
            object_position={Position::TopRight}
            layout={Layout::Responsive}
            width="600"
            height="400"
        />
    }
}"# }
                    </pre>
                    <Example15 />
                </div>
            </div>
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::default());
    leptos::mount::mount_to_body(|| view! { <App/> })
}
