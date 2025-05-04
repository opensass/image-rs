# Y Image RS Yew Usage

Adding Image RS to your project is simple:

1. Make sure your project is set up with **Yew**. Follow their [Getting Started Guide](https://yew.rs/docs/getting-started/introduction) for setup instructions.

1. Add the Image RS component to your dependencies by including it in your `Cargo.toml` file:

   ```sh
   cargo add image-rs --features=yew
   ```

1. Import the `Image` component into your Yew component and start using it in your app.

## 🛠️ Usage

Incorporating Image RS into your Yew application is easy. Follow these steps:

1. Import the `Image` component into your Yew project:

   ```rust
   use yew::prelude::*;
   use image_rs::yew::Image;
   use image_rs::Layout;
   ```

1. Use the `Image` component within your Yew application:

   ```rust
   use yew::prelude::*;
   use image_rs::yew::Image;
   use image_rs::Layout;

   #[function_component(App)]
   pub fn app() -> Html {
       html! {
           <Image
               src="/images/photo.jpg"
               alt="A beautiful view"
               width="800"
               height="600"
               layout={Layout::Responsive}
           />
       }
   }
   ```

## 🔧 Props

### 🖼️ Main Props

| Property       | Type           | Description                                 | Default      |
| -------------- | -------------- | ------------------------------------------- | ------------ |
| `src`          | `&'static str` | The image source URL.                       | `""`         |
| `alt`          | `&'static str` | Alt text for accessibility.                 | `"Image"`    |
| `fallback_src` | `&'static str` | Image shown if the primary source fails.    | `""`         |
| `width`        | `&'static str` | Width in pixels.                            | `""`         |
| `height`       | `&'static str` | Height in pixels.                           | `""`         |
| `layout`       | `Layout`       | Layout type: `Responsive`, `Fixed`, etc.    | `Responsive` |
| `placeholder`  | `&'static str` | Placeholder image while loading.            | `"empty"`    |
| `loading`      | `Loading`      | `Lazy` or `Eager` loading strategy.         | `Lazy`       |
| `priority`     | `bool`         | (Deprecated) Use `loading = Eager` instead. | `false`      |

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

| Property          | Type           | Description                                          | Default   |
| ----------------- | -------------- | ---------------------------------------------------- | --------- |
| `style`           | `&'static str` | Inline CSS styles.                                   | `""`      |
| `class`           | `&'static str` | CSS class name(s).                                   | `""`      |
| `object_fit`      | `ObjectFit`    | How the image fits: `Contain`, `Cover`, `Fill`, etc. | `Contain` |
| `object_position` | `Position`     | Image position inside container (e.g., `TopRight`).  | `Center`  |
| `sizes`           | `&'static str` | Defines image sizes for responsive rendering.        | `""`      |
| `quality`         | `&'static str` | Image quality hint (`"low"`, `"high"`).              | `""`      |
| `blur_data_url`   | `&'static str` | Low-quality blur-up image while loading.             | `""`      |

### ⚙️ Behavioral Props

| Property   | Type               | Description                                             | Default |
| ---------- | ------------------ | ------------------------------------------------------- | ------- |
| `on_load`  | `Callback<()>`     | Triggered when image finishes loading.                  | No-op   |
| `on_error` | `Callback<String>` | Triggered if the image fails to load.                   | No-op   |
| `decoding` | `Decoding`         | Controls image decode strategy: `Auto`, `Sync`, `Async` | `Auto`  |

### 🌐 Network & Source Props

| Property         | Type             | Description                                         | Default             |
| ---------------- | ---------------- | --------------------------------------------------- | ------------------- |
| `srcset`         | `&'static str`   | Comma-separated list of responsive image sources.   | `""`                |
| `crossorigin`    | `CrossOrigin`    | CORS policy (`Anonymous`, `UseCredentials`).        | `CrossOrigin::None` |
| `referrerpolicy` | `ReferrerPolicy` | Referrer policy for requests.                       | `NoReferrer`        |
| `usemap`         | `&'static str`   | Associates the image with a `<map>` by ID.          | `""`                |
| `ismap`          | `bool`           | Enables server-side image maps (inside `<a href>`). | `false`             |

### ⚡ Performance Props

| Property         | Type            | Description                                                 | Default   |
| ---------------- | --------------- | ----------------------------------------------------------- | --------- |
| `fetchpriority`  | `FetchPriority` | Network priority (`High`, `Low`, `Auto`).                   | `Auto`    |
| `elementtiming`  | `&'static str`  | Marks image with ID for `PerformanceElementTiming`.         | `""`      |
| `attributionsrc` | `&'static str`  | URL for Attribution Reporting (experimental).               | `""`      |
| `lazy_boundary`  | `&'static str`  | Distance from viewport to trigger lazy load (e.g. `200px`). | `"100px"` |
| `unoptimized`    | `bool`          | Disables built-in image optimization.                       | `false`   |

### 🧠 Accessibility Props (ARIA)

| Property           | Type           | Description                                                        | Default   |
| ------------------ | -------------- | ------------------------------------------------------------------ | --------- |
| `aria_current`     | `&'static str` | Indicates the current step/page (`"page"`, `"step"`, etc).         | `""`      |
| `aria_describedby` | `&'static str` | ID of an element describing the image.                             | `""`      |
| `aria_expanded`    | `&'static str` | `"true"` or `"false"` for expanded/collapsed state.                | `""`      |
| `aria_hidden`      | `&'static str` | Hides image from assistive tech (`"true"` or `"false"`).           | `"false"` |
| `aria_live`        | `AriaLive`     | Dynamic update priority (`Off`, `Polite`, `Assertive`).            | `Off`     |
| `aria_pressed`     | `AriaPressed`  | Indicates toggle state (`True`, `False`, `Mixed`, or `Undefined`). | `False`   |
| `aria_controls`    | `&'static str` | ID of the element that the image controls.                         | `""`      |
| `aria_labelledby`  | `&'static str` | ID of the label element for the image.                             | `""`      |

### 🧱 Utility

| Property   | Type      | Description                                                    | Default |
| ---------- | --------- | -------------------------------------------------------------- | ------- |
| `node_ref` | `NodeRef` | Reference to the DOM image element for JS interop or tracking. | Default |

## 💡 Notes

- The `src` and `alt` attributes are required for basic functionality.
- The `width` and `height` are essential for `Responsive`, `Intrinsic`, and `Fixed` layouts.
- Use the `placeholder` and `blur_data_url` properties for a smoother loading experience.
- Customize the appearance and behavior using `class`, `style`, and other props like `layout` and `object_fit`.
- Callbacks like `on_load` and `on_error` allow you to handle the image loading process effectively.
- The `Layout::Fill` value ignores the width and height props and stretches to fill the container.
- Accessibility attributes like `aria-label` and `aria-hidden` can be used directly on the image element.
- Priority images (`priority = true`) are loaded eagerly rather than lazily.
- If both `src` and `fallback_src` fail, the `on_error` callback is triggered with an error message.
- `blur_data_url` is used for rendering a low-quality blurred image while the full image loads.
- **IntersectionObserver**: This is used for intelligent lazy loading of images as they enter the viewport.
- **Async/Await**: Fetch operations use non-blocking async/await for smoother fallback handling.

## 📈 Benchmark

1. Open browser DevTools (Press F12) > **Lighthouse** tab.
1. Record page load by clicking "Analyze Page Load".
1. Observe:
   - **First Contentful Paint (FCP)**
   - **Largest Contentful Paint (LCP)**
   - **Time to Interactive (TTI)**
   - **Total network transfer size**
   - **Memory usage**

### 🚀 Summary

| Feature                     | Yew Image RS | Next.js Image |
| :-------------------------- | :----------- | :------------ |
| Native Rust+Wasm            | ✅           | ❌            |
| Built-in Image Optimization | ✅           | ✅            |
| SSR/SEO Friendly            | ✅           | ✅            |
| Fine-grained DOM Control    | ✅           | ❌            |
| Smaller JS Payload          | ✅           | ✅            |

### 📊 Performance Results

When loading **10 images**, **Yew Image RS** and **Next.js Image** are **on par**:

| Metric                         | Yew (Wasm) | Next.js |
| :----------------------------- | :--------- | :------ |
| Performance Score (Lighthouse) | 100        | 100     |
| Memory Usage (Heap)            | ~8 MB      | ~8 MB   |

However, when scaling up to **10,000 images loaded simultaneously**:

| Metric                         | Yew (Wasm)  | Next.js               |
| :----------------------------- | :---------- | :-------------------- |
| Performance Score (Lighthouse) | 64          | ❌ (Lighthouse fails) |
| Memory Usage (Heap)            | ~78 MB      | ~83 MB                |
| Scrolling Smoothness           | Very Smooth | Laggy                 |

**Key observations:**

- **Wasm** (Yew) handles large DOM updates much better than **JavaScript**.
- **Memory usage** is slightly lower with **Wasm**.
- **Next.js** site **failed Lighthouse audit** at 10,000 images (due to TTI timeout).
- **Smoothness** is significantly better with Yew under heavy load.

### 🛠️ Future Improvements

- **Image RS** is working on **automatic image optimization**.
- **Progressive loading** and **lazy hydration** strategies are being researched for even better large-scale performance.
