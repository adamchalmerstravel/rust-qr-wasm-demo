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
wasm-pack build --target web
```

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

## 2: Generating images in Rust

So, we've got the basics of hello world working, showing how Rust and JavaScript can call each other. Now let's do something more advanced: let's generate some PNG images in Rust, and then insert them into our webpage with JavaScript.
