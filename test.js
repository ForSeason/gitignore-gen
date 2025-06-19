const { generateGitignore } = require('./');
const assert = require('assert');

const base = `
# Node
node_modules/
npm-debug.log*
`.trim();

function runTest(name, cfg, expected) {
  const out = generateGitignore(base, cfg);
  try {
    assert.strictEqual(out, expected);
    console.log(`\u2713 ${name}`);
  } catch (err) {
    console.error(`\u2717 ${name}`);
    console.error('Expected:\n' + expected);
    console.error('Got:\n' + out);
    process.exitCode = 1;
  }
}

runTest('focus + ignore', { ignoreDirs: ['dist', 'logs/temp'], focusDir: 'src' },
`/*
!.gitignore
!/src/
!/src/**

/dist/
/logs/temp/

# Node
node_modules/
npm-debug.log*
`);

runTest('ignore only', { ignoreDirs: ['dist', 'logs/temp'], focusDir: undefined },
`/dist/
/logs/temp/

# Node
node_modules/
npm-debug.log*
`);

runTest('nested focus dedup', { ignoreDirs: ['dist', 'dist'], focusDir: 'src/app' },
`/*
!.gitignore
!/src/
!/src/app/
!/src/app/**

/dist/

# Node
node_modules/
npm-debug.log*
`);

runTest('base only', { ignoreDirs: [], focusDir: '' },
`# Node
node_modules/
npm-debug.log*
`);
