# iptk - ip toolkit

## build

```bash
cargo build
```

## usage

```bash
iptk 4 10.10.10.1/24
# or iptk ip4 10.10.10.1 255.255.255.0

# output:

# -- Network Information --
# Id:         10.10.10.0     00001010.00001010.00001010.00000000
# Broadcast:  10.10.10.255   00001010.00001010.00001010.11111111
# Mask:       255.255.255.0  11111111.11111111.11111111.00000000
# Wildcard:   0.0.0.255      00000000.00000000.00000000.11111111
# Hosts:      254
```
```bash
iptk vlsm 10.0.0.0/16 100 200 300 400
# or iptk vlsm 10.0.0.0 255.255.0.0 100 200 300 400

# output:

# -- Network Information --
# Id:         10.0.0.0      00001010.00000000.00000000.00000000
# Broadcast:  10.0.255.255  00001010.00000000.11111111.11111111
# Mask:       255.255.0.0   11111111.11111111.00000000.00000000
# Wildcard:   0.0.255.255   00000000.00000000.11111111.11111111
# Hosts:      65534
# -- VLSM --
# Needed Hosts: 400; Actual Size: 510; 10.0.0.0/23 => 10.0.1.255/23
# Needed Hosts: 300; Actual Size: 510; 10.0.2.0/23 => 10.0.3.255/23
# Needed Hosts: 200; Actual Size: 254; 10.0.4.0/24 => 10.0.4.255/24
# Needed Hosts: 100; Actual Size: 126; 10.0.5.0/25 => 10.0.5.127/25
# -- VLSM Stats --
# Total Needed Hosts: 1000
# Max Hosts Available: 1400
# % subnet used: 71.43%
# % network used: 2.14%
```
```bash
iptk 6 fe80::3f1c:98c0:4c64:6b44

# warning: it will always use a prefix length of /64 temporarily. (for now)
# output:

# Parsed Ip:  fe80:0:0:0:3f1c:98c0:4c64:6b44
# -- Network Information --
# Id:         fe80:0:0:0:0:0:0:0
# Mask:       ffff:ffff:ffff:ffff:0:0:0:0
```