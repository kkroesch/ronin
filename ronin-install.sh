#!/bin/bash
set -euo pipefail

# --- Farben & Symbole ---
warn() {
  echo -e "\e[38;5;208m⚠️  $1\e[0m"
}

fatal() {
  echo -e "\e[31m❌ $1\e[0m" >&2
  exit 1
}

success() {
  echo -e "\e[32m✅ $1\e[0m"
}

info() {
  echo -e "\e[36mℹ️  $1\e[0m"
}

progress() {
  echo -ne "\e[33m⏳ $1...\e[0m"
}

done() {
  echo -e "\e[32m done\e[0m"
}

# --- Konfiguration ---
MACHINE_DIR="/var/lib/machines/"
NSPAWN_FILE="/etc/systemd/nspawn/"
NETDEV_FILE="/etc/systemd/network/br0.netdev"
BRIDGE_FILE="/etc/systemd/network/br0.network"
BRIDGE_VETH_FILE="/etc/systemd/network/bridge-ve.network"

# --- Rootcheck ---
if [[ $EUID -ne 0 ]]; then
  fatal "Bitte als root oder mit sudo ausführen."
fi

progress "Erstelle systemd-networkd-Bridge-Konfiguration"
mkdir -p /etc/systemd/network

echo "[NetDev]
Name=br0
Kind=bridge" > "$NETDEV_FILE"

echo "[Match]
Name=br0

[Network]
DHCP=yes" > "$BRIDGE_FILE"

echo "[Match]
Name=ve-*

[Network]
Bridge=br0" > "$BRIDGE_VETH_FILE"
done

progress "Aktiviere systemd-networkd"
systemctl enable --now systemd-networkd
done

success "Bridge-Konfiguration abgeschlossen."

