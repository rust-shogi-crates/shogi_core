window.BENCHMARK_DATA = {
  "lastUpdate": 1659692842816,
  "repoUrl": "https://github.com/rust-shogi-crates/shogi_core",
  "entries": {
    "Benchmark result (shogi_legality_lite)": [
      {
        "commit": {
          "author": {
            "email": "3303362+koba-e964@users.noreply.github.com",
            "name": "Hiroki Kobayashi",
            "username": "koba-e964"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "4345684813975ee7255514cacaca671dbad719f3",
          "message": "Merge pull request #30 from rust-shogi-crates/fix/publishing-benchmark-results\n\nFix GitHub Action (Rust)",
          "timestamp": "2022-06-03T23:16:14+09:00",
          "tree_id": "fd7f4e14934565f3b7998a79d93d4010e043667e",
          "url": "https://github.com/rust-shogi-crates/shogi_core/commit/4345684813975ee7255514cacaca671dbad719f3"
        },
        "date": 1654265838841,
        "tool": "cargo",
        "benches": [
          {
            "name": "bitboard::tests::pop_bench",
            "value": 86,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "bitboard::tests::shift_down_bench",
            "value": 19888,
            "range": "± 319",
            "unit": "ns/iter"
          },
          {
            "name": "bitboard::tests::shift_left_bench",
            "value": 18310,
            "range": "± 82",
            "unit": "ns/iter"
          },
          {
            "name": "bitboard::tests::shift_right_bench",
            "value": 19986,
            "range": "± 72",
            "unit": "ns/iter"
          },
          {
            "name": "bitboard::tests::shift_up_bench",
            "value": 19914,
            "range": "± 44",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "3303362+koba-e964@users.noreply.github.com",
            "name": "Hiroki Kobayashi",
            "username": "koba-e964"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "3ba8f5bbacb2b3e7503820607a16df960756027d",
          "message": "Merge pull request #31 from rust-shogi-crates/feature/bench\n\nAdd benchmark results to README",
          "timestamp": "2022-06-03T23:38:27+09:00",
          "tree_id": "257b977fb90edc4b170a893e7a6d5a2da8b9e49f",
          "url": "https://github.com/rust-shogi-crates/shogi_core/commit/3ba8f5bbacb2b3e7503820607a16df960756027d"
        },
        "date": 1654267187994,
        "tool": "cargo",
        "benches": [
          {
            "name": "bitboard::tests::pop_bench",
            "value": 91,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "bitboard::tests::shift_down_bench",
            "value": 20848,
            "range": "± 2416",
            "unit": "ns/iter"
          },
          {
            "name": "bitboard::tests::shift_left_bench",
            "value": 20090,
            "range": "± 2641",
            "unit": "ns/iter"
          },
          {
            "name": "bitboard::tests::shift_right_bench",
            "value": 21099,
            "range": "± 2966",
            "unit": "ns/iter"
          },
          {
            "name": "bitboard::tests::shift_up_bench",
            "value": 21674,
            "range": "± 3500",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "3303362+koba-e964@users.noreply.github.com",
            "name": "Hiroki Kobayashi",
            "username": "koba-e964"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "29b84c7400dfd0880b687c601b269bf72fdeba38",
          "message": "Merge pull request #32 from rust-shogi-crates/feature/optimize-bitboard\n\nOptimize `Bitboard`-related functions",
          "timestamp": "2022-06-04T17:51:13+09:00",
          "tree_id": "e7a39a4e6a970aa4fa5d71d68f8616ac9bc27dcb",
          "url": "https://github.com/rust-shogi-crates/shogi_core/commit/29b84c7400dfd0880b687c601b269bf72fdeba38"
        },
        "date": 1654332735591,
        "tool": "cargo",
        "benches": [
          {
            "name": "bitboard::tests::pop_bench",
            "value": 98,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "bitboard::tests::shift_down_bench",
            "value": 20253,
            "range": "± 67",
            "unit": "ns/iter"
          },
          {
            "name": "bitboard::tests::shift_left_bench",
            "value": 19035,
            "range": "± 85",
            "unit": "ns/iter"
          },
          {
            "name": "bitboard::tests::shift_right_bench",
            "value": 20540,
            "range": "± 86",
            "unit": "ns/iter"
          },
          {
            "name": "bitboard::tests::shift_up_bench",
            "value": 20297,
            "range": "± 58",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "3303362+koba-e964@users.noreply.github.com",
            "name": "Hiroki Kobayashi",
            "username": "koba-e964"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "c12e734eddd1a1be8c75618578789125809700e8",
          "message": "Merge pull request #33 from rust-shogi-crates/release/0.1.3\n\nBump version to 0.1.3",
          "timestamp": "2022-06-04T19:05:25+09:00",
          "tree_id": "5ebc505b9c514e74f4aa8e507c93fb73dbfd6b84",
          "url": "https://github.com/rust-shogi-crates/shogi_core/commit/c12e734eddd1a1be8c75618578789125809700e8"
        },
        "date": 1654337185064,
        "tool": "cargo",
        "benches": [
          {
            "name": "bitboard::tests::pop_bench",
            "value": 99,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "bitboard::tests::shift_down_bench",
            "value": 20487,
            "range": "± 712",
            "unit": "ns/iter"
          },
          {
            "name": "bitboard::tests::shift_left_bench",
            "value": 19249,
            "range": "± 4046",
            "unit": "ns/iter"
          },
          {
            "name": "bitboard::tests::shift_right_bench",
            "value": 20800,
            "range": "± 945",
            "unit": "ns/iter"
          },
          {
            "name": "bitboard::tests::shift_up_bench",
            "value": 20491,
            "range": "± 761",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "3303362+koba-e964@users.noreply.github.com",
            "name": "Hiroki Kobayashi",
            "username": "koba-e964"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "9875c498f6ef65a433e90cc242e789747f80d1e8",
          "message": "Merge pull request #38 from rust-shogi-crates/fix/tousi-for-move\n\nimpl ToUsi for Move",
          "timestamp": "2022-06-11T01:00:25+09:00",
          "tree_id": "66e75ab459a31280a793072f61d2a2262b9fdcf0",
          "url": "https://github.com/rust-shogi-crates/shogi_core/commit/9875c498f6ef65a433e90cc242e789747f80d1e8"
        },
        "date": 1654876896167,
        "tool": "cargo",
        "benches": [
          {
            "name": "bitboard::tests::pop_bench",
            "value": 98,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "bitboard::tests::shift_down_bench",
            "value": 23855,
            "range": "± 331",
            "unit": "ns/iter"
          },
          {
            "name": "bitboard::tests::shift_left_bench",
            "value": 22064,
            "range": "± 318",
            "unit": "ns/iter"
          },
          {
            "name": "bitboard::tests::shift_right_bench",
            "value": 24380,
            "range": "± 57",
            "unit": "ns/iter"
          },
          {
            "name": "bitboard::tests::shift_up_bench",
            "value": 23857,
            "range": "± 533",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "3303362+koba-e964@users.noreply.github.com",
            "name": "Hiroki Kobayashi",
            "username": "koba-e964"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "5a61a2348e206e5a7fa90b5a449b1e5260bc4f93",
          "message": "Merge pull request #39 from rust-shogi-crates/fix/occupied-bitboard\n\nAdd {Position, PartialPosition}::occupied_bitboard",
          "timestamp": "2022-06-11T01:16:21+09:00",
          "tree_id": "068e463c3a7c1503e2ebfa11fa8979248bf58b8a",
          "url": "https://github.com/rust-shogi-crates/shogi_core/commit/5a61a2348e206e5a7fa90b5a449b1e5260bc4f93"
        },
        "date": 1654877843041,
        "tool": "cargo",
        "benches": [
          {
            "name": "bitboard::tests::pop_bench",
            "value": 97,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "bitboard::tests::shift_down_bench",
            "value": 23521,
            "range": "± 1536",
            "unit": "ns/iter"
          },
          {
            "name": "bitboard::tests::shift_left_bench",
            "value": 21609,
            "range": "± 1485",
            "unit": "ns/iter"
          },
          {
            "name": "bitboard::tests::shift_right_bench",
            "value": 24528,
            "range": "± 823",
            "unit": "ns/iter"
          },
          {
            "name": "bitboard::tests::shift_up_bench",
            "value": 23667,
            "range": "± 1304",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "3303362+koba-e964@users.noreply.github.com",
            "name": "Hiroki Kobayashi",
            "username": "koba-e964"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "ebfb05ea326a336702f5bff1c4a43d20fb56e992",
          "message": "Merge pull request #40 from rust-shogi-crates/release/0.1.4\n\nBump version to 0.1.4",
          "timestamp": "2022-06-11T01:42:40+09:00",
          "tree_id": "4fd48c4358c1f93070432564f7a3c811b692c5ec",
          "url": "https://github.com/rust-shogi-crates/shogi_core/commit/ebfb05ea326a336702f5bff1c4a43d20fb56e992"
        },
        "date": 1654879421137,
        "tool": "cargo",
        "benches": [
          {
            "name": "bitboard::tests::pop_bench",
            "value": 99,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "bitboard::tests::shift_down_bench",
            "value": 20321,
            "range": "± 65",
            "unit": "ns/iter"
          },
          {
            "name": "bitboard::tests::shift_left_bench",
            "value": 19011,
            "range": "± 31",
            "unit": "ns/iter"
          },
          {
            "name": "bitboard::tests::shift_right_bench",
            "value": 20526,
            "range": "± 42",
            "unit": "ns/iter"
          },
          {
            "name": "bitboard::tests::shift_up_bench",
            "value": 20286,
            "range": "± 27",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "3303362+koba-e964@users.noreply.github.com",
            "name": "Hiroki Kobayashi",
            "username": "koba-e964"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "fa27c71eac55d20eef5c445dbdbf75d99102ede3",
          "message": "Merge pull request #43 from rust-shogi-crates/feature/clarify-assumptions\n\nAdd assumptions on returned values of `array_index()`",
          "timestamp": "2022-08-05T18:45:42+09:00",
          "tree_id": "ee103f9832cdf87fc89da52688bacfb233a7a986",
          "url": "https://github.com/rust-shogi-crates/shogi_core/commit/fa27c71eac55d20eef5c445dbdbf75d99102ede3"
        },
        "date": 1659692800950,
        "tool": "cargo",
        "benches": [
          {
            "name": "bitboard::tests::pop_bench",
            "value": 101,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "bitboard::tests::shift_down_bench",
            "value": 23826,
            "range": "± 303",
            "unit": "ns/iter"
          },
          {
            "name": "bitboard::tests::shift_left_bench",
            "value": 21971,
            "range": "± 992",
            "unit": "ns/iter"
          },
          {
            "name": "bitboard::tests::shift_right_bench",
            "value": 24013,
            "range": "± 677",
            "unit": "ns/iter"
          },
          {
            "name": "bitboard::tests::shift_up_bench",
            "value": 23882,
            "range": "± 1106",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "3303362+koba-e964@users.noreply.github.com",
            "name": "Hiroki Kobayashi",
            "username": "koba-e964"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "0bf8798ea31a0055bbe07dc3b0d780dabd077c53",
          "message": "Merge pull request #41 from rust-shogi-crates/fix/sfen-typo\n\nSEFN -> SFEN",
          "timestamp": "2022-08-05T18:46:34+09:00",
          "tree_id": "ccdf6b2687f5030a9388ac4f275cdbdfd4ce9e78",
          "url": "https://github.com/rust-shogi-crates/shogi_core/commit/0bf8798ea31a0055bbe07dc3b0d780dabd077c53"
        },
        "date": 1659692841943,
        "tool": "cargo",
        "benches": [
          {
            "name": "bitboard::tests::pop_bench",
            "value": 96,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "bitboard::tests::shift_down_bench",
            "value": 20221,
            "range": "± 50",
            "unit": "ns/iter"
          },
          {
            "name": "bitboard::tests::shift_left_bench",
            "value": 19012,
            "range": "± 49",
            "unit": "ns/iter"
          },
          {
            "name": "bitboard::tests::shift_right_bench",
            "value": 20532,
            "range": "± 58",
            "unit": "ns/iter"
          },
          {
            "name": "bitboard::tests::shift_up_bench",
            "value": 20274,
            "range": "± 51",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}