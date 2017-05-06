#!/bin/bash

SCRIPT_DIR=$(dirname $0)

rm -rf build
mkdir build

cp -R $SCRIPT_DIR build/covet

pushd build
pushd covet

# create changelog
AUTHOR="Armin Häberling <armin.aha@gmail.com>"
VERSION=`cargo metadata --no-deps | python3 -c "import sys, json; print(json.load(sys.stdin)['packages'][0]['version'])"`

cat > debian/changelog << EOF
covet ($VERSION) unstable; urgency=low

  * Packaged ${VERSION}

 -- ${AUTHOR}  $(date -R)

EOF

dpkg-buildpackage -b -rfakeroot -us -uc
popd
lintian covet_*.changes
popd
