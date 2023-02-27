# iptk - ip toolkit

## build
```bash
cargo build
```
## usage
```bash
iptk ip4 192.168.1.1 255.255.255.0

# output:

# IP:   192.168.1.1
# Mask: 255.255.255.0
# -- Network Information --
# Network Id:  192.168.1.0
# Broadcast:   192.168.1.255
# Hosts:       254
```
```bash
iptk vlsm 10.0.0.0 255.255.0.0 100 200 300 400

# output:

# IP:   10.0.0.0
# Mask: 255.255.0.0
# -- Network Information --
# Network Id:  10.0.0.0
# Broadcast:   10.0.255.255
# Hosts:       65534
# -- VLSM --
# Needed Size: 400; Actual Size: 510; 10.0.0.0/23 => 10.0.1.255/23
# Needed Size: 300; Actual Size: 510; 10.0.2.0/23 => 10.0.3.255/23
# Needed Size: 200; Actual Size: 254; 10.0.4.0/24 => 10.0.4.255/24
# Needed Size: 100; Actual Size: 126; 10.0.5.0/25 => 10.0.5.127/25
# -- VLSM Stats --
# Total Needed Hosts: 1000
# Max Hosts Available: 1400
# % subnet used: 71.43%
# % network used: 1.53%
```