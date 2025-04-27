use image_rs::yew::Image;
use image_rs::{Decoding, Layout, ObjectFit, Position};
use yew::prelude::*;

#[function_component(LandingPage)]
pub fn landing_page() -> Html {
    let images: Vec<Html> = (0..10000)
        .map(|i| {
            html! {
                <Image
                    src="https://placehold.co/800?text=Hello+World&font=roboto"
                    alt="Photo"
                    width="400"
                    height="600"
                    layout={Layout::Responsive}
                    quality="high"
                    placeholder="https://placehold.co/800?text=Hello+World&font=roboto"
                    fallback_src="https://placehold.co/800?text=Hello+World&font=roboto"
                    priority={i < 5}
                    decoding={Decoding::Async}
                    object_fit={ObjectFit::Cover}
                    object_position={Position::Center}
                    style="border-radius: 8px;"
                    class="benchmark-image"
                />
            }
        })
        .collect();

    html! { <div class="image-grid">{ for images }</div> }
}
