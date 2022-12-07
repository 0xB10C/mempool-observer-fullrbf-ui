# mempool-observer-fullrbf-ui

Quick and dirty custom static site generator showing full-RBF replacements and their block inclusions.

Usage: `mempool-observer-fullrbf-ui <path/to/*.csv> <html output dir>`.

Takes an CSV file with replacements events in the following format as input and produces a set of HTML files showing information about the **full-RBF** replacement events.

```CSV
timestamp,replaced_txid,replaced_fee,replaced_vsize,replaced_raw,replacement_txid,replacement_fee,replacement_vsize,replacement_raw
1670314778,732deec5209fdeee8136053fc67254e580e07dc52415ec28bca792bb0447004c,8354,110,020000000001011580e7b64d77bee246ac3f241bdb277076e64b737f865eae65454df82f14133f1100000000fdffffff0199d4dc0400000000160014afde86add624371ad0d648387f56865d197e54eb024730440220602149230fbfc4abc265077d014a4eac94f40a46869386bd7305bf145c45e30f022009021d1f0068aba6860ba834f30d1d1f146d4363744351f4ad97ac92602423dd012102cc35398135669fe23e89d67acddb0b9dc227d384d5044cc41a87b932c2456efc00000000,ef3f9c361278eb12d5b0fe70911871fa114f969e2c3b96889df8bcc2be384551,12469,110,020000000001011580e7b64d77bee246ac3f241bdb277076e64b737f865eae65454df82f14133f1100000000fdffffff0186c4dc040000000016001416f15047033aff1809b75b39c190b7094af3bea00247304402204604a788f311045f4182609c9a3369f85f6c86a93ba6f2fa1b4ff257c96eec890220390f8a0bd537dc61edc651b295b28370abf93ae56137626d0fd4bc0cb8be99e3012102cc35398135669fe23e89d67acddb0b9dc227d384d5044cc41a87b932c2456efc00000000
```

This is intended as a temporary installment and e.g. doesn't include proper error handling or setup instructions for others.
