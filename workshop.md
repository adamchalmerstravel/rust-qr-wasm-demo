# Rust WebAssembly workshop for Rust Asia

Welcome to the wild world of WebAssembly. For decades, JavaScript was the only way to run code in the browser. But today, all modern browsers ship with a tiny virtual machine that can run a tiny instruction set called WebAssembly. You're not meant to write WebAssembly instructions by hand. Instead, you compile a higher-level language (like Rust) into WebAssembly. That way, any programming language can run code in the browser as long as they can compile to WebAssembly. In this workshop, we're going to write a web app that uses both JavaScript and Rust together. WebAssembly is the glue that lets you combine both languages in the browser! 

If you're a JavaScript developer, you might want to use WebAssembly to access a huge ecosystem of libraries from other languages, or get better performance for CPU-intensive code. If you're a Rust developer, you might want to use WebAssembly to ship your Rust code in frontend apps, dramatically increasing kinds of software you can ship without mastering all the details of JavaScript.

## Prerequisites:

  1. Install Rustup:

     https://rustup.rs/

  2. Install Rust:
     ```
     rustup install stable
     rustup target add wasm32-unknown-unknown
     ```
  3. Install wasm-pack:

     https://rustwasm.github.io/wasm-pack/installer/

  4. Check wasm-pack is installed properly:
     ```
     wasm-pack --version
     ```
     This command should show `wasm-pack 0.13.1` or newer.
  5. Download this repo: 

     https://github.com/adamchalmerstravel/rust-qr-wasm-demo

     (You can download this repo using the `git` CLI, the GitHub desktop app, or download the code as a ZIP file from the webpage)

  6. Install Python, or some other way to run a local web server. You probably already have Python installed somewhere in your terminal, but if not, download it: 
   
     https://www.python.org/downloads/ 


## 1: Getting to Hello World

### 1.1 Writing a Web-friendly Rust library

You've downloaded the repo. Inside you'll see a Rust project, defined by its `Cargo.toml` file and the `src/` directory. Inside `src/` you'll see a `lib.rs` file which defines a very small Rust library, with two functions. This library will both call JS code from Rust, and define a Rust function that can be called from JS. See, Rust and JS can call each other! The communication goes both ways. Let's see how.

The first function is called `alert`, and it only has a function signature. You'll notice it doesn't have a body! That's because this function isn't written in Rust -- it's a JavaScript function which we're just going to _invoke_ from Rust. The `alert` function is basically just the good old `window.alert()` function from JavaScript, and it can be used from Rust. That's why it has the `#[wasm_bindgen]` attribute and the `extern "C"` block.

The next function is called `msg`, and it does two things. It makes a little popup window by calling the `alert()` function from before, and then it returns a string.

### 1.2 Compiling Rust to WebAssembly

Now that we've seen the Rust library, let's compile it to WebAssembly so that we can call it from the frontend. Open up the terminal and run:

```sh
wasm-pack build --target web --debug
```

> Note: the --debug flag should not be used for production builds of your web app which actually get deployed to users. We're using it here in this workshop because it speeds up WebAssembly compilation. But this comes at the cost of slower runtime performance. In production, remove this --debug flag.

This creates a new directory, `pkg/`. It contains a JavaScript package. Inside are a few files:

 * `package.json` defines the JavaScript library. It's very similar to the `Cargo.toml` which defines the Rust library!
 * `cool_qr.js` defines methods for loading the WebAssembly into JavaScript, sending JavaScript data into the WebAssembly's `msg` function parameters, and getting data back out from the `msg` function's return value into Javascript again.
 * `cool_qr_bg.wasm` is a binary file containing the actual WebAssembly instructions that correspond to the `msg` function body.

There's also TypeScript type definitions, but we aren't using them in this workshop. So don't worry about them.

### 1.3 Calling WebAssembly from Javascript

Now let's actually call those compiled functions! Open up `index.html` and take a look.

