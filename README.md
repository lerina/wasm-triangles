<main>
##  topics:

    What is WebAssembly?
    Creating a Rust and WebAssembly project skeleton
    Translating JavaScript code into Rust code
    Drawing to the screen with HTML5 Canvas

## Context

- Technical requirements: Rust, web browser, http server
    * Wasm toolchain 
        + wasm-pack: This is a Rust tool for building Rust-generated WebAssembly code.
        + wasm-bindgen does is create those bindings and the boilerplate needed to call JavaScript functions from your Rust code, as well as provide tools to create bindings in the other direction so that JavaScript code can call back into the Rust code.
        + web-sys to call browser APIs such as the canvas and requestAnimationFrame.
- Wasm is a binary format. "WebAssembly (abbreviated Wasm) is a binary instruction format for a stack-based virtual machine. Wasm is designed as a portable compilation target for programming languages, enabling deployment on the web for client and server applications." 
This is different than transpiling or source-to-source compiling, where languages such as TypeScript are converted into JavaScript for running in JavaScript environments. Those languages are still ultimately running JavaScript, whereas Wasm is bytecode. This makes it a smaller download and parsing and compiling steps are removed when running it, which can lead to significant performance improvements.
- Rust has a great type system, excellent developer tooling,

## Initialize the project

### Install Rust and Ecosystem

- Install Rust using rustup

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh 
```

- Install wasm-pack

```
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh 
```

- Install a web server

```
cargo install http 
```

### Start the project

```
cargo new --lib wasm-triangles 
cd wasm-triangles 
```

### Web site structure

```
mkdir -p www/js www/css www/html 
```

### Install project dependencies

edit the Cargo.toml file

```
[lib]
crate-type = ["cdylib",]

[dependencies]
wasm-bindgen = "0.2.8"
console_error_panic_hook = "0.1.7"

[dependencies.web-sys]
version = "0.3.58"
features = ["console", "Window", "Document", 
            "HtmlCanvasElement", "CanvasRenderingContext2d", "Element",]


[dev-dependencies]
wasm-bindgen-test = "*"
futures = "0.3.21"
js-sys = "0.3.58"
wasm-bindgen-futures = "0.4.31"
```
 ### download dependencies

 ```
 wasm-pack build --out-dir www/pkg
 ```

## initial drawing to the HTML Canvas

### Tiny index page

```html
<!DOCTYPE html>
<html>
<head>
  <meta charset="UTF-8">
  <title>wasm triangles</title>
  <link rel="stylesheet" href="../css/styles.css">
  <link rel="icon" href="./favicon.png">
</head>
<body>

<main>
  <canvas id="canvas" height="600" width="600">
    Your browser does not support the Canvas.
  </canvas>
</main>

  <script type="module" src="../js/index.js"></script>
</body>
</html>
```

Note: [type="module" is essential](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Guide/Modules#applying_the_module_to_your_html)

### Initial JavaScript 
index.js will just import the wasm bytecode for now.

```javascript
import init from "../pkg/wasm_triangles.js";

init();
```

### Initial Rust Code

```rust
use wasm_bindgen::prelude::*;
use web_sys::console;
use wasm_bindgen::JsCast;

// When attached to a pub function this attribute will configure the start 
// section of the wasm executable to be emitted, executing the tagged function 
// as soon as the wasm module is instantiated.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas = document
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();

    let context = canvas
        .get_context("2d")? //  get_context returns a Result<Option<Object>>
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()?;

    //context.set_fill_style(&"#0000FF".into());     
    context.set_fill_style(&"rgb(150,50,0)".into());
    context.move_to(300.0, 0.0); // top of triangle

    context.begin_path();
    context.line_to(0.0, 600.0); // bottom left of triangle
    context.line_to(600.0, 600.0); // bottom right of triangle
    context.line_to(300.0, 0.0); // back to top of triangle
    context.close_path();
    
    context.stroke();
    context.fill();

    console::log_2(&"Color : %s ".into(), &context.fill_style().into());

    Ok(())
}
```

### Build script for convinience

Put this script in `run.sh`

```sh
# compile for web
wasm-pack build --target web --out-dir www/pkg

# display link for easy access
echo "Serving at: http://127.0.0.1:8080/html/"

# run the web server
http -a 127.0.0.1 -p 8080 www
```

make it executable

```sh
chmod +x run.sh
```

### Run it!

```sh
./run.sh
```

point the browser to http://127.0.0.1:8080/html/

ctrl-shift c

A triangle is drawn in the canvas
 and console displays the log message

 
</main>
