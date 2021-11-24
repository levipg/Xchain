# Change Log

## [v0.3.1](https://github.com/youtubaijia/xchain/tree/v0.3.1) (2019-07-19)
[Full Changelog](https://github.com/youtubaijia/xchain/compare/v0.3.0...v0.3.1)

**Implemented enhancements:**

- Add node shutdown over REST [\#643](https://github.com/youtubaijia/xchain/pull/643)
- Add lastBlock info to node state REST [\#642](https://github.com/youtubaijia/xchain/pull/642)
- Add REST endpoint for getting node settings [\#634](https://github.com/youtubaijia/xchain/pull/634)

**Closed issues:**

- Node crash with Crit error when sending multiple transactions from the same Account, with the same Counter, in 2 consecutive slots [\#641](https://github.com/youtubaijia/xchain/issues/641)
- Transaction rejected because "Account with invalid signature" when sending multiple transactions from the same Account in the same slot \(with different Counter values\) [\#640](https://github.com/youtubaijia/xchain/issues/640)

**Merged pull requests:**

- Upgrade Slog to 2.5.1 [\#637](https://github.com/youtubaijia/xchain/pull/637)
- Simplify slot\_start\_time storage to seconds [\#636](https://github.com/youtubaijia/xchain/pull/636)
- Remove unused JCLI deps [\#635](https://github.com/youtubaijia/xchain/pull/635)
- Rename more fields in p2p config [\#632](https://github.com/youtubaijia/xchain/pull/632)

## [v0.3.0](https://github.com/youtubaijia/xchain/tree/v0.3.0) (2019-07-12)
[Full Changelog](https://github.com/youtubaijia/xchain/compare/v0.2.4...v0.3.0)

**Implemented enhancements:**

- Log Level consistency between CLI and config file [\#622](https://github.com/youtubaijia/xchain/issues/622)
- breaking change: move to fragment id to refer to utxo [\#633](https://github.com/youtubaijia/xchain/pull/633)
- Rework handling of inbound blocks and headers from the network [\#626](https://github.com/youtubaijia/xchain/pull/626)

**Fixed bugs:**

- Node crash if sending multiple transactions in the same slot [\#586](https://github.com/youtubaijia/xchain/issues/586)
- broken link, registering stake key guide [\#565](https://github.com/youtubaijia/xchain/issues/565)

**Closed issues:**

- add-output example missing value [\#628](https://github.com/youtubaijia/xchain/issues/628)
- gelf logging broken [\#621](https://github.com/youtubaijia/xchain/issues/621)
- Transactions are rejected when genesis file is re-encoded manually [\#610](https://github.com/youtubaijia/xchain/issues/610)

**Merged pull requests:**

- Rename Message to Fragment [\#631](https://github.com/youtubaijia/xchain/pull/631)
- Clean up unnecessary lifetimes in configuration\_builder test tools [\#630](https://github.com/youtubaijia/xchain/pull/630)
- Small doc updates [\#629](https://github.com/youtubaijia/xchain/pull/629)
- Clean up and extend log configuration [\#627](https://github.com/youtubaijia/xchain/pull/627)
- Process events in the client connection [\#620](https://github.com/youtubaijia/xchain/pull/620)
- Fix node crashing when multiple TXs for same account are in slot [\#619](https://github.com/youtubaijia/xchain/pull/619)
- make sure we use the latest stable available [\#616](https://github.com/youtubaijia/xchain/pull/616)
- move the address prefix to -lib and jcli [\#614](https://github.com/youtubaijia/xchain/pull/614)
- Fix localhost for BSD & OSX [\#613](https://github.com/youtubaijia/xchain/pull/613)
- scripts: bootstrap small fixes [\#611](https://github.com/youtubaijia/xchain/pull/611)
- \[Test\] node stops producing blocks test case [\#609](https://github.com/youtubaijia/xchain/pull/609)
- scripts: create-account-and-delegate POSIX-syntax [\#608](https://github.com/youtubaijia/xchain/pull/608)
- xchain: fix binary version release 0.2.3 -\> 0.2.4 [\#607](https://github.com/youtubaijia/xchain/pull/607)
- Add proper error reporting to JCLI REST commands [\#605](https://github.com/youtubaijia/xchain/pull/605)
- Pulling missing chain blocks from the network [\#601](https://github.com/youtubaijia/xchain/pull/601)
- use binaries by default and support building source too [\#594](https://github.com/youtubaijia/xchain/pull/594)
- Docker: use alpine base image and versioned releases [\#567](https://github.com/youtubaijia/xchain/pull/567)

## [v0.2.4](https://github.com/youtubaijia/xchain/tree/v0.2.4) (2019-07-04)
[Full Changelog](https://github.com/youtubaijia/xchain/compare/v0.2.3...v0.2.4)

**Implemented enhancements:**

- Improve bootstrap script to prevent a non-stake case to appear [\#604](https://github.com/youtubaijia/xchain/pull/604)
- Rest stake distribution [\#603](https://github.com/youtubaijia/xchain/pull/603)
- More graceful error handling in blockchain task [\#588](https://github.com/youtubaijia/xchain/pull/588)
- More logging improvements; add output to stdout [\#587](https://github.com/youtubaijia/xchain/pull/587)

**Fixed bugs:**

- block0 initial funds should accept multiple entries [\#579](https://github.com/youtubaijia/xchain/issues/579)
- jcli add-certificate does not take fees into account [\#499](https://github.com/youtubaijia/xchain/issues/499)

**Closed issues:**

- bootstrap script error [\#602](https://github.com/youtubaijia/xchain/issues/602)
- v0.2.3 Cannot Compile \(Experimental Alpine Docker\) [\#590](https://github.com/youtubaijia/xchain/issues/590)
- cargo install compile fail [\#581](https://github.com/youtubaijia/xchain/issues/581)
- add-output results in invalid internal encoding error [\#577](https://github.com/youtubaijia/xchain/issues/577)
- Documentation : empty faucet warning \(?\) [\#564](https://github.com/youtubaijia/xchain/issues/564)
- bootstrap error in genesis\_praos, genesis file corrupted [\#562](https://github.com/youtubaijia/xchain/issues/562)
- Documentation: Improve the documentation related to Staking&Delegation  [\#530](https://github.com/youtubaijia/xchain/issues/530)
- documentation: Add a consolidated/consistent/easier way for starting the node [\#515](https://github.com/youtubaijia/xchain/issues/515)
- documentation: improve the documentation for 'jcli rest v0 account get' [\#484](https://github.com/youtubaijia/xchain/issues/484)

**Merged pull requests:**

- Finalize Divide and Reuse [\#600](https://github.com/youtubaijia/xchain/pull/600)
- More changes in the xchain-lib API [\#599](https://github.com/youtubaijia/xchain/pull/599)
- take into account the certificate when computing the fees [\#598](https://github.com/youtubaijia/xchain/pull/598)
- Use certificate from xchain lib [\#597](https://github.com/youtubaijia/xchain/pull/597)
- Improve testing of the Block0Configuration [\#596](https://github.com/youtubaijia/xchain/pull/596)
- updated delegation script  [\#595](https://github.com/youtubaijia/xchain/pull/595)
- REST refactoring and simplification [\#593](https://github.com/youtubaijia/xchain/pull/593)
- Test Improvement. Implement dumping logs on console on xchain error [\#592](https://github.com/youtubaijia/xchain/pull/592)
- Update chain-deps [\#585](https://github.com/youtubaijia/xchain/pull/585)
- experiment with changelog generation [\#584](https://github.com/youtubaijia/xchain/pull/584)
- Fix multi output funds support in genesis yaml file [\#583](https://github.com/youtubaijia/xchain/pull/583)
- Add notes on protobuf and C compilers to the install steps [\#582](https://github.com/youtubaijia/xchain/pull/582)
- Improve GELF logging backend configuration [\#574](https://github.com/youtubaijia/xchain/pull/574)
- bootstrap script ported to Windows Powershell [\#572](https://github.com/youtubaijia/xchain/pull/572)
- Update blockchain.md [\#568](https://github.com/youtubaijia/xchain/pull/568)
- Pull missing blocks [\#554](https://github.com/youtubaijia/xchain/pull/554)
- Add example for account input and links to scripts [\#487](https://github.com/youtubaijia/xchain/pull/487)

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
