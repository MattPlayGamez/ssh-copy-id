# 🔐 SSH-copy-id Installer (Windows)

Een kleine Rust-applicatie om je publieke SSH-sleutel automatisch naar een remote server te kopiëren voor **wachtwoordloos inloggen (autologin)** via SSH zoals  het geval in Linux.

---

## 📦 Beschrijving

Deze tool automatiseert het proces van het kopiëren van jouw **publieke SSH-sleutel** (`id_rsa.pub`, `id_ed25519.pub`, enz.) naar een **remote Linux-systeem**, door gebruik te maken van PowerShell en een eenvoudig SSH-commando.

De tool doet in feite het volgende:
- Leest je `~/.ssh/id*.pub` sleutel
- Verstuurt deze via SSH naar de opgegeven gebruiker/host
- Plakt de sleutel aan op `~/.ssh/authorized_keys` van de remote server

---

## ⬇️ Installatie
De tool kan worden gedownload met de onderliggende one-liner
````powershell
powershell.exe -Command "if(-not([Security.Principal.WindowsPrincipal]::new([Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltinRole]::Administrator))){Start-Process powershell -Verb RunAs -ArgumentList '-NoProfile -Command `"Invoke-WebRequest https://github.com/MattPlayGamez/ssh-copy-id/releases/download/main/ssh-copy-id.exe -OutFile C:\Windows\ssh-copy-id.exe`"';exit}else{Invoke-WebRequest https://github.com/MattPlayGamez/ssh-copy-id/releases/download/main/ssh-copy-id.exe -OutFile C:\Windows\ssh-copy-id.exe}" 
````
---

## 🚀 Gebruik

### ⚠️ Vereisten

- Windows
- PowerShell beschikbaar in je `PATH`
- Een gegenereerde SSH-sleutel in `~/.ssh` (bijv. `id_ed25519.pub`)
- SSH toegankelijk op de remote machine
- `ssh` geconfigureerd op Windows (bijv. via `OpenSSH`)

---

### 🔧 Installatie

1. **Compileer het project** met Cargo:
```bash
cargo build --release
```

2. **Uitvoeren**:
```bash
target\release\ssh-autokey.exe <user@host>
```

> Bijvoorbeeld:
```bash
target\release\ssh-autokey.exe matt@192.168.0.2
```

---

## 🧪 Voorbeeld

```powershell
> ssh-autokey.exe matt@192.168.0.2
✅ Public key copied successfully to matt@192.168.0.2
```

Na dit commando zou je zonder wachtwoord kunnen inloggen:

```bash
ssh matt@192.168.0.2
```

---

## 🆘 Help

Toon helptekst:

```bash
ssh-autokey.exe --help
```

Of bij te weinig argumenten:

```bash
ssh-autokey.exe
```

Geeft:

```
Too few arguments.
Usage: ssh-autokey.exe <SSH User@Host; e.g. matt@192.168.0.2>
```

---

## ⚠️ Let op

- Dit **overschrijft** `authorized_keys` niet, maar voegt toe.
- Zorg dat `ssh` en `powershell` beschikbaar zijn in je `PATH`.

---

## 📃 Licentie

MIT License — gebruik, wijzig en verspreid vrij.