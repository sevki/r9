#!/bin/sh
# Regenerate the raw c2rust output for one file of Harvey's sys/src/9/port.
#
#   transpile.sh /path/to/harvey port/qlock.c OUTDIR
#
# Needs c2rust 0.22.1 and clang 18 (llvm-18-dev, libclang-18-dev). Harvey must
# be checked out on the GPL-C11 branch, whose amd64 kernel builds with stock
# gcc/clang. Output lands in OUTDIR/src/<name>.rs.
set -eu
harvey=$1 file=$2 out=$3
here=$(cd "$(dirname "$0")" && pwd)
d=$harvey/sys/src/9
tmp=$(mktemp -d)
cat >"$tmp/compile_commands.json" <<JSON
[{"directory":"$d","file":"$d/$file","arguments":["clang","-ffreestanding","-fno-builtin","-D__HARVEY__","-I$here","-I$harvey/amd64/include","-I$harvey/sys/include","-I$d/amd64","-I$d/port","-c","$d/$file"]}]
JSON
mkdir -p "$out"
c2rust transpile -o "$out" "$tmp/compile_commands.json"
rm -r "$tmp"
