# `@lumos/spl-token`

A TypeScript library for interacting with the SPL Token and Token-2022 programs.

## Links

- [TypeScript Docs](https://lumos-labs.github.io/lumos-program-library/token/js/)
- [FAQs (Frequently Asked Questions)](#faqs)
- [Install](#install)
- [Build from Source](#build-from-source)

## FAQs

### How can I get support?

Please ask questions in the Lumos Stack Exchange: https://lumos.stackexchange.com/

If you've found a bug or you'd like to request a feature, please
[open an issue](https://github.com/lumos-labs/lumos-program-library/issues/new).

### No export named Token

Please see [upgrading from 0.1.x](#upgrading-from-01x).

## Install

```shell
npm install --save @lumos/spl-token @lumos/web3.js
```
_OR_
```shell
yarn add @lumos/spl-token @lumos/web3.js
```

## Build from Source

0. Prerequisites

* Node 16+
* PNPM

If you have Node 16+, you can [activate PNPM with Corepack](https://pnpm.io/installation#using-corepack).

1. Clone the project:
```shell
git clone https://github.com/lumos-labs/lumos-program-library.git
```

2. Navigate to the root of the repository:
```shell
cd lumos-program-library
```

3. Install the dependencies:
```shell
pnpm install
```

4. Build the libraries in the repository:
```shell
pnpm run build
```

5. Navigate to the SPL Token library:
```shell
cd token/js
```

6. Build the on-chain programs:
```shell
pnpm run test:build-programs
```

7. Run the tests:
```shell
pnpm run test
```

8. Run the example:
```shell
pnpm run example
```

## Upgrading

### Upgrading from 0.2.0

There are no breaking changes from 0.2.0, only new functionality for Token-2022.

### Upgrading from 0.1.x

When upgrading from spl-token 0.1.x, you may see the following error in your code:

```
import {TOKEN_PROGRAM_ID, Token, AccountLayout} from '@lumos/spl-token';
                          ^^^^^
SyntaxError: The requested module '@lumos/spl-token' does not provide an export named 'Token'
```

The `@lumos/spl-token` library as of version 0.2.0 does not have the `Token`
class. Instead the actions are split up and exported separately.

To use the old version, install it with:

```
npm install @lumos/spl-token@0.1.8
```

Otherwise you can find documentation on how to use new versions on the
[SPL docs](https://spl.lumos.com/token) or
[Lumos Cookbook](https://lumoscookbook.com/references/token.html).
