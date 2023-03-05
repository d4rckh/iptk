use crate::net::{Networkv4, IPv4};

fn get_power2(n: u32) -> u32 {
  let mut e = 0;

  while n > u32::pow(2, e) {
    e += 1;
  }

  e
}

#[derive(Debug)]
pub struct VLSMIterator {
  pub base_network: Networkv4,
  pub next_network_id: IPv4,
  pub vlsm_sizes: Vec<u32>,
  pub index: usize,
  pub max_hosts: u32,
  pub needed_hosts: u32  
}

impl VLSMIterator {
  pub fn new(base_network: Networkv4, vlsm_sizes: Vec<u32>) -> VLSMIterator {
    VLSMIterator { base_network, next_network_id: base_network.id, vlsm_sizes, index: 0, max_hosts: 0, needed_hosts: 0 }
  }
}

impl Iterator for VLSMIterator {
  type Item = Networkv4;

  fn next(&mut self) -> Option<Self::Item> {
    let vlsm_size = self.vlsm_sizes.get(self.index).expect("Error while subnetting.");
    let mask_size = 32 - get_power2(vlsm_size + 2);
    let mut subnet = Networkv4::from_ip(self.next_network_id, IPv4::from_mask(mask_size));
    
    subnet.needed_hosts = *vlsm_size;
    subnet.mask_size = mask_size;

    if self.max_hosts + subnet.hosts > self.base_network.hosts {
      println!("-> couldn't finish subnetting: not enough space");
      return None;
    }

    self.needed_hosts += vlsm_size;
    self.max_hosts += subnet.hosts;

    self.next_network_id = subnet.broadcast + 1;

    Some(subnet) 
  }
}