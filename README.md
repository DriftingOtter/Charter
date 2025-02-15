# Charter - Ephemeral Network Resource Leasing *(Pre-Testing Stage)*

> A tool for creating ephemeral ad hoc ports and IPs for testing and development.

## Use Cases

Instead of relying on permanent configurations, **Charter** focuses on short-term network setups, making it ideal for:

- **Developers** testing applications that require temporary networking.
- **Engineers** simulating constrained or ephemeral environments.
- **Teams** deploying short-lived services without long-term configuration overhead.

## Features

- **Automated Setup & Teardown** – Charter automates the creation and cleanup of network resources.
- **Ephemeral Networking** – Resources expire automatically, reducing security risks and resource waste.
- **Simplified Testing** – Ideal for API testing, temporary port forwarding, and simulated network conditions.
- **Cloud & Local Integration** – Works on local machines and supports cloud-based temporary resource allocation.
- **Security by Default** – Ensures proper cleanup by closing ports and removing IP assignments after expiration.

## Installation & Build

Clone the repository and navigate into it:

```bash
git clone https://github.com/driftingotter/charter.git
cd charter
```

Build the project:

```bash
cargo build --release
```

### Optional: Move Binary to Local Binaries Folder

To access Charter without specifying the binary path, move it to your local binaries folder:

```bash
cp target/release/charter ~/.local/bin
```

Ensure `~/.local/bin` is in your PATH. Add it by updating your shell configuration file (`~/.bashrc` or `~/.zshrc`):

```bash
export PATH="$HOME/.local/bin:$PATH"
```

## Usage

Charter accepts various flags and options to configure temporary network resources. Below are sample use cases.

### Example Commands

1. **Lease a Temporary Public IP**

   ```bash
   charter ip --duration 1h --provider aws
   ```
   *(Leases a public IP from AWS for 1 hour.)*

2. **Open a Port for a Limited Time**

   ```bash
   charter port 8080 --duration 30m
   ```
   *(Opens port 8080 for 30 minutes, then automatically closes it.)*

3. **Simulate a Slow Connection**

   ```bash
   charter throttle --bandwidth 100kbps --process myapp
   ```
   *(Throttles the `myapp` process to 100 kbps bandwidth.)*

4. **Create an Ephemeral VPN Tunnel**

   ```bash
   charter vpn --duration 2h --config vpn.conf
   ```
   *(Creates a VPN tunnel using the provided configuration for 2 hours.)*

## Authors

- **Daksh Kaul** *(driftingotter 🦦)*

## Citations

For port creation logic:
- Jackson, Elliot. *"How to Find an Available TCP Port in Rust."* Elliot Jackson's Blog, 25 July 2017, [https://elliotekj.com/posts/2017/07/25/find-available-tcp-port-rust](https://elliotekj.com/posts/2017/07/25/find-available-tcp-port-rust).


