# [@Fluent UI v9](https://react.fluentui.dev/) NextJS AppDir router support plugin

This plugin allows the use of `@fluentui/react-components` and `@griffel` in the new NextJS@13+ appDir router. The goal of this plugin is to add the `"use client";` directive to all files in `@fluentui/react-components` and `@griffel`, but note it's not limited to these libraries.

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

### My modules are getting ignored and gives me an error in NextJS 14.1.2+:

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
  // 👇 packages that need the directive
  transpilePackages: ["@fluentui/react-components"]
};

module.exports = nextConfig;
```

### Version compat

- For versions `< 14.1.2` please use `0.1.5` otherwise there wil be issues with the swc_core version causing rust to panic.
- For versions `>= 14.1.2` please use `0.1.7` for the same reason as above, `0.1.7` introduced version `0.90.*` of swc_core which makes it compatible with the versions used in `<= 14.1.2`.
- For versions `>= 15.0.0` please use `0.2.0` due to a necessary swc_core bump.

### My packages aren't getting tree-shaken

This is a known issue of using this directive, one way to get around it is to optimizePackage/modularizeImports imports:

- https://nextjs.org/docs/app/api-reference/next-config-js/optimizePackageImports
- https://nextjs.org/docs/architecture/nextjs-compiler#modularize-imports
