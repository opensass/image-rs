<div align="center">

# 🖼️ Image RS

[![Crates.io](https://img.shields.io/crates/v/image-rs)](https://crates.io/crates/image-rs)
[![Crates.io Downloads](https://img.shields.io/crates/d/image-rs)](https://crates.io/crates/image-rs)
![Crates.io License](https://img.shields.io/crates/l/image-rs)
[![made-with-rust](https://img.shields.io/badge/Made%20with-Rust-1f425f.svg?logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Rust](https://img.shields.io/badge/Rust-1.85%2B-blue.svg)](https://www.rust-lang.org)
[![Maintenance](https://img.shields.io/badge/Maintained%3F-yes-green.svg)](https://github.com/wiseaidev)

[![Join our Discord](https://dcbadge.limes.pink/api/server/b5JbvHW5nv)](https://discord.gg/b5JbvHW5nv)

<!-- absolute url for docs.rs cause assets is excluded from crate -->
![logo](https://raw.githubusercontent.com/opensass/image-rs/refs/heads/main/assets/logo.webp)

</div>

## 🎬 Demo

<!-- absolute url for docs.rs cause assets is excluded from crate -->
![image-rs-demo](https://raw.githubusercontent.com/opensass/image-rs/refs/heads/main/assets/demo.gif)

| Framework | Live Demo |
| --- | --- |
| Yew | [![Netlify Status](https://api.netlify.com/api/v1/badges/a0efc7e9-f20e-4dd9-93e1-c8f4fde7506f/deploy-status)](https://image-rs.netlify.app) |
| Dioxus | TODO |
| Leptos | TODO |

## 📜 Intro

Image RS is a **highly optimized**, **feature-rich** image component built for **WASM-based frameworks** like **Yew**, **Dioxus**, and **Leptos**. It offers lazy loading, blur-up placeholders, fallback image handling, responsive layouts, and full ARIA accessibility.

## 🤔 Why Use Image RS?

The following features make Image RS a must-have for modern WASM apps:

1. **🚀 Performance Optimized**: Smart lazy loading with `IntersectionObserver` and fallback strategies.
1. **🎨 Advanced Layouts**: Responsive, Fill, Intrinsic, Fixed, Stretch, and ScaleDown layouts.
1. **🧩 Accessibility First**: Full ARIA attribute support to build inclusive UIs.
1. **⚡ Interactive Events**: Callbacks for loading completion and error handling.
1. **🖼️ Visual Enhancements**: Blur placeholders and fallback images for seamless loading UX.

## Yew Usage

<!-- absolute url for docs.rs cause YEW.md is not included in crate -->
Refer to [our guide](https://github.com/opensass/image-rs/blob/main/YEW.md) to integrate this component into your Yew app.

## 🧬 Dioxus Usage (TODO)

<!-- absolute url for docs.rs cause DIOXUS.md is not included in crate -->
Refer to [our guide](https://github.com/opensass/image-rs/blob/main/DIOXUS.md) to integrate this component into your Dioxus app.

## 🌱 Leptos Usage (TODO)

<!-- absolute url for docs.rs cause LEPTOS.md is not included in crate -->
Refer to [our guide](https://github.com/opensass/image-rs/blob/main/LEPTOS.md) to integrate this component into your Leptos app.

## 📈 Benchmark

1. Open browser DevTools (Press F12) > **Lighthouse** tab.
1. Record page load by clicking	"Analyze Page Load".
1. Observe:
   - **First Contentful Paint (FCP)**
   - **Largest Contentful Paint (LCP)**
   - **Time to Interactive (TTI)**
   - **Total network transfer size**
   - **Memory usage**

### 🚀 Summary

| Feature | Yew Image RS | Next.js Image |
|:-------|:-------------|:--------------|
| Native Rust+Wasm | ✅ | ❌ |
| Built-in Image Optimization | ✅ | ✅ |
| SSR/SEO Friendly | ✅ | ✅ |
| Fine-grained DOM Control | ✅ | ❌ |
| Smaller JS Payload | ✅ | ✅ |

### 📊 Performance Results

When loading **10 images**, **Yew Image RS** and **Next.js Image** are **on par**:

| Metric | Yew (Wasm) | Next.js |
|:------|:-----------|:--------|
| Performance Score (Lighthouse) | 100 | 100 |
| Memory Usage (Heap) | ~8 MB | ~8 MB |

However, when scaling up to **10,000 images loaded simultaneously**:

| Metric | Yew (Wasm) | Next.js |
|:------|:-----------|:--------|
| Performance Score (Lighthouse) | 64 | ❌ (Lighthouse fails) |
| Memory Usage (Heap) | ~78 MB | ~83 MB |
| Scrolling Smoothness | Very Smooth | Laggy |

**Key observations:**
- **Wasm** (Yew) handles large DOM updates much better than **JavaScript**.
- **Memory usage** is slightly lower with **Wasm**.
- **Next.js** site **failed Lighthouse audit** at 10,000 images (due to TTI timeout).
- **Smoothness** is significantly better with Yew under heavy load.

### 🛠️ Future Improvements

- **Image RS** is working on **automatic image optimization**.
- **Progressive loading** and **lazy hydration** strategies are being researched for even better large-scale performance.

## 🤝 Contributions

Contributions are welcome! Whether it's bug fixes, feature requests, or examples, we would love your help to make Image RS better.

1. Fork the repository.
1. Create a new branch for your feature/bugfix.
1. Submit a pull request for review.

## 📜 License

Image RS is licensed under the [MIT License](LICENSE). You are free to use, modify, and distribute this library in your projects.
