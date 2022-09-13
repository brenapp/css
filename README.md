![css](https://user-images.githubusercontent.com/8839926/189790296-93dcb95b-da7f-46a0-a684-38b0abd3bc83.png)


An easy-to-follow CSS parser and minifier written in Rust, with no dependancies! This module takes a CSS file as input, will tokenize and parse the source code for parse errors, and output a parallel `.min.css` file with a minified source graph.

Consider the following `main.css` file:
```css
.App-Header {
  height: 8vh;
  display: flex;
  color: #fff;
  align-items: center;
  justify-content: space-between;
}
```

Calling minify here will generate a `main.min.css` file like so:

```css
.App-Header { height: 8vh; display: flex; color: #fff; align-items: center; justify-content: space-between; } 
```

> Please note: This project was made in my free time and for fun. While I did my best to follow the parsing spec, there is no gaurentee of spec compliance. There are no compliance or regression tests. If you need something like this for real uses, I would reccomend you use [Servo's CSS Parser](https://github.com/servo/rust-cssparser/)
