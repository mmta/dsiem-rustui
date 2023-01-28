#!/bin/sh

[ "$1" != "prod" ] && [ "$1" != "dev" ] && echo need prod or dev as 1st argument && exit 1
mode=$1

for cmd in git trunk docker; do
  command -v $cmd >/dev/null 2>&1 || {
    echo ${cmd} command doesnt exist
    exit 1
  }
done

root=$(git rev-parse --show-toplevel)
cd $root

index="index.dev.html"
[ "$mode" = "prod" ] && rel_flag="--release" && public="--public-url /ui" && index="index.html"

./scripts/css.sh $mode &&
  echo "** building wasm $mode version" &&
  trunk build $public $rel_flag $index

[ "$mode" = "prod" ] &&
  echo "** building dsiem-rustui docker image" &&
  rm -rf ./docker/dist &&
  cp -r dist ./docker/ &&
  cd docker &&
  docker build -t dsiem-rustui .
