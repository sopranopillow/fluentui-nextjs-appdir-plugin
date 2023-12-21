# [@Fluent UI v9](https://react.fluentui.dev/) NextJS AppDir router support plugin

This plugin allows the use of `@fluentui/react-components` and `@griffel` in the new NextJS@14 appDir router. The goal of this plugin is to add the `"use client";` directive to all files in `@fluentui/react-components` and `@griffel`, but note it's not limited to these libraries.

### Installation

```sh
# yarn
yarn add fluentui-next-appdir-directive

# npm
npm i fluentui-next-appdir-directive
```

### Configuration for `@fluentui/react-components` and `@griffel`:

```js
// next.config.js

/** @type {import('next').NextConfig} */
const nextConfig = {
  experimental: {
    swcPlugins: [
      ["fluentui-next-appdir-directive",{
         paths: [
          "@griffel",
          "@fluentui"
          // 👇 you can add another dependency that needs the directive
          "your dependency name"
        ]
      }],
    ],
  },
};

module.exports = nextConfig;
```

### Configuration for usage outside of NextJS

```json
// .swcrc
{
  "jsc": {
    "experimental": {
      "plugins": [
        ["fluentui-next-appdir-directive", {
          "paths": [
            "@griffel",
            "@fluentui"
             // 👇 you can add another dependency that needs the directive
             "your dependency name"
          ]
        }]
      ]
    }
  }
}
```

> Note: strings inside paths should only contain the scope/package name, in our case @fluentui/react-components -> @fluentui
