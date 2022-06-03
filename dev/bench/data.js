window.BENCHMARK_DATA = {
  "lastUpdate": 1654267188420,
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
      }
    ]
  }
}