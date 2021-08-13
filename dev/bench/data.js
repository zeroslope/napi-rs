window.BENCHMARK_DATA = {
  lastUpdate: 1628860372248,
  repoUrl: 'https://github.com/napi-rs/napi-rs',
  entries: {
    Benchmark: [
      {
        commit: {
          author: {
            email: 'lynweklm@gmail.com',
            name: 'LongYinan',
            username: 'Brooooooklyn',
          },
          committer: {
            email: 'noreply@github.com',
            name: 'GitHub',
            username: 'web-flow',
          },
          distinct: true,
          id: 'a2806f59fe45a26ce77a77f1d24a6d7a759db6fc',
          message:
            'Merge pull request #697 from napi-rs/fix-utf16\n\nfix(napi): utf16 value should not contains 0 terminate',
          timestamp: '2021-08-13T21:08:47+08:00',
          tree_id: '33045b0689c0e11a3d19d1e3931b8522df2e93ce',
          url: 'https://github.com/napi-rs/napi-rs/commit/a2806f59fe45a26ce77a77f1d24a6d7a759db6fc',
        },
        date: 1628860370572,
        tool: 'benchmarkjs',
        benches: [
          {
            name: 'noop#napi-rs',
            value: 50329113,
            range: '±1.05%',
            unit: 'ops/sec',
            extra: '91 samples',
          },
          {
            name: 'noop#JavaScript',
            value: 693685138,
            range: '±1.14%',
            unit: 'ops/sec',
            extra: '86 samples',
          },
          {
            name: 'Plus number#napi-rs',
            value: 19983299,
            range: '±1.26%',
            unit: 'ops/sec',
            extra: '87 samples',
          },
          {
            name: 'Plus number#JavaScript',
            value: 720151164,
            range: '±0.86%',
            unit: 'ops/sec',
            extra: '88 samples',
          },
          {
            name: 'Create buffer#napi-rs',
            value: 349702,
            range: '±8.8%',
            unit: 'ops/sec',
            extra: '70 samples',
          },
          {
            name: 'Create buffer#JavaScript',
            value: 2190976,
            range: '±8.88%',
            unit: 'ops/sec',
            extra: '82 samples',
          },
          {
            name: 'createArray#createArrayJson',
            value: 33799,
            range: '±0.79%',
            unit: 'ops/sec',
            extra: '93 samples',
          },
          {
            name: 'createArray#create array for loop',
            value: 7168,
            range: '±0.88%',
            unit: 'ops/sec',
            extra: '93 samples',
          },
          {
            name: 'createArray#create array with serde trait',
            value: 7206,
            range: '±1%',
            unit: 'ops/sec',
            extra: '93 samples',
          },
          {
            name: 'getArrayFromJs#get array from json string',
            value: 16264,
            range: '±1.22%',
            unit: 'ops/sec',
            extra: '87 samples',
          },
          {
            name: 'getArrayFromJs#get array from serde',
            value: 9592,
            range: '±1.09%',
            unit: 'ops/sec',
            extra: '86 samples',
          },
          {
            name: 'getArrayFromJs#get array with for loop',
            value: 12293,
            range: '±0.85%',
            unit: 'ops/sec',
            extra: '92 samples',
          },
          {
            name: 'Get Set property#Get Set from native#u32',
            value: 373605,
            range: '±5.2%',
            unit: 'ops/sec',
            extra: '75 samples',
          },
          {
            name: 'Get Set property#Get Set from JavaScript#u32',
            value: 329588,
            range: '±5.18%',
            unit: 'ops/sec',
            extra: '74 samples',
          },
          {
            name: 'Get Set property#Get Set from native#string',
            value: 330988,
            range: '±5.21%',
            unit: 'ops/sec',
            extra: '73 samples',
          },
          {
            name: 'Get Set property#Get Set from JavaScript#string',
            value: 316839,
            range: '±5%',
            unit: 'ops/sec',
            extra: '77 samples',
          },
          {
            name: 'Async task#spawn task',
            value: 39762,
            range: '±2.07%',
            unit: 'ops/sec',
            extra: '74 samples',
          },
          {
            name: 'Async task#ThreadSafeFunction',
            value: 463,
            range: '±8.03%',
            unit: 'ops/sec',
            extra: '47 samples',
          },
          {
            name: 'Async task#Tokio future to Promise',
            value: 26875,
            range: '±2.15%',
            unit: 'ops/sec',
            extra: '74 samples',
          },
          {
            name: 'Query#query * 100',
            value: 2083,
            range: '±1.84%',
            unit: 'ops/sec',
            extra: '84 samples',
          },
          {
            name: 'Query#query * 1',
            value: 30714,
            range: '±2.34%',
            unit: 'ops/sec',
            extra: '80 samples',
          },
        ],
      },
    ],
  },
}
