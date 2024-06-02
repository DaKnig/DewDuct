#!/usr/bin/sh

set -Eeuo pipefail

if [ $# -ne 1 ]
then
	echo "usage: $0 ssh-device-name"
	exit 1
fi

# clean dir
echo removing apk files and APKBUILD...
rm -rf ~/.local/var/pmbootstrap/cache_git/pmaports/testing/
rm -f ~/.local/var/pmbootstrap/packages/edge/aarch64/dewduct*.apk
# stay updated!
echo agent setup...
eval $(ssh-agent)
ssh-add

echo pmbootstrap pulling...
pmbootstrap pull

mkdir -p ~/.local/var/pmbootstrap/cache_git/pmaports/testing/dewduct
cp aport/APKBUILD_dev ~/.local/var/pmbootstrap/cache_git/pmaports/testing/dewduct/APKBUILD

# prepare the apk
echo checksum...
pmbootstrap checksum dewduct
cp ~/.local/var/pmbootstrap/cache_git/pmaports/testing/dewduct/APKBUILD aport/APKBUILD_dev
echo build...
# hack: to avoid rsync-ing target folder (tens of gigs!) into the chrootsx
mv target .git/
pmbootstrap build --arch aarch64 dewduct --src="$PWD"
mv .git/target target
# push it
echo sideload...
scp ~/.local/var/pmbootstrap/packages/edge/aarch64/dewduct*.apk \
    $1:/tmp
ssh $1 -t sudo apk add /tmp/dewduct*.apk --allow-untrusted
