# Ronin – Lightweight Dev Container Orchestrator

Ronin ist ein schlankes CLI-Tool zur Verwaltung von Entwicklungscontainern auf Basis von `systemd-nspawn`. Es ist gedacht für Entwickler, die systemnahe Isolierung und einfache Reproduzierbarkeit ohne OCI-Daemon benötigen – ideal als Alternative zu Toolbox oder Docker für Fedora-Workstations.

---

## 🛠 Voraussetzungen

* **Betriebssystem**: Fedora 39 oder neuer
* **Pakete** (werden automatisch installiert):

  * `systemd-container`
  * `dnf` (hostseitig)

---

## 📦 Installation

```bash
git clone https://github.com/youruser/ronin.git
cd ronin
chmod +x ronin ronin-init.sh
sudo cp ronin /usr/local/bin/
```

---

## 🚀 Schnellstart

```bash
ronin init web1      # Erstellt neuen Container "web1" (Fedora @core)
ronin start web1     # Startet den Container
ronin shell web1     # Wechselt in eine Root-Shell im Container
ronin exec web1 -- dnf install gcc make
ronin stop web1      # Beendet den Container
ronin rm web1        # Entfernt Container und Konfiguration
```

---

## 🔍 Befehlsübersicht

```bash
ronin init <name>       # Erstellt RootFS und .nspawn-Datei
ronin list              # Zeigt alle Maschinen
ronin start <name>      # Startet Maschine via machinectl
ronin stop <name>       # Stoppt Container (systemd poweroff)
ronin shell <name>      # Rootshell im Container (systemd-nspawn)
ronin exec <name> -- <cmd>  # Führt Befehl im Container aus
ronin rm <name>         # Beendet und entfernt Containerdateien
```

---

## 🌐 Netzwerk

Ronin legt automatisch eine Bridge `br0` mit DHCP ein – dein Container erhält dadurch eine IP und kann ins Internet.

---

## ❓ Fragen & Roadmap

* Unterstützung für Benutzercontainer (rootless)?
* Templates / Profile für Sprachumgebungen (Rust, Python ...)?
* Mount-Presets (`~/dev → /mnt/dev`)?

---

Mit Ronin hast du eine systemnahe, performante und transparente Umgebung, ganz ohne Images und Daemons. Ideal für Nerds mit Root.