You'll see a `<script>` tag with JavaScript code inside it. We start by importing the WebAssembly with an `import` statement, then we can actually call those functions with `console.log(msg())`. Remember, `msg` is a function we defined in Rust, compiled to WebAssembly, and call from JavaScript.

Let's serve the webpage. I'm using Python, but you can use any tool you want. If you're using Python too, run this:

```
python -m http.server 9000
```

Now open up `localhost:9000` in your web browser. If everything went right, you should get a popup alert with the message in it! That's because the `msg` function called `window.alert`. And if you open up your browser's console, you'll see the message we returned from `msg` and then logged via `console.log`.

Congratulations, you've used 3 different tools: Rust, JavaScript, and WebAssembly, and glued them together. Very nice.

### 1.4 Debugging

I personally use a lot of `println!` calls for debugging Rust (and a lot of `console.log` calls for debugging JavaScript). But when your browser runs Rust code via WebAssembly, the Rust code can't use `println!` to help you debug. Nothing will actually get printed. WebAssembly doesn't have a `stdout` to show you these printed messages.

So how do you debug your Rust code inside WebAssembly? Well, instead of `println!` we'll just call the browser's normal `console.log` function, from Rust! First add `web-sys = { version = "0.3.77", features = ["console"] }` to `Cargo.toml` under `[dependencies]`, then put a log statement like this:

```rust
web_sys::console::log_1(&format!("Your log message goes here").into())
```

Now you can print logs from Rust code and see them in the browser console. To learn more, visit https://rustwasm.github.io/wasm-bindgen/examples/console-log.html which explains this is more depth.

## 2: Generating images in Rust

So, we've got the basics of hello world working, showing how Rust and JavaScript can call each other. Now let's do something more advanced: let's generate some PNG images in Rust, and then insert them into our webpage with JavaScript.

### 2.1 Adding crates

