#!/bin/bash

ROOTFS="/tmp/fedora-rootfs"
RELEASE="42"

echo "Creating Fedora $RELEASE rootfs at $ROOTFS..."

sudo mkdir -p $ROOTFS

echo "Installing base packages..."
sudo dnf install --installroot=$ROOTFS --releasever=$RELEASE \
    --setopt=install_weak_deps=false --setopt=installonly_limit=3 \
    --use-host-config -y \
    fedora-release \
    bash \
    coreutils \
    findutils \
    grep \
    sed \
    tar \
    gzip \
    which \
    procps-ng \
    util-linux \
    shadow-utils \
    hostname \
    iproute \
    net-tools \
    openssh-clients \
    curl \
    wget \
    ca-certificates \
    glibc-langpack-en \
    tzdata \
    systemd \
    systemd-udev

echo "Configuring /etc/resolv.conf..."
echo "nameserver 8.8.8.8" | sudo tee $ROOTFS/etc/resolv.conf

echo "Configuring /etc/hosts..."
echo "127.0.0.1 localhost" | sudo tee $ROOTFS/etc/hosts

echo "Configuring /etc/passwd and /etc/group..."
echo "root:x:0:0:root:/root:/bin/bash" | sudo tee $ROOTFS/etc/passwd
echo "root:x:0:" | sudo tee $ROOTFS/etc/group

echo "Configuring /etc/nsswitch.conf..."
cat <<EOF | sudo tee $ROOTFS/etc/nsswitch.conf
passwd: files
group: files
shadow: files
hosts: files dns
EOF

echo "Configuring /etc/profile..."
cat <<EOF | sudo tee $ROOTFS/etc/profile
export PATH=/usr/local/bin:/usr/bin:/bin:/usr/local/sbin:/usr/sbin:/sbin
export LANG=en_US.UTF-8
EOF

echo "✅ Done! Rootfs is at $ROOTFS"
