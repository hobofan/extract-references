#! /usr/bin/env sh
set -euxo pipefail

cd test/external_data
wget -i fetch_list
