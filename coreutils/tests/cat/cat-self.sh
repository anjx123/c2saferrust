#!/bin/bash
# Check that cat operates correctly when the input is the same as the output.

# Copyright 2014-2024 Free Software Foundation, Inc.

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

echo x >out || framework_failure_
echo x >out1 || framework_failure_
returns_ 1 cat out >>out || fail=1
compare out out1 || fail=1

# This example is taken from the POSIX spec for 'cat'.
echo x >doc || framework_failure_
echo y >doc.end || framework_failure_
cat doc doc.end >doc || fail=1
compare doc doc.end || fail=1

# This terminates even though it copies a file to itself.
# Coreutils 9.5 and earlier rejected this.
echo x >fx || framework_failure_
echo y >fy || framework_failure_
cat fx fy >fxy || fail=1
for i in 1 2; do
  cat fx >fxy$i || fail=1
done
for i in 3 4 5 6; do
  cat fx >fx$i || fail=1
done
cat - fy <fxy1 1<>fxy1 || fail=1
compare fxy fxy1 || fail=1
cat fxy2 fy 1<>fxy2 || fail=1
compare fxy fxy2 || fail=1
returns_ 1 cat fx fx3 1<>fx3 || fail=1
returns_ 1 cat - fx4 <fx 1<>fx4 || fail=1
returns_ 1 cat fx5 >>fx5 || fail=1
returns_ 1 cat <fx6 >>fx6 || fail=1

Exit $fail
