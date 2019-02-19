#! /usr/bin/env sh
set -euxo pipefail

cd tests/external_data
wget -i fetch_list
