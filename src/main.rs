use std::env;

fn parse_ip4(ip: &str) -> u32 {
  let ip_octets: Vec<_> = ip.split(".").map(|s| s.parse::<u32>().unwrap()).collect();

  if ip_octets.len() != 4 {
    panic!("IPv4 must have 4 octets");
  }

  (ip_octets.get(0).unwrap() << 24) + 
  (ip_octets.get(1).unwrap() << 16) + 
  (ip_octets.get(2).unwrap() << 8) + 
  ip_octets.get(3).unwrap()
}

fn dec_to_ip4(dec: u32) -> String {
  [ (dec >> 24) & 255, (dec >> 16) & 255, (dec >> 8) & 255, dec & 255 ]
    .iter().map(|c| c.to_string()).collect::<Vec<String>>().join(".")
}

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

  let ip4 = args.next().expect("Provide an IP address.");
  let mask = args.next().expect("Provide a network mask.");

  let ipdec = parse_ip4(ip4.as_str());
  let maskdec = parse_ip4(mask.as_str());

  let niddec = ipdec & maskdec;
  let broadcastdec = ipdec | !maskdec;
  let nid = dec_to_ip4(niddec);
  let broadcast = dec_to_ip4(broadcastdec);
  let hosts = broadcastdec - niddec - 1; 

  if command == "ip4" || command == "vlsm" {

    println!("IP:   {ip4}");
    println!("Mask: {mask}");

    println!("-- Network Information --");
    println!("Network Id:  {nid}");
    println!("Broadcast:   {broadcast}");
    println!("Hosts:       {hosts}");
  
  } 
  
  if command == "vlsm" {
    
    println!("-- VLSM --");

    let mut total_needed_hosts: u32 = 0;
    let mut max_hosts: u32 = 0;

    let mut vlsm_sizes = args.map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();

    vlsm_sizes.sort();
    vlsm_sizes.reverse();

    let mut last_niddec = niddec;

    for vlsm_size in vlsm_sizes {
      let mask = 32 - get_power2(vlsm_size + 2);
      let shosts = u32::pow(2, 32 - mask) - 2;
      let niddec = last_niddec;
      let broadcastdec = niddec + shosts + 1;
      let nid = dec_to_ip4(niddec);
      let bc = dec_to_ip4(broadcastdec);

      if max_hosts + shosts > hosts {
        println!("-> couldn't finish subnetting: not enough space");
        break;
      }

      total_needed_hosts += vlsm_size;
      max_hosts += shosts;

      println!("Needed Size: {vlsm_size}; Actual Size: {shosts}; {nid}/{mask} => {bc}/{mask}");

      last_niddec = broadcastdec + 1;
    }

    println!("-- VLSM Stats --");

    let p_susage: f64 = total_needed_hosts as f64 / max_hosts as f64 * 100.0;
    let p_nusage: f64 = max_hosts as f64 / hosts as f64 * 100.0;

    println!("Total Needed Hosts: {total_needed_hosts}");
    println!("Max Hosts Available: {max_hosts}");
    println!("% subnet used: {:.1$}%", p_susage, 2);
    println!("% network used: {:.1$}%", p_nusage, 2);

  }
}
