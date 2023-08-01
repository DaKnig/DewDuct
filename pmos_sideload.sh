#!/usr/bin/sh
# clean dir
rm -rf ~/.local/var/pmbootstrap/cache_git/pmaports/testing/
# stay updated!
echo agent setup...
eval $(ssh-agent)
ssh-add

echo pulling...
pmbootstrap pull
git push

mkdir -p ~/.local/var/pmbootstrap/cache_git/pmaports/testing/dewduct
cp pmaport/APKBUILD ~/.local/var/pmbootstrap/cache_git/pmaports/testing/dewduct

# prepare the apk
echo checksum...
pmbootstrap checksum dewduct
echo build...
pmbootstrap build --arch aarch64 dewduct
# push it
echo sideload...
scp ~/.local/var/pmbootstrap/packages/edge/aarch64/dewduct*.apk \
    $1:/tmp
ssh $1 -t doas apk add /tmp/dewduct*.apk --allow-untrusted
