mod net;

use std::env;
use net::{IPv4, Networkv4, IPv6, Networkv6};

fn get_power2(n: u32) -> u32 {
  let mut e = 0;

  while n > u32::pow(2, e) {
    e += 1;
  }

  e
}

fn main() {

  let mut args = env::args();

  args.next(); // skip first arg

  let command = args.next().expect("Provide a command");
  
  // IPv6
  if command == "ip6" || command == "ipv6" || command == "6" {
    let ip_arg = args.next().expect("Provide an IPv6 address.");
    
    let ip = IPv6::from_str(ip_arg.as_str());
    let network = Networkv6::from_ip(ip, 64);
    println!("Parsed Ip:  {}", ip);
    println!("-- Network Information --");
    println!("Id:         {}", network.id);
    println!("Broadcast:  {}", network.broadcast);
    println!("Mask:       {}", network.mask);
  }

  // IPv4 section
  if command == "ip4" || command == "ipv4" || command == "4" || command == "vlsm" {
    let ip_arg = args.next().expect("Provide an IPv4 address.");

    let network = 
      if ip_arg.contains("/") {
        let ip_split = ip_arg.split("/").collect::<Vec<&str>>();
        let ip = IPv4::from_str(ip_split.get(0).unwrap());
        let mask = IPv4::from_mask(ip_split.get(1).unwrap().parse::<u32>().unwrap());
  
        Networkv4::from_ip(ip, mask)
      } else {
        let mask_arg = args.next().expect("Provide a subnet mask or use slash notation with the IP address.");
        let ip = IPv4::from_str(ip_arg.as_str());
        let mask = IPv4::from_str(mask_arg.as_str());
  
        Networkv4::from_ip(ip, mask)
      };  

    println!("-- Network Information --");
    println!("Id:         {}  \t{0:b}", network.id);
    println!("Broadcast:  {}  \t{0:b}", network.broadcast);
    println!("Mask:       {}  \t{0:b}", network.mask);
    println!("Wildcard:   {}  \t{0:b}", network.wildcard);
    println!("Hosts:      {}", network.hosts);
  
    if command == "vlsm" {
      
      println!("-- VLSM --");

      let mut total_needed_hosts: u32 = 0;
      let mut max_hosts: u32 = 0;

      let mut vlsm_sizes = args.map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();

      vlsm_sizes.sort();
      vlsm_sizes.reverse();

      let mut next_network_id = network.id;

      for vlsm_size in vlsm_sizes {
        let mask_size = 32 - get_power2(vlsm_size + 2);
        let subnet = Networkv4::from_ip(next_network_id, IPv4::from_mask(mask_size));

        if max_hosts + subnet.hosts > network.hosts {
          println!("-> couldn't finish subnetting: not enough space");
          break;
        }

        total_needed_hosts += vlsm_size;
        max_hosts += subnet.hosts;

        println!("Needed Hosts: {vlsm_size}; Actual Size: {}; {}/{} => {}/{}",
          subnet.hosts, subnet.id, mask_size, subnet.broadcast, mask_size
        );

        next_network_id = subnet.broadcast + 1;
      }

      println!("-- VLSM Stats --");

      let p_susage: f64 = total_needed_hosts as f64 / max_hosts as f64 * 100.0;
      let p_nusage: f64 = max_hosts as f64 / network.hosts as f64 * 100.0;

      println!("Total Needed Hosts: {total_needed_hosts}");
      println!("Max Hosts Available: {max_hosts}");
      println!("% subnet used: {:.1$}%", p_susage, 2);
      println!("% network used: {:.1$}%", p_nusage, 2);

    }
  }
}
