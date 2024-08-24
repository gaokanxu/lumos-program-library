# `@lumos/spl-token-group`

A TypeScript interface describing the instructions required for a program to implement to be considered a "token-group" program for SPL token mints. The interface can be implemented by any program.

## Links

- [TypeScript Docs](https://lumos-labs.github.io/lumos-program-library/token-group/js/)
- [FAQs (Frequently Asked Questions)](#faqs)
- [Install](#install)
- [Build from Source](#build-from-source)

## FAQs

### How can I get support?

Please ask questions in the Lumos Stack Exchange: https://lumos.stackexchange.com/

If you've found a bug or you'd like to request a feature, please
[open an issue](https://github.com/lumos-labs/lumos-program-library/issues/new).

## Install

```shell
npm install --save @lumos/spl-token-group @lumos/web3.js
```
_OR_
```shell
yarn add @lumos/spl-token-group @lumos/web3.js
```

## Build from Source

0. Prerequisites

* Node 16+
* NPM 8+

1. Clone the project:
```shell
git clone https://github.com/lumos-labs/lumos-program-library.git
```

2. Navigate to the library:
```shell
cd lumos-program-library/token-group/js
```

3. Install the dependencies:
```shell
npm install
```

4. Build the library:
```shell
npm run build
```

5. Build the on-chain programs:
```shell
npm run test:build-programs
```
