#!/bin/sh

# SPDX-FileCopyrightText: 2026 Hylke Bons <hello@planetpeanut.studio>
# SPDX-License-Identifier: GPL-3.0-or-later


set -euo pipefail


git checkout main

echo ""
echo ""
echo "Step 1/3: Bump the version number in the following files:"
echo " - ./meson.build"
echo " - ./Cargo.lock"
echo " - ./Cargo.toml"
echo ""
read -p "When done, press [Enter]: "


NAME=$(cargo metadata --no-deps --format-version=1 | jq -r '.packages[] | select(.source == null) | .name')
VERSION=$(cargo metadata --no-deps --format-version=1 | jq -r '.packages[] | select(.source == null) | .version')
FLATPAK_ID=$(cargo metadata --no-deps --format-version=1 | jq -r '.packages[] | select(.source == null) | .metadata.flatpak.id')

git add ./meson.build ./Cargo.lock ./Cargo.toml
git commit -m "meta: Bump version to $VERSION"

echo ""
echo ""
echo "Step 2/3: Update the release notes:"
echo " - ./data/$FLATPAK_ID.yml"
echo ""
read -p "When done, press [Enter]: "


git add "./data/$FLATPAK_ID.yml"
git commit -m "meta: Update release notes for $VERSION"
meson setup --wipe ./build
meson dist -C ./build --allow-dirty
git tag $VERSION HEAD
git push origin main
git push --tags origin main
xdg-open ./build/meson-dist/

echo ""
echo ""
echo "Step 3/5: Upload release tarball to GitHub"
echo " - Tag: $VERSION"
echo " - Title: $NAME $VERSION"
echo " - ./build/meson-dist/$NAME-$VERSION.tar.xz"
echo " - https://github.com/hbons/$NAME/releases/new"
echo ""
read -p "When done, press [Enter]: "


SHA256=$(shasum -a 256 build/meson-dist/*.tar.xz | awk '{print $1}')

echo ""
echo ""
echo "Step 4/5: Bump version in manifest "
echo " - SHA256: $SHA256"
echo " - ./$FLATPAK_ID.yml"
echo ""
read -p "When done, press [Enter]: "


git add ./$FLATPAK_ID.yml
git commit -m "flatpak: Bump manifest to $VERSION"
git push origin main

echo ""
echo ""
echo "Step 5/5: Update the manifest on Flathub:"
echo " - https://github.com/flathub/$FLATPAK_ID/edit/master/$FLATPAK_ID.yml"
echo ""
read -p "When done, press [Enter]: "


echo ""
echo ""
echo "All done!"
echo ""
