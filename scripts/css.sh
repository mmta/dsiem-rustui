#!/bin/sh

for cmd in npx npm; do
  command -v $cmd >/dev/null 2>&1 || {
    echo ${cmd} command doesnt exist
    exit 1
  }
done

root=$(git rev-parse --show-toplevel)
cd $root

[ "$1" = "" ] && echo use "prod" or "dev" as 1st argument && exit 1
[ "$1" = "prod" ] && NODE_ENV=production npx tailwindcss -c ./tailwind.config.js -o ./tailwind.css --minify
[ "$1" = "dev" ] && npx tailwindcss -o ./tailwind.css

npm init -y && npm install text-spinners@1.0.5
