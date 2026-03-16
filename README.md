# Leptodon starter
This is a template repository to get started with [leptos](https://github.com/leptos-rs/leptos) + [leptodon](https://github.com/openanalytics/leptodon) inside an axum ssr environment.

## Getting Started
1. Click the green `Use this template` button and create a repo using this template.
2. Clone your repository
3. Install cargo-leptos `cargo install --locked cargo-leptos`
4. Run with `cargo leptos serve`

### Additional steps for updating the tailwind stylesheet.
5. `npm install`
6. Run tailwind's css generator `npx tailwindcss -i input.css -o style/output.css --watch` 

The starter should now be available on http://localhost:3000

## Versions
- Uses leptos ssr, and based on https://github.com/leptos-rs/start-axum
- Rust stable version 1.92
- Tailwindv3
 
## `build.rs` ?
 - Runs `cargo run codegen` to generate `.tailwind` for tailwind's class generation.
 - Generates `src/generated_demolist.rs` containing a list of all demos to construct routes and sidebar entries programatically. 
   - Allows for easy alfabetical sorting and reduces maintenance burden of adding/removing entries.
