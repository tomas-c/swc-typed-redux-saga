# SWC plugin for typed-redux-saga that works with Next.js

[![npm](https://img.shields.io/npm/v/swc-plugin-typed-redux-saga.svg)](https://www.npmjs.com/package/swc-plugin-typed-redux-saga)

This plugin adds [typed-redux-saga](https://github.com/agiledigital/typed-redux-saga) macro support for SWC.

Tested with Next.js version v14.2.2


## Installation

Install the package:

```
# yarn
yarn add swc-plugin-typed-redux-saga
```

or

```
# npm
npm install swc-plugin-typed-redux-saga
```

And update next.config.js:


```javascript
// next.config.js

module.exports = {
  experimental: {
    swcPlugins: [
      [require.resolve("swc-plugin-typed-redux-saga"), {}]
    ],
  }
}

```

