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

fn main() {
  let command = env::args().nth(1).expect("Provide a command");

  if command == "ip4" {
    
    let ip4 = env::args().nth(2).expect("Provide an IP address.");
    let mask = env::args().nth(3).expect("Provide a network mask.");

    let ipdec = parse_ip4(ip4.as_str());
    let maskdec = parse_ip4(mask.as_str());

    let niddec = ipdec & maskdec;
    let broadcastdec = ipdec | !maskdec;
    let nid = dec_to_ip4(niddec);
    let broadcast = dec_to_ip4(broadcastdec);
    let hosts = broadcastdec - niddec - 1; 

    println!("IP:   {ip4}");
    println!("Mask: {mask}");

    println!("-- Network Information --");
    println!("Network Id:  {nid}");
    println!("Broadcast:   {broadcast}");
    println!("Hosts:       {hosts}");
  }
}
