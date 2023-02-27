# iptk - ip toolkit

## building
```bash
cargo build
```
## usage
```bash
ipcalc ip4 192.168.1.1 255.255.255.0

# output:

# IP:   192.168.1.1
# Mask: 255.255.255.0
# -- Network Information --
# Network Id:  192.168.1.0
# Broadcast:   192.168.1.255
# Hosts:       254

```