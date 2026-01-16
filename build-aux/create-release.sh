#!/bin/sh

# SPDX-FileCopyrightText: 2026 Hylke Bons <hello@planetpeanut.studio>
# SPDX-License-Identifier: GPL-3.0-or-later

set -e

meson setup ./build
meson dist -C ./build
shasum -a 256 build/meson-dist/*.tar.xz
