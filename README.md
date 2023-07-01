# rustystack-ui
the rustystack frontend, an ungodly amalgam of dioxus, html through macros, and formantic for ease of styling.
***
# description and motivations
originally, it was based upon yew and tailwind, but solution was discarded basically at the start of development, for some very simple reasoning:
- it was annoying to install without getting javascript involved, and this should be as pure rust as possible (with only css and html to assist it)
- yew requires the nightly rust toolchain and i couldn't be bothered to get the rust-analyzer to work with macros in nightly

progress will likely be slow, maybe even stop for periods of time, but that's fine, this is a **passion project** first and foremost.

i love rust, i have minimal webdev experience, and as rust and rust-like languages crop up, this could be an exciting and active frontier to be exploring.
***
# instructions
make sure to install `dioxus cli`.
``` bash
cargo install dioxus-cli
# this install takes some time...
```

then type into your terminal, when you are ready to 'compile' the websites,
```bash
dioxus build ./index.html
```

this will then cause your `./dist` folder to be populated with the `wasm` content, and unfortunately a tiny bit of `js` too. hopefully one day all of this can be purged in favor of a 100% rust-based front-end, however as far as i understand at this point, `js` is actually favorable, since DOM calls from `js` is actually faster

alternatively, if you want to rapidly develop the site, or need to rapidly build the site (such as in production, when frontend changes in response to backend changes), type the following into your terminal
```bash
dioxus serve ./index.html
```

this will casue `dioxus` to watch your `./src/*` files, and repopulate the `./dist` folder in response to changes.

this will also cause `dioxus` to load your frontend for viewing (rapid development and iteration) at `localhost:8080`.
***
# styling
the majority of the reusable blocks of code takes the following structure
```rust
fn reusable_block(cx: Scope<()>) -> Element {
  cx.render(rsx ! {
    div {
      class: "<fomantic styling>",
      <css_property_1>: "<value 1>",
      // ...
      <css_property_n>: "<value n>",
      <html component> {
        <content>
      }
    }
  })
}
```
