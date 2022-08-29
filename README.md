# markdown_records

A minimalistic library to transform arbitrary markdown into a set of markdown
records. Intended to be able to feed search engines structured data from
markdown.

## Usage

```ts
import { toRecords } from "https://deno.land/x/markdown_records/mod.ts";

const records = await toRecords(`# An example

Markdown content...

\`\`\`ts
/** Example code */
const a = "a";
\`\`\`
`);

console.log(records); // view the structured data
```

---

Copyright 2022 Kitson P. Kelly. All rights reserved. MIT Licensed.
