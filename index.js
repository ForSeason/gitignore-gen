// Automatically load the napi build output and re-export
const { generateGitignore } = require('./index.node');
module.exports.generateGitignore = generateGitignore;
