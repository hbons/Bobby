# Release Steps

## 0. Slow down and enjoy 🌱

* Reflect on the improvements made
* When ready, follow the next steps in order


## 1. Bump version number

* Edit `./meson.build`, `./Cargo.toml`

```shell
cargo update
git add ./meson.build ./Cargo.lock ./Cargo.toml
git status
git commit -m "meta: Bump version to v<VERSION>"
git push
```


## 2. Update release notes

* Edit `./data/<APP_ID>.metainfo.xml`

```shell
git add ./data/*.metainfo.xml
git status
git commit -m "meta: Update release notes for v<VERSION>"
git push
```


## 3. Build tarball
```shell
meson setup --wipe ./build
meson dist -C ./build --allow-dirty
xdg-open ./build/meson-dist
```


## 4. Update Flatpak manifest

```shell
shasum -a 256 ./build/meson-dist/*.tar.xz
```

* Edit `url` and `sha256` in `./<APP_ID>.yml`

```shell
git add ./*.yml
git status
git commit -m "flatpak: Bump manifest to v<VERSION>"
git push
```

  
## 5. Tag release

```shell
git tag v<VERSION> HEAD
git push --tags
```

* Create a **new release** on [GitHub](https://github.com)
  * Upload the release tarball
  * Select the newly pushed tag name and use `<APP_NAME> v<VERSION>` as the title


## 6. Publish to Flathub

* Open a PR
  * Go to `https://github.com/flathub/<APP_ID>` and update the manifest
    * Bump the runtime if needed
  * Commit with message `Bump to v<VERSION>`
* Test the automated build
* Merge the PR


## 7. Celebrate! 🥳
