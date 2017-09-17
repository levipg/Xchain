#!/bin/bash

CLI='../xblockchain-cli/target/debug/xblockchain-cli'

PRIVKEY=${1:-'key.xprv'}
PUBKEY=${2:-'key.xpub'}

echo "PRIV = $PRIVKEY"
echo "PUB  = $PUBKEY"

$CLI debug generate-xprv $PRIVKEY
$CLI debug xprv-to-xpub $PRIVKEY $PUBKEY


