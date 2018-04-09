#!/usr/bin/env bash

# Jormaungandr setup tool - Amias Channer
#
# Generates a folder with a specified amount of interoperating xchain nodes
#
echo
echo "Xchain - Setup generator"
echo

clipath=$(which xblockchain-cli)

# params
# folder - folder to prepare for a demo
folder=$1

# nodes - amount of nodes to prepare
nodes=${2:-0}

# genesis - genesis data for nodes to use
genesis=${3:-'demo-genesis.yaml'}

# xblockchain-cli - location of xblockchain-cli 
cli=${4:-$clipath}

# config - template for config
config=${5:-'demo-config.yaml'}


echo "Using these options:"
echo "  Folder: $folder"
echo "  Nodes: $nodes"
echo "  Genesis: $genesis"
echo "  CLI: $cli"
echo "  Config: $config"
echo

if [[ ! $folder ]]; then
  echo  "Error: Please supply a fully qualified folder name as the first parameter"
  exit 1
fi

if [[ -d $folder ]]; then
  echo  "Error: $folder already exists"
  exit 1
fi

if [[ $nodes == 0 ]]; then
  echo "Error: Please supply a node count as the second parameter"
  exit 1
fi

if [[ $genesis ]]; then
  if [[ ! -e $genesis ]]; then
    echo "Error: Cannot read genesis from $genesis"
    exit 1
  fi
else
  echo "Error: Please supply a genesis file as the third parameter"
  exit 1
fi

if [[ $cli ]]; then
  if [[ ! -e $cli ]]; then
    echo "Error: could not read xblockchain-cli at $cli"
    exit 1
  fi
else
  echo "Error: Please supply a xblockchain-cli executable as the fourth parameter"
  exit 1
fi

# we have all the info we need at this point

echo "Building Xchain"
echo
# cd ..
# cargo build
# cd demo  
echo
echo "Build finished"
echo

mkdir -p $folder  

echo "Copying in template"
cp -rv template/* $folder/
echo

echo "Copying in binaries"
cp $cli $folder/bin/xblockchain-cli
cp ../target/debug/xchain $folder/bin/
echo


echo "Making Configs for $nodes nodes"
counter=0
cp $config $folder'/config.yaml'

keys_for_genesis=''

pushd .

while [[ $counter -lt $nodes ]]; do
  node_folder=$folder/nodes/$(($counter+1))
  mkdir -p $node_folder
  cd $node_folder
    
  let counter=$counter+1
		
  stub='node_'$counter
  privkey=$node_folder/$stub'.xprv'
  pubkey=$node_folder/$stub'.xpub'

  echo "Making keys for $stub"
  echo "PRIV = $privkey"
  echo "PUB  = $pubkey"

  ../../bin/xblockchain-cli debug generate-xprv $privkey
  ../../bin/xblockchain-cli debug xprv-to-xpub $privkey $pubkey

  echo "Adding key to global config"
  pubkeycontents=`cat $pubkey`
  echo "    - $pubkeycontents" >> $folder'/config.yaml'
  echo
    
  # put the private key
  privkeycontents=`cat $privkey`
  keys_for_genesis+='"'$privkeycontents'":1,'
done  


popd

# remove trailing comma from list of keys
trimmed_keys=${keys_for_genesis::-1}

echo "Copying in genesis and patching in keys"
cat $genesis  | jq -r '.bootStakeholders |= {'$trimmed_keys'}' > $folder/genesis.json
echo

echo "Setup is complete"
echo 
echo "Its not Ragnarok yet but if you want to unleash your xchain do this:"
echo
echo "cd $folder/bin/" 
echo "./start_nodes.sh"
echo 
  

