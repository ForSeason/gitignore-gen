# gitignore-gen

Smart `.gitignore` generator written in Rust using [napi-rs](https://github.com/napi-rs/napi-rs).

## Build

```bash
npm install -g @napi-rs/cli
npm run build
```

## Example

```javascript
const { generateGitignore } = require('./');
const base = `
# Node
node_modules/
npm-debug.log*
`;

const cfg = { ignoreDirs: ['dist'], focusDir: 'src' };
console.log(generateGitignore(base, cfg));
```
