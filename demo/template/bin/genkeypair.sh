#!/bin/bash

# genkeypair - uses xblockchain-cli to create public and private keys
CLI='./xblockchain-cli'

# params - priv and pub key names
PRIVKEY=${1:-'key.xprv'}
PUBKEY=${2:-'key.xpub'}

echo "PRIV = $PRIVKEY"
echo "PUB  = $PUBKEY"

$CLI debug generate-xprv $PRIVKEY
$CLI debug xprv-to-xpub $PRIVKEY $PUBKEY
