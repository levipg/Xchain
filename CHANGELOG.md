# Change Log

## [v0.2.3](https://github.com/youtubaijia/xchain/tree/v0.2.3) (2019-06-23)
[Full Changelog](https://github.com/youtubaijia/xchain/compare/v0.2.2...v0.2.3)

**Merged pull requests:**

- Move JCLI's genesis into xchain-lib [\#560](https://github.com/youtubaijia/xchain/pull/560)
- Proposal to replace ENTRYPOINT with CMD in Dockerfile [\#559](https://github.com/youtubaijia/xchain/pull/559)

## [v0.2.2](https://github.com/youtubaijia/xchain/tree/v0.2.2) (2019-06-21)
[Full Changelog](https://github.com/youtubaijia/xchain/compare/v0.2.1...v0.2.2)

**Closed issues:**

- jcli 0.2.1 \[jcli key generate --type\]  [\#501](https://github.com/youtubaijia/xchain/issues/501)
- REST account API: The delegation field format should be improved [\#491](https://github.com/youtubaijia/xchain/issues/491)
- gelf logging support for slog [\#447](https://github.com/youtubaijia/xchain/issues/447)

**Merged pull requests:**

- mark gelf as optional feature [\#557](https://github.com/youtubaijia/xchain/pull/557)
- Fix incorrect PATH setting [\#555](https://github.com/youtubaijia/xchain/pull/555)
- Update introduction.md [\#552](https://github.com/youtubaijia/xchain/pull/552)
- Update delegating\_stake.md [\#550](https://github.com/youtubaijia/xchain/pull/550)
- add more documentation [\#547](https://github.com/youtubaijia/xchain/pull/547)
- UTxO Info as a common interface between jcli, xchain and the tests [\#543](https://github.com/youtubaijia/xchain/pull/543)
- move to chain-deps [\#542](https://github.com/youtubaijia/xchain/pull/542)
- ignore unstable test [\#539](https://github.com/youtubaijia/xchain/pull/539)
- remove non needed reference [\#538](https://github.com/youtubaijia/xchain/pull/538)
- fix the path to the default genesis block in the documentation [\#537](https://github.com/youtubaijia/xchain/pull/537)
- add hex-crate to replace xblockchain::util::hex [\#534](https://github.com/youtubaijia/xchain/pull/534)
- account state for both xchain, jcli and tests [\#532](https://github.com/youtubaijia/xchain/pull/532)
- Revert "New corner cases for transaction module" [\#531](https://github.com/youtubaijia/xchain/pull/531)
- Add script for create account and delegating with it [\#529](https://github.com/youtubaijia/xchain/pull/529)
- provide more details on the error if available [\#528](https://github.com/youtubaijia/xchain/pull/528)
- Move genesis.yaml initial state to single list [\#527](https://github.com/youtubaijia/xchain/pull/527)
- Fix for soak test after \#522 [\#526](https://github.com/youtubaijia/xchain/pull/526)
- Documentation update [\#524](https://github.com/youtubaijia/xchain/pull/524)
- Unify xchain API Types to allow better reusability [\#522](https://github.com/youtubaijia/xchain/pull/522)
- Update introduction.md [\#521](https://github.com/youtubaijia/xchain/pull/521)
- bootstrap: fix printed example command [\#519](https://github.com/youtubaijia/xchain/pull/519)
- Changing function declaration to POSIX-syntax [\#518](https://github.com/youtubaijia/xchain/pull/518)
- Increase logger async buffer from 128 to 1024 entries [\#509](https://github.com/youtubaijia/xchain/pull/509)
- removing dup getopts d [\#504](https://github.com/youtubaijia/xchain/pull/504)
- jcli: fix key type in help message ed25510bip32 -\> ed25519bip32 [\#502](https://github.com/youtubaijia/xchain/pull/502)
- adding a few more flags/options to the bootstrap script [\#498](https://github.com/youtubaijia/xchain/pull/498)
- Support GELF logging output. [\#497](https://github.com/youtubaijia/xchain/pull/497)
- Remove todo in quickstart section about P2P [\#486](https://github.com/youtubaijia/xchain/pull/486)
- Test stability fix for transaction test cases [\#483](https://github.com/youtubaijia/xchain/pull/483)

## [v0.2.1](https://github.com/youtubaijia/xchain/tree/v0.2.1) (2019-06-15)
[Full Changelog](https://github.com/youtubaijia/xchain/compare/v0.2.0...v0.2.1)

**Fixed bugs:**

- output and format of logger defined in config yaml is ignored [\#494](https://github.com/youtubaijia/xchain/issues/494)
- jcli transaction id not changing when adding certificate [\#475](https://github.com/youtubaijia/xchain/issues/475)

**Merged pull requests:**

- JCLI fixes and Ed25519 sk unification [\#500](https://github.com/youtubaijia/xchain/pull/500)
- Stop ignoring config.yaml logger settings [\#495](https://github.com/youtubaijia/xchain/pull/495)
- Extend faucet script [\#492](https://github.com/youtubaijia/xchain/pull/492)
- Poll the gRPC client for readiness [\#489](https://github.com/youtubaijia/xchain/pull/489)
- replace invalid TransactionId [\#488](https://github.com/youtubaijia/xchain/pull/488)
- Fix README typo: public\_access-\>public\_address [\#482](https://github.com/youtubaijia/xchain/pull/482)
- add option to disable colours, fix find for deleting tmp files [\#480](https://github.com/youtubaijia/xchain/pull/480)
- Stake key certificate does not exist anymore [\#461](https://github.com/youtubaijia/xchain/pull/461)

## [v0.2.0](https://github.com/youtubaijia/xchain/tree/v0.2.0) (2019-06-13)
[Full Changelog](https://github.com/youtubaijia/xchain/compare/v0.1.0...v0.2.0)

**Fixed bugs:**

- Error when verifying transaction with fee [\#449](https://github.com/youtubaijia/xchain/issues/449)
- Can't read secret key for creating witness with jcli [\#448](https://github.com/youtubaijia/xchain/issues/448)

**Closed issues:**

- jcli: remove 'allow\_account\_creation' from the config generated with 'jcli genesis init \> genesis.yaml'  [\#471](https://github.com/youtubaijia/xchain/issues/471)
- Invalid Node secret file: bft.signing\_key: Invalid prefix: expected ed25519e\_sk but was ed25519\_sk at line 6 column 16 [\#460](https://github.com/youtubaijia/xchain/issues/460)
- remove the shell ansi colours from scripts/stakepool-single-node-test [\#441](https://github.com/youtubaijia/xchain/issues/441)

**Merged pull requests:**

- jcli: 'remove allow\_account\_creation' from 'jcli genesis init' [\#477](https://github.com/youtubaijia/xchain/pull/477)
- Mention add-certificate in stake delegation [\#476](https://github.com/youtubaijia/xchain/pull/476)
- Last minute updates [\#474](https://github.com/youtubaijia/xchain/pull/474)
- Update to API changes in network-grpc [\#468](https://github.com/youtubaijia/xchain/pull/468)
- enable fixing the builds under nix, by making the xchain path configurable [\#464](https://github.com/youtubaijia/xchain/pull/464)
- Bft secretkey cleanup [\#462](https://github.com/youtubaijia/xchain/pull/462)
- Add a full transaction creation and sending example to the docs [\#459](https://github.com/youtubaijia/xchain/pull/459)
- Fix error when the current epoch is nearly finished and no block have been created [\#458](https://github.com/youtubaijia/xchain/pull/458)
- update xblockchain-deps and fix issue with fee check [\#455](https://github.com/youtubaijia/xchain/pull/455)
- Trim strings read with JCLI read\_line [\#454](https://github.com/youtubaijia/xchain/pull/454)
- Adding a utility that'll convert a between different addresses [\#453](https://github.com/youtubaijia/xchain/pull/453)
- Added scripts for bft node and send transaction [\#445](https://github.com/youtubaijia/xchain/pull/445)
- Update network-grpc, ported to tower-hyper [\#444](https://github.com/youtubaijia/xchain/pull/444)
- new test case for genesis utxo stake pool [\#443](https://github.com/youtubaijia/xchain/pull/443)
- improve jcli account-id parsing  [\#442](https://github.com/youtubaijia/xchain/pull/442)
- remove stake key and related certificate, fix network compilation [\#440](https://github.com/youtubaijia/xchain/pull/440)



\* *This Change Log was automatically generated by [github_changelog_generator](https://github.com/skywinder/Github-Changelog-Generator)*