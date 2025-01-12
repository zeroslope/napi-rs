const configuration = {
  extensions: ['ts', 'tsx'],
  files: ['cli/**/*.spec.ts', 'examples/**/__test__/**/*.spec.ts'],
  require: ['ts-node/register/transpile-only'],
  environmentVariables: {
    TS_NODE_PROJECT: './examples/tsconfig.json',
  },
  timeout: '1m',
  workerThreads: false,
}

if (parseInt(process.versions.napi, 10) < 4) {
  configuration.compileEnhancements = false
}

export default configuration