We're going to use some Rust libraries (which are usually called "crates"). They're available on [crates.io](https://crates.io), the official Rust package registry. Let's add them to our `Cargo.toml` under the `[dependencies]` key. That's where you tell Cargo which packages your project depends on.

```toml
[dependencies]
base64 = "0.22.1"
image = "0.25"
qrcode = "0.14.1"
```

To make sure Cargo can find these, let's run a quick `cargo check` in the CLI. It should succeed. If not, you probably typed something wrong in `Cargo.toml`.

### 2.2 Generating QR codes

Now let's write a Rust function to build a QR code which encodes some string of text. First, at the top of your Rust file add these imports:

```rust
use base64::Engine;
use image::Luma;
use qrcode::QrCode;
```

Now let's generate a PNG image containing a QR code with some data.

```rust
pub fn qr_png_for(url: String) -> Vec<u8> {
  let code = QrCode::new(url).unwrap();
  let img = code.render::<Luma<u8>>().build();
  let mut bytes = Vec::new();
  img.write_to(
    &mut std::io::Cursor::new(&mut bytes),
    image::ImageFormat::Png,
  )
  .unwrap();
  bytes
}
```

This function returns the raw bytes for a PNG file which contains a QR code that links to the given `url` parameter. Neat! To make it easier for JavaScript to use this image, let's encode it in base64.

```rust
#[wasm_bindgen]
pub fn qr_png_b64(url: String) -> String {
  let png_bytes = qr_png_for(url);
  base64::engine::general_purpose::STANDARD.encode(png_bytes)
}
```

Great! Let's recompile our library to WASM:

```sh
wasm-pack build --target web --debug
```

Make sure it compiles. If something's wrong, check for typos or ask for help!

### 2.3 Showing QR codes in the frontend

Now let's call this QR function from JavaScript. Open up `index.html` and let's replace the `<script>` tag with this instead:

```js
<script type="module">
  import init, {qr_png_b64} from "./pkg/cool_qr.js";
  init().then(() => {
    const updateQR = (event) => {
      const dataInput = document.getElementById("qrData");
      const data = dataInput.value;
      console.log("Data:", data)
      const container = document.getElementById("qr");
      const pngBytes = qr_png_b64(data);
      container.src = "data:image/png;base64," + pngBytes;
    };
    document.getElementById("qrData").addEventListener("input", updateQR)
    updateQR()
  });
</script>
```

What's going on here? Well, we're:

 - Importing the `qr_png_b64` function (defined in Rust and compiled into WebAssembly) into our JavaScript code.
 - Calling it, with the URL the user entered into the HTML text input
 - Getting an empty `<img>` node in the HTML
 - Putting the PNG bytes (which have been base64-encoded) into that `<img>` node

Great! So we've learned that a whole bunch of Rust libraries can be compiled to WebAssembly and work just fine in the browser. On my machine, the WebAssembly library compiles into 814K, which I think is pretty neat. It should be relatively fast for our end users to download.

This is helpful because the Rust logic to render QR codes into PNGs can be packed up into a library, and deployed on:

 - Your backend
 - A CLI tool
 - The frontend (running in the user's machine, in their browser)

You can deploy a Rust library to any of these places, and that's very convenient. You can reuse the same logic on your server or client, and avoid duplicating your logic in multiple languages for multiple environments.

## 3: Using async

Let's add another feature: checking if the URL is valid. You wouldn't want to print out a lot of QR codes only to discover that you made a typo in the URL! This will teach us how to use async Rust inside WebAssembly.

### 3.1 Async Rust

Firstly, let's add a few more crates to the `[dependencies]` section in `Cargo.toml`:

```toml
reqwest = "0.12"
url = "2.5.4"
wasm-bindgen-futures = "0.4.50"
```

Then we'll add an async Rust function which validates URLs.

```rust
#[wasm_bindgen]
pub async fn validate_link(data: String) -> Result<(), String> {
  let url = match url::Url::parse(&data) {
    Ok(u) => u,
    Err(e) => return Err(e.to_string()),
  };
  match reqwest::get(url).await {
    Ok(_) => Ok(()),
    Err(e) => Err(e.to_string()),
  }
}
```

Don't forget to recompile the Rust code with `wasm-pack`!

There's two interesting things about this `validate_link` function: it's async, and it returns a `Result`. How does this get used from JavaScript?

### 3.2 Async and Results in JavaScript

First, update the `import` statement to include `validate_qr`:

```js
import init, {qr_png_b64, validate_link} from "./pkg/cool_qr.js";
```

Now let's add this JS function which calls the Rust function:

```js
async function validateURL(event) {
  const dataInput = document.getElementById("qrData");
  const data = dataInput.value;
  const { isValid, err } = await validate_link(data)
    .then((_) => { return { isValid: true, err: undefined } })
    .catch((err) => { return { isValid: false, err } });
  const isValidSpan = document.getElementById("isValid");
  const isValidText = isValid ? "Link resolves" : "Link does not resolve";
  const isValidRes = document.createTextNode(isValidText);
  while (isValidSpan.firstChild) {
    isValidSpan.removeChild(isValidSpan.firstChild);
  }
  isValidSpan.appendChild(isValidRes);
}
document.getElementById("validate").addEventListener("click", validateURL)
```

When we call `validate_link` we must:

1. Use `await` to make sure the async function actually completes
2. Handle the `Result` by using `.then` to check for the `Ok` variant, and `.catch` to check for the `Err` variant.

Then we can insert text into the webpage depending on what the Rust function returned.

## 4: Extensions

If you've made it this far, great! Here are some ideas to extend this app.

 - Render mathematical fractals, like the Mandelbrot set, into the PNG instead of a QR code. Compare a JavaScript implementation and a Rust/WASM implementation. Which code do you like better? Which one is faster?
 - Make the WebAssembly binary smaller, using tips from the [Rust WASM book](https://rustwasm.github.io/book/reference/code-size.html).
 - Right now, our QR code app generates data in Rust and manipulates DOM elements (e.g. setting the `<img>` source) in JavaScript. Try manipulating the DOM elements in Rust instead, by importing the JS functions like `document.getElementById` and then setting their properties.