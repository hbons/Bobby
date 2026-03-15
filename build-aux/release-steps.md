# Release Steps

## 0. Slow down and enjoy 🌱

* Reflect on the improvements made
* When ready, follow the next steps in order


## 1. Bump version number

* Edit `./meson.build`, `./Cargo.lock`, `./Cargo.toml`

```shell
git add ./meson.build ./Cargo.lock ./Cargo.toml
git commit -m "meta: Bump version to <VERSION>"
git push
```


## 2. Update release notes

* Edit `./data/<APP_ID>.metainfo.xml`

```shell
git add ./data/<APP_ID>.metainfo.xml
git commit -m "meta: Update release notes for <VERSION>"
git push
```


## 3. Build and upload release

```shell
meson setup --wipe ./build
meson dist -C ./build --allow-dirty
xdg-open ./build/meson-dist
```

* Create a [new release on GitHub](https://github.com/hbons/Bobby/releases/new)
  * Upload the release tarball
  * Select the newly pushed tag name and use `<APP_NAME> <VERSION>` as the title


## 4. Update Flatpak manifest

```shell
shasum -a 256 ./build/meson-dist/*.tar.xz
```

* Edit `url` and `sha256` in `<APP_ID>.yml`

```shell
git add ./<APP_ID>.yml
git commit -m "flatpak: Bump manifest to <VERSION>"
git push
```

## 5. Tag release

```shell
git tag <VERSION> HEAD
git push --tags
```

* Wait for the release to build in CI and test the `.flatpak`


## 6. Publish to Flathub

* Open a Pull Request
  * Go to `https://github.com/flathub/<APP_ID>` and update the manifest.
  * Commit with message `Bump to <VERSION>`


## 7. Celebrate! 🥳
