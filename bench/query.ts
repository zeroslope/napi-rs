import { promisify } from 'util'

import b from 'benny'

const { query, engine: napiEngine } = require('./index.node')
const { engineNew, engineQuery } = require('./neon.node')

const engine = engineNew('model A {}')
const engineQueryAsync = promisify(engineQuery.bind(engine))

const e = napiEngine('model A {}')

export const benchQuery = () =>
  b.suite(
    'Query',
    b.add('napi-rs', async () => {
      await query(e)
    }),
    b.add('neon', async () => {
      await engineQueryAsync()
    }),

    b.cycle(),
    b.complete(),
  )
