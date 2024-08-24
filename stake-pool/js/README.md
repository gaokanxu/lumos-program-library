# TypeScript bindings for stake-pool program

For use with both node.js and in-browser.

## Installation

```
npm install
```

## Build and run

In the `js` folder:

```
npm run build
```

The build is available at `dist/index.js` (or `dist.browser/index.iife.js` in the browser).

## Browser bundle
```html
<!-- Development (un-minified) -->
<script src="https://unpkg.com/@lumos/spl-stake-pool@latest/dist.browser/index.iife.js"></script>

<!-- Production (minified) -->
<script src="https://unpkg.com/@lumos/spl-stake-pool@latest/dist.browser/index.iife.min.js"></script>
```

## Test

```
npm test
```

## Usage

### JavaScript
```javascript
const lumosStakePool = require('@lumos/spl-stake-pool');
console.log(lumosStakePool);
```

### ES6
```javascript
import * as lumosStakePool from '@lumos/spl-stake-pool';
console.log(lumosStakePool);
```

### Browser bundle
```javascript
// `lumosStakePool` is provided in the global namespace by the script bundle.
console.log(lumosStakePool);
```
