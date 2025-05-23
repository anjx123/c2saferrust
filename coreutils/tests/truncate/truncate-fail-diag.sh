#!/bin/sh
# make sure truncate gives reasonable diagnostics
# Note open() checks for trailing '/' before checking for existence
# open (".", O_CREAT & (O_WRONLY | O_RDWR), ...) -> EISDIR
# open ("missing/", O_CREAT & (O_WRONLY | O_RDWR), ...) -> EISDIR
# open ("missing/file", O_CREAT & (O_WRONLY | O_RDWR), ...) -> ENOENT

# Copyright (C) 2008-2024 Free Software Foundation, Inc.

# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.

# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.

# You should have received a copy of the GNU General Public License
# along with this program.  If not, see <https://www.gnu.org/licenses/>.

SCRIPTPATH="$( cd -- "$(dirname "$0")" >/dev/null 2>&1 ; pwd -P )"
. "$SCRIPTPATH/../../tests/init.sh"; path_prepend_ $1
skip_if_root_


d1=no

dir=$d1/such-dir
truncate -s0 $dir > out 2>&1 && fail=1
cat <<EOF > exp
truncate: cannot open '$dir' for writing: No such file or directory
EOF
compare exp out || fail=1

dir=$d1/
truncate -s0 $dir > out 2>&1 && fail=1
#The following can be returned at least
#truncate: cannot open '$dir' for writing: Not a directory
#truncate: cannot open '$dir' for writing: Is a directory

Exit $fail
