#!/bin/sh
# This script sets up the environment to run the tests. These values were copied
# from the TESTS_ENVIRONMENT variable in the original coreutils Makefile.

. ./lang-default

export TMPDIR=/tmp \
    VERSION=9.5.42-bbc97 \
    RUN_EXPENSIVE_TESTS=yes \
    RUN_VERY_EXPENSIVE_TESTS=yes \
    LOCALE_FR=none \
    LOCALE_FR_UTF8=none \
    abs_top_builddir=/tmp/coreutils \
    abs_top_srcdir=/tmp/coreutils \
    fail=0 \
    host_os=linux-gnu \
    host_triplet=aarch64-unknown-linux-gnu \
    srcdir=. \
    top_srcdir=. \
    CONFIG_HEADER=src/cat/c/include/config.h \
    CC='gcc' \
    AWK='mawk' \
    EGREP='/usr/bin/grep -E' \
    EXEEXT='' \
    MAKE='/usr/bin/make' \
    PACKAGE_VERSION=9.5.42-bbc97 \
    PERL='perl' \
    SHELL='/bin/bash'

PATH='/usr/xpg4/bin:'"$PATH"
PATH='/usr/sbin:'"$PATH"

. ./envvar-check

# Build the getlimits utility used by some tests
#make --directory=getlimits/
export PATH="$(pwd)/getlimits:$PATH"

9>&2
