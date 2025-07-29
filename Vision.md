# Vision: Ronin

Ronin ist ein leichtgewichtiger Container-Orchestrator, der mit minimaler Komplexität maximale Kontrolle über lokale und verteilte Applikationen bietet. Statt auf Kubernetes, Docker oder proprietäre Plattformen zu setzen, nutzt Ronin die nativen Möglichkeiten von Linux – insbesondere `systemd-nspawn` – und kombiniert sie mit einer klaren, deklarativen Konfiguration im TOML-Format.

## Warum Ronin?

* **Komplexität vermeiden**: Moderne Orchestrierungs-Tools wie Kubernetes bringen massive Abhängigkeiten, Lernaufwand und Overhead mit sich – selbst für einfache Anwendungsfälle.
* **Einfachheit leben**: Ronin folgt der UNIX-Philosophie: kleine, fokussierte Werkzeuge, die gut zusammenarbeiten.
* **Selbstbestimmte Infrastruktur**: Keine Cloud-Abhängigkeiten, keine Registries, keine magischen Controller. Ronin funktioniert mit SSH und rsync.
* **Transparenz und Wartbarkeit**: Was Ronin ausführt, ist sichtbar, greifbar und lokal nachvollziehbar.

## Kernprinzipien

* **Deklarativ**: Anwendungen werden als TOML-Dateien beschrieben.
* **Reproduzierbar**: Ein `ronin deploy` erzeugt stets denselben Zustand.
* **Portabel**: Keine spezielle Plattform nötig. Läuft auf jeder Linux-Maschine mit `systemd`.
* **Lesbar und wartbar**: Keine YAML-Hieroglyphen, keine Operator-Controller-Helm-Kaskaden.

## Zielgruppe

* Entwickler und DevOps, die schlanke und wartbare Deployments suchen
* Selbsthoster und Homelab-Enthusiasten
* Edge-Computing-Projekte und Embedded-Anwendungen
* Minimalisten, denen Kubernetes zu viel ist

## Langfristige Vision

Ronin soll eine echte Alternative zu Kubernetes in einfachen Szenarien werden:

* Lokale Plattform zum Starten, Stoppen und Verwalten von Containern
* Netzwerk- und Ingress-Management ohne Magie
* Declarative Deployments über Git oder Filesystem
* Optional: Web-UI, REST-API, Plugin-System
* Cluster-Betrieb über mehrere Hosts mit:

  * Overlay-Netzwerken via Open vSwitch
  * Gemeinsamen Verzeichnissen über NFS, GlusterFS oder CephFS
  * Steuerung über SSH, rsync und TOML – kein zentraler Controller notwendig
  * Automatische Einrichtung von Nodes via Ansible

Ronin steht für eine Infrastruktur, die dir gehört. Kein Lock-in. Kein Zirkus. Nur du, dein Code und eine Prise `systemd`.

## Roadmap

| Feature            | Status            | Bemerkung                                                              |
| ------------------ | ----------------- | ---------------------------------------------------------------------- |
| Failover           | ✅ geplant         | via watchdog oder Agent, lokal oder verteilt                           |
| Upscaling          | ✅ geplant         | deklarativ per `replicas = N`, mehrere Hosts                           |
| Loadbalancer       | ✅ direkt möglich  | via Caddy, HAProxy oder statischem Proxy                               |
| Autoscaling        | 🔶 später denkbar | auf Basis von CPU- oder Netzlast                                       |
| Web-UI/API         | 🔶 optional       | für einfache Bedienung und Statusanzeige                               |
| Plugins            | 🔶 offen          | für Erweiterungen (Secrets, Templates, etc.)                           |
| Multi-Node-Support | ✅ geplant         | mit Open vSwitch + Cluster-FS (NFS, Gluster, Ceph), ohne Control Plane |
| Ansible-Setup      | ✅ geplant         | Automatische Einrichtung der Nodes                                     |

## CLI-Referenz

```bash
ronin init                 # Erstellt eine neue leere App-Konfiguration im aktuellen Verzeichnis
ronin build                # Erstellt systemd-nspawn-ready Container aus der App-Beschreibung
ronin deploy [app]         # Deployt eine App lokal oder remote (per SSH + rsync)
ronin apply [dir]          # Wendet eine App-Konfiguration direkt an (ohne rsync)
ronin status               # Zeigt laufende Instanzen und ihren Zustand
ronin logs <app>           # Zeigt Logs einer App
ronin stop <app>           # Stoppt eine laufende App
ronin remove <app>         # Entfernt eine App inklusive Container-Instanz
ronin generate-manpage     # Erstellt eine man-Page aus der eingebauten CLI-Doku
ronin version              # Zeigt die installierte Version
```

Für ausführliche Hilfe zu einem Unterbefehl:

```bash
ronin <subcommand> --help
```
