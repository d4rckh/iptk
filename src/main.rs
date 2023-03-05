mod net;
mod vlsm;

use std::env;
use net::{IPv4, Networkv4, IPv6, Networkv6};
use vlsm::{VLSMIterator};

fn main() {

  let mut args = env::args();

  args.next(); // skip first arg

  let command = args.next().expect("Provide a command");
  
  // IPv6
  if command == "ip6" || command == "ipv6" || command == "6" {
    let ip_arg = args.next().expect("Provide an IPv6 address.");
    let ip_split = ip_arg.split("/").collect::<Vec<&str>>();
    let ip = IPv6::from_str(ip_split.get(0).unwrap());

    let network = 
      if ip_split.len() == 1 {
        Networkv6::from_ip(ip, 64)
      } else {
        let ip = IPv6::from_str(ip_split.get(0).unwrap());
        let mask = ip_split.get(1).unwrap().parse::<u32>().expect("Provide a valid prefix length");
        Networkv6::from_ip(ip, mask)  
      };

    println!("-- IP Information --");
    println!("Parsed Ip:  {}", ip);
    println!("Decimal:    {}", ip.dec);
    println!("-- Network Information --");
    println!("Id:         {}", network.id);
    println!("Mask:       {}", network.mask);
    println!("Hosts:      {}", network.hosts);
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


      let mut vlsm_sizes = args.map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();

      vlsm_sizes.sort();
      vlsm_sizes.reverse();

      let mut vlsm = VLSMIterator::new(network, vlsm_sizes);

      vlsm.by_ref().for_each(|subnet| 
        println!("Needed Hosts: {}; Actual Size: {}; {}/{} => {}/{}",
        subnet.needed_hosts, subnet.hosts, subnet.id, subnet.mask_size, subnet.broadcast, subnet.mask_size)
      );
      
      println!("-- VLSM Stats --");

      let p_susage: f64 = vlsm.needed_hosts as f64 / vlsm.max_hosts as f64 * 100.0;
      let p_nusage: f64 = vlsm.max_hosts as f64 / network.hosts as f64 * 100.0;

      println!("Total Needed Hosts: {}", vlsm.needed_hosts);
      println!("Max Hosts Available: {}", vlsm.max_hosts);
      println!("% subnet used: {:.1$}%", p_susage, 2);
      println!("% network used: {:.1$}%", p_nusage, 2);

    }
  }
}
