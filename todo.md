### **Key Features**
1. **Ephemeral Public IP Leasing**:
   - Dynamically assign and release public IPs for specific testing scenarios.
   - Optionally bind to cloud providers (AWS, Azure, GCP) to allocate temporary resources.

2. **Dynamic Port Allocation**:
   - Lease local or remote ports for a defined time. *DONE*
   - Automatically configure NAT/firewall rules during the lease period.
   - Close ports and revoke rules automatically after expiration.

3. **Bandwidth Throttling**:
   - Simulate restricted environments by capping bandwidth for specific processes or services (e.g., simulate slow connections for testing).

4. **Custom Subnet Environments**:
   - Spawn temporary, isolated subnets for sandboxing and testing communications between containers, VMs, or local apps.

5. **Time-Limited VPNs or Tunnels**:
   - Automatically configure and run ephemeral VPN connections or SSH tunnels that self-destruct after a specified duration.

6. **Activity Logs and Alerts**:
   - Log all resource usage and notify when leases expire.
   - Option to integrate with monitoring tools for real-time updates.

---

### **Sample Use Cases**
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

