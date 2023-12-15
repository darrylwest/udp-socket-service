#!/usr/bin/env bash
# dpw@alamo
# 2023-12-15 18:49:04
#

set -eu

echo "status" | nc -w 1 -u 127.0.0.1 22200
echo ""
