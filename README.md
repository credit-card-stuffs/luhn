# Luhn

This package is a port of a very good existing package called [fast-luhn](https://github.com/cybermatt/fast-luhn) created by [@cybermatt](https://github.com/cybermatt).

It runs in NodeJs/Browser environments through [WebAssembly](https://rustwasm.github.io/wasm-pack/), with very good speed and contains the necessary functions to work with the [luhn algorithm](https://en.wikipedia.org/wiki/Luhn_algorithm) quickly and efficiently.

## Overview

Below is an overview of the package.

#### Install

Package can be installed using npm cli

```
npm i @credit-card-stuffs/luhn
```

#### Import

You can import like any other npm package

```ts
import luhn from "@credit-card-stuffs/luhn"
```

#### Validate

Check if string is valid by luhn algorithm. Return bool.

```ts
luhn.validate("471629309440")
// false
```

#### Digit

Calculate next digit for string of numbers. Return int.

```ts
luhn.digit("47162930944")
// 7
```

#### Complete

Add luhn-check digit to string of numbers. Return string.

```ts
luhn.complete("2398560146")
// 23985601469
```

#### Generate

Generate luhn-valid string of numbers with length. Return string.

```ts
luhn.generate(50)
// 04648626855744999947592497373533751558978979404719
```

#### Tests

Tests can be run with command

```sh
$ cargo test
```

#### Building

You can build the package from source code

##### Dependencies

First of all you need to resolve some dependencies to get started.

-   1. Rust
-   2. NodeJs
-   3. Wasm-pack

##### Build

```sh
$ git clone https://github.com/credit-card-stuffs/luhn
$ cd luhn
$ wasm-pack build --target nodejs
```

After that you should see a folder called `pkg/` in which it contains the generated Js files.
