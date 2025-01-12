# Charter - Ephemeral Network Resource Leasing __(Pre-testing stage)__

> A tool for creating ephemeral ad hoc ports & IPs for testing and development.

## Usecase

Instead of permanent configurations, **Charter** focuses on short-term setups, making it ideal for:

- Developers testing applications.
- Engineers simulating constrained environments.
- Teams deploying temporary services.

## Features

- **Automation**: Automates setup and teardown of network configurations.
- **Ephemeral Nature**: Focuses on short-lived setups to reduce resource waste and improve security.
- **Testing Made Easy**: Simplifies testing workflows that involve networking, such as API endpoint tests under various conditions.
- **Cloud Integration**: Works locally and integrates with cloud services for temporary resource allocation.
- **Secure by Default**: Automatically cleans up after lease expiration, closing ports, and removing IP assignments.

## Installation and Build

Clone the repository and navigate into it:
```bash
git clone https://github.com/DriftingOtter/Charter.git 
cd Charter
```

Build the project:
```bash
cargo build --release
```

### Optional: Move Binary to Local Binaries Folder

To access `charter` without specifying the binary path, move it to your local binaries folder:
```bash
cp target/release/charter ~/.local/bin
```

Ensure `~/.local/bin` is in your `PATH`. You can add it by updating your shell configuration file (`~/.bashrc` or `~/.zshrc`):
```bash
export PATH="$HOME/.local/bin:$PATH"
```

## Synopsis

### Additional Arguments

Charter accepts various flags and options to configure temporary network resources. Below are some of the sample commands.

### Sample Use Cases

1. **Temporary Public IP for Testing**:
   ```bash
   charter ip --duration 1h --provider aws
   ```
   _(Lease a public IP from AWS for 1 hour.)_

2. **Open a Port for Limited Time**:
   ```bash
   charter port 8080 --duration 30m
   ```
   _(Open port 8080 for 30 minutes, then automatically close it.)_

3. **Simulate a Slow Connection**:
   ```bash
   charter throttle --bandwidth 100kbps --process myapp
   ```
   _(Throttle the `myapp` process to 100 kbps bandwidth.)_

4. **Create an Ephemeral VPN**:
   ```bash
   charter vpn --duration 2h --config vpn.conf
   ```
   _(Spin up a VPN tunnel using the provided configuration for 2 hours.)_

## Authors

- Daksh Kaul // DriftingOtter 🦦

## Citations

For port creation code:
- Jackson, Elliot. "How to Find an Available TCP Port in Rust." Elliot Jackson's Blog, 25 July 2017, https://elliotekj.com/posts/2017/07/25/find-available-tcp-port-rust.

