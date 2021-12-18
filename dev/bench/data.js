window.BENCHMARK_DATA = {
  lastUpdate: 1639806886480,
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
          id: '3f2e44d3db0c3e74a8b86d88bbb40422afd22ff9',
          message:
            'Merge pull request #939 from napi-rs/fix/buffer-vec-conversion\n\nfix(napi): impl From<Buffer> for Vec<u8>',
          timestamp: '2021-12-18T13:41:40+08:00',
          tree_id: 'e861d46b2f80726ada50edb0f6826bdfde44d33b',
          url: 'https://github.com/napi-rs/napi-rs/commit/3f2e44d3db0c3e74a8b86d88bbb40422afd22ff9',
        },
        date: 1639806885416,
        tool: 'benchmarkjs',
        benches: [
          {
            name: 'noop#napi-rs',
            value: 54523049,
            range: '±0.44%',
            unit: 'ops/sec',
            extra: '91 samples',
          },
          {
            name: 'noop#JavaScript',
            value: 713295816,
            range: '±0.17%',
            unit: 'ops/sec',
            extra: '96 samples',
          },
          {
            name: 'Plus number#napi-rs',
            value: 18674595,
            range: '±0.24%',
            unit: 'ops/sec',
            extra: '95 samples',
          },
          {
            name: 'Plus number#JavaScript',
            value: 711918126,
            range: '±0.13%',
            unit: 'ops/sec',
            extra: '94 samples',
          },
          {
            name: 'Create buffer#napi-rs',
            value: 374463,
            range: '±8.38%',
            unit: 'ops/sec',
            extra: '61 samples',
          },
          {
            name: 'Create buffer#JavaScript',
            value: 1789795,
            range: '±3.83%',
            unit: 'ops/sec',
            extra: '80 samples',
          },
          {
            name: 'createArray#createArrayJson',
            value: 38953,
            range: '±0.09%',
            unit: 'ops/sec',
            extra: '95 samples',
          },
          {
            name: 'createArray#create array for loop',
            value: 7875,
            range: '±0.1%',
            unit: 'ops/sec',
            extra: '98 samples',
          },
          {
            name: 'createArray#create array with serde trait',
            value: 7811,
            range: '±0.1%',
            unit: 'ops/sec',
            extra: '98 samples',
          },
          {
            name: 'getArrayFromJs#get array from json string',
            value: 16561,
            range: '±0.7%',
            unit: 'ops/sec',
            extra: '94 samples',
          },
          {
            name: 'getArrayFromJs#get array from serde',
            value: 10560,
            range: '±0.06%',
            unit: 'ops/sec',
            extra: '100 samples',
          },
          {
            name: 'getArrayFromJs#get array with for loop',
            value: 12504,
            range: '±0.07%',
            unit: 'ops/sec',
            extra: '99 samples',
          },
          {
            name: 'Get Set property#Get Set from native#u32',
            value: 420323,
            range: '±4.61%',
            unit: 'ops/sec',
            extra: '78 samples',
          },
          {
            name: 'Get Set property#Get Set from JavaScript#u32',
            value: 354440,
            range: '±4.49%',
            unit: 'ops/sec',
            extra: '82 samples',
          },
          {
            name: 'Get Set property#Get Set from native#string',
            value: 364001,
            range: '±4.35%',
            unit: 'ops/sec',
            extra: '80 samples',
          },
          {
            name: 'Get Set property#Get Set from JavaScript#string',
            value: 322337,
            range: '±4.67%',
            unit: 'ops/sec',
            extra: '82 samples',
          },
          {
            name: 'Async task#spawn task',
            value: 37824,
            range: '±1.25%',
            unit: 'ops/sec',
            extra: '84 samples',
          },
          {
            name: 'Async task#ThreadSafeFunction',
            value: 438,
            range: '±3.8%',
            unit: 'ops/sec',
            extra: '65 samples',
          },
          {
            name: 'Async task#Tokio future to Promise',
            value: 29266,
            range: '±1.11%',
            unit: 'ops/sec',
            extra: '88 samples',
          },
          {
            name: 'Query#query * 100',
            value: 2170,
            range: '±1.61%',
            unit: 'ops/sec',
            extra: '89 samples',
          },
          {
            name: 'Query#query * 1',
            value: 30196,
            range: '±2.1%',
            unit: 'ops/sec',
            extra: '85 samples',
          },
        ],
      },
    ],
  },
}
