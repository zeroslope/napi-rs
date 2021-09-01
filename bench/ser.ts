import b from 'benny'

const { serdeString, serdeBuffer, rkyv } = require('../ser.node')

export const benchSer = () =>
  b.suite(
    'Ser',
    b.add('serde string', () => {
      JSON.parse(serdeString())
    }),
    b.add('serde buffer', () => {
      JSON.parse(serdeBuffer().toString('utf8'))
    }),

    b.add('rkyv', () => {
      rkyv()
    }),

    b.cycle(),
    b.complete(),
  )
