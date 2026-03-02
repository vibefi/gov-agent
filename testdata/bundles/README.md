This folder contains vapp bundles which should result in low scores. 


DO NOT RUN ANYTHING IN HERE.


## malicious_uniswapv2

Content taken from commit 762c7a9bbe861aa7723ddd9f8b6e41b611674546 `vibefi/dapp-examples/uniswap-v2`.

It contains only one malicious rogue send in App.tsx#L316. Instead of swapping tokens, it just sends the amount to a random address. 

To avoid committing the raw fixture tree, this bundle is stored as:
- `malicious_uniswapv2.tar.gz`

The e2e helper (`e2e/src/publish-test-bundle.ts`) extracts the archive at runtime when `malicious_uniswapv2/` is not present.

## red_team_vapp

Obviously malicious app.

To avoid committing the raw fixture tree, this bundle is stored as:
- `red_team_vapp.tar.gz`

Expected signals:
- risky package scripts (`curl ... | bash`)
- suspicious source tokens (`child_process`, `eval(`, external HTTP)
- suspicious manifest path traversal marker (`../`)

This fixture is intentionally unsafe and should drive low-confidence / reject behavior.
