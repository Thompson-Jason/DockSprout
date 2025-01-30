# DockSprout 🌱🐳

**A simple CLI tool to bring up Docker containers from multiple `docker-compose.yml` files in subdirectories.**

---
## **📌 Features**

- Scans a **central directory** for `docker-compose.yml` files.
- Recursively searches **subdirectories** for Docker Compose projects.
- Automatically **brings up (or down)** all discovered containers using `docker compose`.
- Use a `.compose-ignore`file to ignore certain directories of containers you don't want brought up.
- A lightweight and convenient **automation tool** for managing multiple Docker services.

---
## **🛠 Installation** 
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


