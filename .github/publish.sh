#!/bin/sh -ex

git branch -D gh-pages || true
git checkout -b gh-pages
rm -rf www/dist/
wasm-pack build rgis
(cd www && npm run build)
cp -R www/dist/* .
git add -A
git config user.name "Automated"
git config user.email "actions@users.noreply.github.com"
git commit -m 'publish'
git push -f origin gh-pages

