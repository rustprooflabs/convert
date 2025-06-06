# Based on PgDD https://github.com/rustprooflabs/pgdd/tree/main/build
# which was original based on ZomboDB's system.
#
# Copyright 2023-2025 RustProof Labs
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
# 
#!/bin/bash

BUILDDIR=`pwd`
BASE=$(dirname `pwd`)
VERSION=$(cat $BASE/convert.control | grep default_version | cut -f2 -d\')
LOGDIR=${BASE}/target/logs
ARTIFACTDIR=${BASE}/target/artifacts
PGRXVERSION=0.14.1

PG_VERS=("pg13" "pg14" "pg15" "pg16" "pg17")
#PG_VERS=("pg17")

echo $BASE
echo $VERSION
echo $LOGDIR
echo $ARTIFACTDIR
echo "PGRX Version: ${PGRXVERSION}"

mkdir -p ${LOGDIR}
mkdir -p ${ARTIFACTDIR}


for image in `ls docker/ | grep postgis ` ; do

    OS_DIST=$(echo ${image}|cut -f2 -d-)
    OS_VER=$(echo ${image}|cut -f3 -d-)

    echo $OS_DIST
    echo $OS_VER
    echo "Pg Version: ${PG_VER}"

    cd ${BUILDDIR}

    cd docker/${image}
    echo "  Building Docker image: ${image}"
    docker build -t ${image} --build-arg PGRXVERSION=${PGRXVERSION}  . 2>&1 > ${LOGDIR}/${image}-build.log || exit 1

    for PG_VER in ${PG_VERS[@]} ; do

        echo "Build convert extension: ${image}-${PG_VER}"
        docker run \
            -e pgver=${PG_VER} \
            -e image=${image} \
            -v ${BASE}:/build \
            --rm \
            ${image} \
            /bin/bash -c '/build/build/package.sh ${pgver} ${image}' \
                > ${LOGDIR}/${image}-${PG_VER}-package.sh.log 2>&1 || echo 'Building this version might have encountered error.'

        echo "${image}-${PG_VER}:  finished"
    done


    echo "Copying artifacts for ${OS_DIST} ${OS_VER}"
    cd $BASE

    for f in $(find target -name "*.deb") $(find target -name "*.rpm") $(find target -name "*.apk"); do
        echo "copy: ${f}"
        cp $f $ARTIFACTDIR/
    done
done

tar -zcvf $BUILDDIR/convert-binaries.tar.gz -C $ARTIFACTDIR .

