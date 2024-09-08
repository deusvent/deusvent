#!/bin/bash
set -euo pipefail

# Github ubuntu runner has only about 15GB of free space, while Docker image for unreal-engine:dev-slim takes 35GB
# We don't need basically anything besides Docker, so remove all the extra files
# Taken and adjusted from https://gist.github.com/antiphishfish/1e3fbc3f64ef6f1ab2f47457d2da5d9d
sudo rm -rf \
  /usr/share/dotnet \
  /usr/share/swift \
  /usr/local/lib/android \
  /opt/ghc \
  /usr/local/.ghcup \
  /usr/local/share/boost \
  /opt/hostedtoolcache/ \
  /usr/local/graalvm/ \
  /usr/local/share/powershell \
  /usr/local/share/chromium \
  /usr/local/lib/node_modules

# Those commands takes around 1.5 minutes to run, uncomment if you need even more space
# sudo docker image prune --all --force
# sudo apt -y -qq -o=Dpkg::Use-Pty=0 remove -y '^dotnet-.*' '^llvm-.*' '^php.*' '^mongodb-.*' '^mysql-.*' \
#   azure-cli google-* google-chrome-stable firefox powershell mono-devel libgl1-mesa-dri
# sudo apt -y -qq -o=Dpkg::Use-Pty=0 autoremove --purge -y
# sudo apt -y -qq -o=Dpkg::Use-Pty=0 autoclean
# sudo apt -y -qq -o=Dpkg::Use-Pty=0 clean

