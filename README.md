# [@Fluent UI v9](https://react.fluentui.dev/) NextJS AppDir router support plugin

This plugin allows the use of `@fluentui/react-components` and `@griffel` in the new NextJS@13 appDir router. The goal of this plugin is to add the `"use client";` directive to all files in `@fluentui/react-components` and `@griffel`. Note that this plugin is not limited to only work with Fluent UI, it can be used with other libraries as well based on the provided configuration. This plugin will turn a file like this:

```jsx
import {
  FluentProvider,
  webLightTheme,
  Button,
} from "@fluentui/react-components";

export const App = () => {
  return (
    <FluentProvider theme={webLightTheme}>
      <Button>Hello from NextJS@13</Button>
    </FluentProvider>
  );
};
```

to:

```jsx
"use client";

import {
  FluentProvider,
  webLightTheme,
  Button,
} from "@fluentui/react-components";

export const App = () => {
  return (
    <FluentProvider theme={webLightTheme}>
      <Button>Hello from NextJS@13</Button>
    </FluentProvider>
  );
};
```
