# 🌱 Leptos Image RS Usage

Adding Image RS to your Leptos project is simple:

1. Make sure your project is set up with Leptos. Refer to their [Getting Started Guide](https://book.leptos.dev/getting_started/index.html) for setup instructions.

1. Add `image-rs` to your dependencies:

   ```sh
   cargo add image-rs --features=lep
   ```

1. Import the `Image` component into your Leptos component and start using it in your app.

## 🛠️ Usage

Incorporating the `Image` component into your Leptos application is easy. Here's a basic example:

```rust
use leptos::{*, prelude::*};
use image_rs::leptos::Image;
use image_rs::Layout;
use leptos::logging::log;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Image
            src="https://example.com/image.jpg"
            alt="An example image"
            width="600"
            height="400"
            layout=Layout::Responsive
            class="my-image"
            style="border-radius: 8px;"
            placeholder="blur"
            blur_data_url="data:image/png;base64,..."
            fallback_src="https://example.com/fallback.jpg"
            on_load=Callback::new(|_| log!("Image loaded successfully"))
            on_error=Callback::new(|err| log!("Image failed: {err}"))
        />
    }
}
```

## 🔧 Props

### `Image` Props

#### Main Props

| Property       | Type           | Description                                         | Default      |
| -------------- | -------------- | --------------------------------------------------- | ------------ |
| `src`          | `&'static str` | The URL of the image to be displayed.               | `""`         |
| `alt`          | `&'static str` | Alt text for accessibility and SEO.                 | `"Image"`    |
| `fallback_src` | `&'static str` | URL for the fallback image in case of error.        | `""`         |
| `width`        | `&'static str` | The width of the image.                             | `""`         |
| `height`       | `&'static str` | The height of the image.                            | `""`         |
| `layout`       | `Layout`       | Image layout: `Fill`, `Responsive`, or `Intrinsic`. | `Responsive` |

#### Loading & Placeholder Props

| Property        | Type           | Description                                             | Default   |
| --------------- | -------------- | ------------------------------------------------------- | --------- |
| `loading`       | `Loading`      | Image loading behavior: `Eager` or `Lazy`.              | `Lazy`    |
| `placeholder`   | `&'static str` | Placeholder type: use `"blur"` for blurred placeholder. | `"empty"` |
| `blur_data_url` | `&'static str` | Base64-encoded data URL used when `placeholder="blur"`. | `""`      |

#### Styling Props

| Property          | Type           | Description                              | Default  |
| ----------------- | -------------- | ---------------------------------------- | -------- |
| `class`           | `&'static str` | CSS class for the `<img>` element.       | `""`     |
| `style`           | `&'static str` | Additional inline styles.                | `""`     |
| `object_fit`      | `ObjectFit`    | How the image should fit within its box. | `Fill`   |
| `object_position` | `Position`     | Image alignment inside the container.    | `Center` |
| `sizes`           | `&'static str` | Sizes attribute for responsive images.   | `""`     |
| `srcset`          | `&'static str` | Srcset for responsive image variants.    | `""`     |

#### Event Callbacks

| Property   | Type                       | Description                                  |
| ---------- | -------------------------- | -------------------------------------------- |
| `on_load`  | `Option<Callback<()>>`     | Triggered when the image successfully loads. |
| `on_error` | `Option<Callback<String>>` | Triggered when the image fails to load.      |

#### Advanced Browser Attributes

| Property         | Type             | Description                              | Default      |
| ---------------- | ---------------- | ---------------------------------------- | ------------ |
| `decoding`       | `Decoding`       | How the browser should decode the image. | `Auto`       |
| `crossorigin`    | `CrossOrigin`    | CORS behavior for the image.             | `None`       |
| `referrerpolicy` | `ReferrerPolicy` | Referrer policy for the request.         | `NoReferrer` |
| `fetchpriority`  | `FetchPriority`  | Priority for fetching the image.         | `Auto`       |

#### Accessibility Props

| Property           | Type           | Description                                              | Default     |
| ------------------ | -------------- | -------------------------------------------------------- | ----------- |
| `aria_current`     | `&'static str` | ARIA current attribute                                   | `""`        |
| `aria_describedby` | `&'static str` | ARIA description                                         | `""`        |
| `aria_expanded`    | `&'static str` | ARIA expanded attribute                                  | `""`        |
| `aria_hidden`      | `&'static str` | Whether the image is hidden from assistive technologies. | `""`        |
| `aria_live`        | `AriaLive`     | ARIA live region setting.                                | `Off`       |
| `aria_pressed`     | `AriaPressed`  | ARIA pressed state.                                      | `Undefined` |
| `aria_controls`    | `&'static str` | ID of the element that this controls.                    | `""`        |
| `aria_labelledby`  | `&'static str` | ID of the element that labels this image.                | `""`        |

#### Other Props

| Property         | Type           | Description                                          | Default |
| ---------------- | -------------- | ---------------------------------------------------- | ------- |
| `node_ref`       | `NodeRef<Img>` | Reference to the `<img>` DOM element.                | `None`  |
| `usemap`         | `&'static str` | HTML `usemap` attribute value.                       | `""`    |
| `ismap`          | `bool`         | Indicates if the image is part of a server-side map. | `false` |
| `elementtiming`  | `&'static str` | Used for performance reporting (e.g., LCP).          | `""`    |
| `attributionsrc` | `&'static str` | Attribution source for content licensing.            | `""`    |

## 💡 Notes

- The `Image` component is flexible for layout use cases: use `Fill` to cover parent, or `Responsive` to maintain aspect ratio.
- Use the `placeholder="blur"` and `blur_data_url` to provide a low-res preview while loading.
- Fallback logic automatically switches to `fallback_src` if the main image fails to load.
- All ARIA attributes and semantic accessibility features are built-in and customizable.
- The component supports lazy loading by default with `loading=Loading::Lazy`.
