#! /bin/sh

mkdir -p ./pkg/src
cp -r ../src.ts/* pkg/src/.
rm -f ./pkg/raw.ts
echo 'export * from "./rapier_wasm2d"' > pkg/src/raw.ts
# See https://serverfault.com/a/137848
find pkg/ -type f -print0 | LC_ALL=C xargs -0 sed -i.bak '\:#if DIM3:,\:#endif:d'
find pkg/ -type f -name '*.bak' | xargs rm
