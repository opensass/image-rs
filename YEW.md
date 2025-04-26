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

### `Image` Component Props

#### **Main Props**

| Property              | Type           | Description                                              | Default   |
| --------------------- | -------------- | -------------------------------------------------------- | --------- |
| `src`                 | `&'static str` | The source URL of the image to be displayed.             | `""`      |
| `alt`                 | `&'static str` | The alternative text for the image.                      | `"Image"` |
| `fallback_src`        | `&'static str` | Optional fallback image if the main image fails to load. | `""`      |
| `width`               | `&'static str` | The width of the image in pixels.                        | `"300"`   |
| `height`              | `&'static str` | The height of the image in pixels.                       | `"200"`   |
| `priority`            | `bool`         | Indicates if the image should be loaded eagerly.         | `false`   |
| `placeholder`         | `&'static str` | URL or data URL for a placeholder image while loading.   | `"empty"` |
| `on_loading_complete` | `Callback<()>` | Callback when the image has finished loading.            | No-op     |

#### **Styling Props**

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

| Property          | Type           | Description                                                  | Default              |
| ----------------- | -------------- | ------------------------------------------------------------ | -------------------- |
| `style`           | `&'static str` | Inline styles for the image container.                       | `""`                 |
| `class`           | `&'static str` | CSS class for the image container.                           | `""`                 |
| `layout`          | `Layout`       | Layout of the image container (e.g., `Fixed`, `Intrinsic`).  | `Layout::Responsive` |
| `sizes`           | `&'static str` | Defines image sizes for different viewport widths.           | `""`                 |
| `quality`         | `&'static str` | Specifies image quality (e.g., "high", "medium", "low").     | `""`                 |
| `object_fit`      | `ObjectFit`    | How the image should be resized to fit its container.        | `ObjectFit::Contain` |
| `object_position` | `Position`     | Position of the image within its container (e.g., `Center`). | `Position::Center`   |

#### **Behavioral Props**

| Property              | Type               | Description                                             | Default |
| --------------------- | ------------------ | ------------------------------------------------------- | ------- |
| `on_error`            | `Callback<String>` | Callback triggered when the image fails to load.        | No-op   |
| `on_loading_complete` | `Callback<()>`     | Callback triggered once the image has finished loading. | No-op   |

### Advanced Props

| Property          | Type           | Description                                                   | Default   |
| ----------------- | -------------- | ------------------------------------------------------------- | --------- |
| `aria_labelledby` | `&'static str` | ID of the element that labels the image for accessibility.    | `""`      |
| `aria_hidden`     | `&'static str` | Indicates if the image is hidden from the user.               | `"false"` |
| `aria_current`    | `&'static str` | Describes the image in a navigation context.                  | `""`      |
| `aria_expanded`   | `&'static str` | Indicates whether the image content is expanded or collapsed. | `""`      |

## 💡 Notes

- The `src` and `alt` attributes are required for basic functionality.
- The `width` and `height` are essential for `Responsive`, `Intrinsic`, and `Fixed` layouts.
- Use the `placeholder` and `blur_data_url` properties for a smoother loading experience.
- Customize the appearance and behavior using `class`, `style`, and other props like `layout` and `object_fit`.
- Callbacks like `on_loading_complete` and `on_error` allow you to handle the image loading process effectively.
- The `Layout::Fill` value ignores the width and height props and stretches to fill the container.
- Accessibility attributes like `aria-label` and `aria-hidden` can be used directly on the image element.
- Priority images (`priority = true`) are loaded eagerly rather than lazily.
- If both `src` and `fallback_src` fail, the `on_error` callback is triggered with an error message.
- `blur_data_url` is used for rendering a low-quality blurred image while the full image loads.
- **IntersectionObserver**: This is used for intelligent lazy loading of images as they enter the viewport.
- **Async/Await**: Fetch operations use non-blocking async/await for smoother fallback handling.
