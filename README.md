<p align="center">
  <img src="https://github.com/clojure-rs/ClojureRS/blob/master/logo/clojureRS-logo-horizontal.png" alt="ClojureRS Logo"/>
</p>

<p align="center"><b>Put simply, Clojure implemented atop Rust! For now, a Clojure interpreter implemented in Rust.</b></p>

<p align="center"> 
<img src="https://img.shields.io/github/workflow/status/clojure-rs/ClojureRS/Rust"></a>
<a href="https://discord.gg/mFE3JNr"><img src="https://img.shields.io/discord/703549047901913189?logo=discord"></a>
</p>

<br>

![REPL](https://i.imgur.com/rpioVBP.png)

<p align="center"><i>The REPL in action!</i></p>

This is my fork look up the original project!

## Features:
- [x] Added Lazy Sequences and `lazy-seq` core function
- [ ] Automatically load [core.clj](./src/clojure/core.clj) and [string.clj](./src/clojure/string.clj) when starting repl environment
- [x] Fixed error with calls to funcs with varargs
- [ ] Implement multi argument functions with `defn`
- [ ] Maybe anonymous reader macro?
