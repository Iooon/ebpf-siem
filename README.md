# eBPF-SIEM (Proof of Concept)

A **Security Information and Event Management (SIEM)** system designed as a proof of concept to demonstrate how modern Linux kernel capabilities (eBPF) can be used for event monitoring. The system is composed of a Linux client agent that captures system data and a backend server that receives and stores this data in a scalable MongoDB database.

⚠️ **Note:** This project is **experimental** and **not production-ready**. It's intended for educational and exploratory purposes only.

---

## 📦 Project Structure

```
.
├── server/         # Async Rust server to receive and store client data
├── go-client/      # eBPF-based Linux client agent written in Go
├── client/         # Alternative client written in Rust using procfs
├── mongo-docker/   # Docker-based MongoDB setup
```

---

## 🔧 Components

### 1. Server (`./server/`)

The server is implemented in **asynchronous Rust** and acts as the central receiver for all client data. It stores the events in a MongoDB instance.

#### 🔨 Build
```bash
cargo build -p server
```

#### 🚀 Run
```bash
cargo run -p server
```

---

### 2. Linux Client (Go + eBPF) (`./go-client/`)

This client uses **eBPF** to collect system information on Linux machines. It is written in **Go**, leveraging low-level kernel features for sandboxed, efficient event tracing.

> ✅ Only runs on **Linux** with eBPF support.

---

### 3. Alternative Rust Client (`./client/`)

This version of the client is written in **Rust**, using the [procfs](https://docs.rs/procfs/latest/procfs/) crate to monitor process changes via polling instead of eBPF.

#### 🔨 Build
```bash
cargo build -p client
```

#### 🚀 Run
```bash
cargo run -p client
```

---

### 4. MongoDB Database (`./mongo-docker/`)

The MongoDB instance is set up using Docker Compose to provide a scalable backend for event storage.

#### 🐳 Start MongoDB
```bash
cd mongo-docker
docker compose up -d
```

---

## 💡 Platform Support

- **Linux:** Full support using eBPF.
- **Windows (Experimental):** With recent developments in [eBPF for Windows](https://github.com/microsoft/ebpf-for-windows/releases), support may be extended in the future.

---

## 🚧 Limitations

- The project is a **proof of concept** and lacks production-level features like authentication, encryption, error handling, and full event coverage.
- Currently designed for Linux environments due to the reliance on eBPF.
- Data schemas and communication protocols are likely to change.

---

## 📚 Resources

- [eBPF Official Site](https://ebpf.io/)
- [eBPF for Windows (Microsoft)](https://github.com/microsoft/ebpf-for-windows/releases)
- [procfs Rust Crate](https://docs.rs/procfs/latest/procfs/)

---

## 🤝 Contributions

Contributions, issues, and feature requests are welcome! Just keep in mind the project's experimental nature.

---

## 📝 License

This project is open-source and available under the [MIT License](./LICENSE).

---
