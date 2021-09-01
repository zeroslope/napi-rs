const { rkyv, serdeString } = require('./ser.node')
const {serialize} = require('./serializer-wasm/pkg')

const a = rkyv()
const b = serialize(new Uint8Array(a.buffer))

console.time('wasm')
const c = rkyv()
const d = serialize(new Uint8Array(c.buffer))
console.timeEnd('wasm')
// console.log(b)

console.time('SerdeJson')
const str = serdeString()
JSON.parse(str)
console.timeEnd('SerdeJson')
