# DockSprout 🌱🐳

**A simple CLI tool to bring up Docker containers from multiple `docker-compose.yml` files in subdirectories.**

---
## **📌 Features**

- Scans a **central directory** for `docker-compose.yml` files.
- Recursively searches **subdirectories** for Docker Compose projects.
- Automatically **brings up (or down)** all discovered containers using `docker compose`.
- Use a `.sprout-ignore`file to ignore certain directories of containers you don't want brought up.
- A lightweight and convenient **automation tool** for managing multiple Docker services.

---
## **🛠 Installation** 

1. Install Rust toolchain <https://www.rust-lang.org/tools/install>
2. Run `cargo install dock_sprout`

### **🔹 Homebrew**
```bash 
brew install Thompson-Jason/tap/dock_sprout
```

### **🔹 Build from Source** 
```bash 
git clone https://github.com/Thompson-Jason/DockSprout.git
cd DockSprout
cargo build --release
```

---
## **🚀 Usage**

`sprout <root-directory> <docker-compose-direction>`

## 🔹 **Example**:
`sprout ~/my-docker-projects up`

```
─── my-docker-projects
    ├── LubeLogger/
    │   └── docker-compose.yml
    ├── ntfy/
    │   └── docker-compose.yaml
    ├── Portainer/
    │    ├── docker-compose.yaml
    │    └──.conf
    └── .sprout-ignore
```

This will bring up all three containers separately allowing for the esability of one command to bring all of your containers up and none of the downsides of a mono compose file.
