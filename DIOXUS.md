# 🧬 Image RS Dioxus Usage

Adding Image RS to your project is simple:

1. Make sure your project is set up with **Dioxus**. Refer to the [Dioxus Getting Started Guide](https://dioxuslabs.com/learn/0.6/getting_started) for setup instructions.

1. Add the **image-rs** library to your dependencies by including it in your `Cargo.toml` file:

   ```sh
   cargo add image-rs --features=dio
   ```

1. Import the `Image` component into your Dioxus application.

## 🛠️ Usage

Incorporating `Image RS` into your **Dioxus** application is simple. Here's how:

1. Add the `Image` component to your Dioxus project:

   ```rust
   use dioxus::prelude::*;
   use image_rs::dioxus::Image;
   ```

1. Use the `Image` component in your app like this:

   ```rust
   use dioxus::prelude::*;
   use image_rs::dioxus::Image;
   use image_rs::{Layout, Loading};

   fn App() -> Element {
       rsx! {
           Image {
               src: "/images/photo.jpg",
               alt: "A beautiful view",
               width: "800",
               height: "600",
               layout: Layout::Responsive,
               loading: Loading::Lazy,
           }
       }
   }
   ```

## 🔧 Props

### 🖼️ Main Props

| Property       | Type           | Description                             | Default      |
| -------------- | -------------- | --------------------------------------- | ------------ |
| `src`          | `&'static str` | Image source path or URL                | `""`         |
| `alt`          | `&'static str` | Alt text for accessibility              | `"Image"`    |
| `fallback_src` | `&'static str` | Fallback image if `src` fails           | `""`         |
| `width`        | `&'static str` | Width in pixels                         | `""`         |
| `height`       | `&'static str` | Height in pixels                        | `""`         |
| `layout`       | `Layout`       | Layout strategy: Responsive, Fill, etc. | `Responsive` |
| `placeholder`  | `&'static str` | Placeholder while loading               | `"empty"`    |
| `loading`      | `Loading`      | Load strategy: `Lazy` or `Eager`        | `Lazy`       |

### 🎨 Styling Props

```sh
+-----------------------------------------------------------+
|                  [Image Container]                        |  <-- `class` & `style`
|                                                           |
|   +-----------------------------------------------+       |  <-- `layout`
|   |              [Loaded Image]                   |       |  <-- `src`
|   +-----------------------------------------------+       |
|                                                           |
+-----------------------------------------------------------+
```

| Property          | Type           | Description                                 | Default   |
| ----------------- | -------------- | ------------------------------------------- | --------- |
| `class`           | `&'static str` | CSS classes                                 | `""`      |
| `style`           | `&'static str` | Inline CSS styles                           | `""`      |
| `object_fit`      | `ObjectFit`    | Resizing mode: `Cover`, `Contain`, etc.     | `Contain` |
| `object_position` | `Position`     | Alignment inside container                  | `Center`  |
| `sizes`           | `&'static str` | Responsive image size hints                 | `""`      |
| `quality`         | `&'static str` | Image quality hint                          | `""`      |
| `blur_data_url`   | `&'static str` | Low-res blurred image to show while loading | `""`      |

### ⚙️ Behavioral Props

| Property   | Type               | Description                                | Default |
| ---------- | ------------------ | ------------------------------------------ | ------- |
| `on_load`  | `Callback<()>`     | Called when image has loaded               | No-op   |
| `on_error` | `Callback<String>` | Called when image fails to load            | No-op   |
| `decoding` | `Decoding`         | Image decoding strategy: Auto, Sync, Async | `Auto`  |

### 🌐 Network & Source Props

| Property         | Type             | Description                                  | Default      |
| ---------------- | ---------------- | -------------------------------------------- | ------------ |
| `srcset`         | `&'static str`   | Set of image sources for responsive behavior | `""`         |
| `crossorigin`    | `CrossOrigin`    | CORS mode                                    | `None`       |
| `referrerpolicy` | `ReferrerPolicy` | Controls how much referrer info is sent      | `NoReferrer` |
| `usemap`         | `&'static str`   | Use with image maps                          | `""`         |
| `ismap`          | `bool`           | Server-side image maps inside `<a>` tags     | `false`      |

### ⚡ Performance Props

| Property         | Type            | Description                                      | Default   |
| ---------------- | --------------- | ------------------------------------------------ | --------- |
| `fetchpriority`  | `FetchPriority` | Image fetch priority (`Auto`, `High`, `Low`)     | `Auto`    |
| `elementtiming`  | `&'static str`  | Performance marker ID                            | `""`      |
| `attributionsrc` | `&'static str`  | Attribution reporting URL (experimental)         | `""`      |
| `lazy_boundary`  | `&'static str`  | How early to trigger lazy load (`e.g., "200px"`) | `"100px"` |
| `unoptimized`    | `bool`          | Disables automatic image optimizations           | `false`   |

### 🧠 Accessibility Props (ARIA)

| Property           | Type           | Description                                                    | Default   |
| ------------------ | -------------- | -------------------------------------------------------------- | --------- |
| `aria_current`     | `&'static str` | Current step/item indicator (`"page"`, `"step"`, etc.)         | `""`      |
| `aria_describedby` | `&'static str` | ID of an element describing the image                          | `""`      |
| `aria_expanded`    | `&'static str` | `"true"` or `"false"` if content is expanded                   | `""`      |
| `aria_hidden`      | `&'static str` | Hides image from assistive tech                                | `"false"` |
| `aria_live`        | `AriaLive`     | Priority of live region updates (`Off`, `Polite`, `Assertive`) | `Off`     |
| `aria_pressed`     | `AriaPressed`  | Toggle state (`True`, `False`, `Mixed`, `Undefined`)           | `False`   |
| `aria_controls`    | `&'static str` | ID of element this image controls                              | `""`      |
| `aria_labelledby`  | `&'static str` | ID of label element                                            | `""`      |

## 💡 Notes

- **Required**: `src` and `alt` are essential for accessibility and rendering.
- **Layout behavior**:

  - `Responsive`, `Fill`, `Intrinsic`, and `Fixed` alter how dimensions are applied.
  - `Fill` ignores width/height and stretches the image.

- **Loading**:

  - `placeholder` and `blur_data_url` create a smoother user experience.
  - Use `on_load` / `on_error` for lifecycle management.

- **Lazy loading**:

  - Uses [IntersectionObserver](https://developer.mozilla.org/en-US/docs/Web/API/Intersection_Observer_API).
  - `lazy_boundary` controls how early the image loads.

- **Optimization**:

  - `unoptimized = true` disables default optimizations.

- **Performance tracking**:

  - `elementtiming` and `attributionsrc` assist in analytics and attribution.

- **Accessibility**:

  - Add `aria-*` attributes to enhance usability with screen readers and assistive devices.
