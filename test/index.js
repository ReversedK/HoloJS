const path = require('path')
const tape = require('tape')

const { Diorama, tapeExecutor, backwardCompatibilityMiddleware } = require('@holochain/diorama')

process.on('unhandledRejection', error => {
  // Will print "unhandledRejection err is not defined"
  console.error('got unhandledRejection:', error);
});

const dnaPath = path.join(__dirname, "../dist/orm.dna.json")
const dna = Diorama.dna(dnaPath, 'holojs')

const diorama = new Diorama({
  instances: {
    alice: dna,
    bob: dna,
    carol: dna,
  },
  bridges: [
    Diorama.bridge('test-bridge', 'alice', 'bob')
  ],
  debugLog: false,
  executor: tapeExecutor(require('tape')),
  middleware: backwardCompatibilityMiddleware,
})

//require('./regressions')(diorama.registerScenario)
require('./tests')(diorama.registerScenario)

diorama.run()