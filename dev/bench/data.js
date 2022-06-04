window.BENCHMARK_DATA = {
  "lastUpdate": 1654337185980,
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
      }
    ]
  }
}