use core::fmt;

#[derive(Debug, Clone, Copy)]
pub struct IPv6 {
  pub dec: u128,
}


#[derive(Debug, Clone, Copy)]
pub struct Networkv6 {
  pub id: IPv6,
  pub broadcast: IPv6,
  pub mask: IPv6,
  pub hosts: u128
}

impl Networkv6 {
  pub fn from_ip(ip: IPv6, mask: u32) -> Networkv6 {
    let mask = IPv6::from_mask(mask);
    let broadcast = ip | !mask;
    let id = ip & mask;
    Networkv6 { 
      id, broadcast, mask, 
      hosts: broadcast.dec - id.dec - 1 
    }
  }
}

impl IPv6 {
  pub fn from_mask(mask: u32) -> IPv6 {
    IPv6::from_dec(!(u128::pow(2, 128 - mask) - 1))
  }

  pub fn from_str(str: &str) -> IPv6 {
    let mut ip_hexs: Vec<&str> = vec![ ];
    let ip_parts: Vec<&str> = str.split("::").collect();

    let mut a: Vec<&str> = ip_parts.get(0).unwrap().split(":").map(|x| if x == "" { "0" } else { x }).collect();
    let mut b: Vec<&str> = if ip_parts.len() > 1 {
      ip_parts.get(1).unwrap().split(":").map(|x| if x == "" { "0" } else { x }).collect()
    } else {
      vec![ ]
    };
    

    let mut fill_zeros = 8 - a.len() - b.len();
    
    ip_hexs.append(&mut a);
    
    while fill_zeros > 0 {
      ip_hexs.push("0");
      fill_zeros -= 1;
    }

    ip_hexs.append(&mut b);

    let ip_dec_part: Vec<u128> = ip_hexs.iter().map(|&f| u128::from_str_radix(f, 16).unwrap()).collect();

    IPv6::from_dec(
      ( ip_dec_part.get(0).unwrap() << 112 ) +
      ( ip_dec_part.get(1).unwrap() << 96 ) +
      ( ip_dec_part.get(2).unwrap() << 80 ) +
      ( ip_dec_part.get(3).unwrap() << 64 ) +
      ( ip_dec_part.get(4).unwrap() << 48 ) +
      ( ip_dec_part.get(5).unwrap() << 32 ) +
      ( ip_dec_part.get(6).unwrap() << 16 ) +
      ( ip_dec_part.get(7).unwrap() )
    )  
  }

  pub fn from_dec(dec: u128) -> IPv6 {
    IPv6 { dec }
  }
}

impl fmt::Display for IPv6 {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", [
      ( self.dec >> 112 ) & 65535,
      ( self.dec >> 96 ) & 65535,
      ( self.dec >> 80 ) & 65535,
      ( self.dec >> 64 ) & 65535,
      ( self.dec >> 48 ) & 65535,
      ( self.dec >> 32 ) & 65535, 
      ( self.dec >> 16 ) & 65535,
      ( self.dec ) & 65535
    ].map(|f| format!("{:x}", f)).join(":"))
  }
}

impl std::ops::Add<u128> for IPv6 {
  type Output = IPv6;
  fn add(self, _rhs: u128) -> IPv6 { IPv6::from_dec(self.dec + _rhs) }
}

impl std::ops::Add<IPv6> for IPv6 {
  type Output = IPv6;
  fn add(self, _rhs: IPv6) -> IPv6 { IPv6::from_dec(self.dec + _rhs.dec) }
}

impl std::ops::Sub<u128> for IPv6 {
  type Output = IPv6;
  fn sub(self, _rhs: u128) -> IPv6 { IPv6::from_dec(self.dec - _rhs) }
}

impl std::ops::Sub<IPv6> for IPv6 {
  type Output = IPv6;
  fn sub(self, _rhs: IPv6) -> IPv6 { IPv6::from_dec(self.dec - _rhs.dec) }
}

impl std::ops::BitAnd<IPv6> for IPv6 {
  type Output = IPv6;
  fn bitand(self, _rhs: IPv6) -> IPv6 { IPv6::from_dec(self.dec & _rhs.dec) }
}

impl std::ops::BitOr<IPv6> for IPv6 {
  type Output = IPv6;
  fn bitor(self, _rhs: IPv6) -> IPv6 { IPv6::from_dec(self.dec | _rhs.dec) }
}

impl std::ops::Not for IPv6 {
  type Output = IPv6;
  fn not(self) -> IPv6 { IPv6::from_dec(!self.dec) }
}

#[derive(Debug, Clone, Copy)]
pub struct IPv4 {
  pub dec: u32,
}

#[derive(Debug, Clone, Copy)]
pub struct Networkv4 {
  pub id: IPv4,
  pub broadcast: IPv4,
  pub mask: IPv4,
  pub wildcard: IPv4,
  pub hosts: u32
}

impl Networkv4 {
  pub fn from_ip(ip: IPv4, mask: IPv4) -> Networkv4 {
    let broadcast = ip | !mask;
    let id = ip & mask;
    Networkv4 { 
      id, broadcast, 
      wildcard: !mask, mask, 
      hosts: broadcast.dec - id.dec - 1 
    }
  }
}

impl IPv4 {
  pub fn from_str(str: &str) -> IPv4 {
    let dec_repr = parse_ip4(str); 
    IPv4 { dec: dec_repr }
  }

  pub fn from_dec(dec: u32) -> IPv4 {
    IPv4 { dec }
  }

  pub fn from_mask(mask: u32) -> IPv4 {
    IPv4::from_dec(!(u32::pow(2, 32 - mask) - 1))
  }
}

impl std::ops::Add<u32> for IPv4 {
  type Output = IPv4;
  fn add(self, _rhs: u32) -> IPv4 { IPv4::from_dec(self.dec + _rhs) }
}

impl std::ops::Add<IPv4> for IPv4 {
  type Output = IPv4;
  fn add(self, _rhs: IPv4) -> IPv4 { IPv4::from_dec(self.dec + _rhs.dec) }
}

impl std::ops::Sub<u32> for IPv4 {
  type Output = IPv4;
  fn sub(self, _rhs: u32) -> IPv4 { IPv4::from_dec(self.dec - _rhs) }
}

impl std::ops::Sub<IPv4> for IPv4 {
  type Output = IPv4;
  fn sub(self, _rhs: IPv4) -> IPv4 { IPv4::from_dec(self.dec - _rhs.dec) }
}

impl std::ops::BitAnd<IPv4> for IPv4 {
  type Output = IPv4;
  fn bitand(self, _rhs: IPv4) -> IPv4 { IPv4::from_dec(self.dec & _rhs.dec) }
}

impl std::ops::BitOr<IPv4> for IPv4 {
  type Output = IPv4;
  fn bitor(self, _rhs: IPv4) -> IPv4 { IPv4::from_dec(self.dec | _rhs.dec) }
}

impl std::ops::Not for IPv4 {
  type Output = IPv4;
  fn not(self) -> IPv4 { IPv4::from_dec(!self.dec) }
}

impl fmt::Display for IPv4 {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", dec_to_ip4(self.dec))
  }
}

impl fmt::Binary for IPv4 {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let bin_repr = dec_to_ip4(self.dec)
      .split(".")
      .map(|x| format!("{:008b}", x.parse::<u8>().unwrap()))
      .collect::<Vec<String>>()
      .join(".");
    write!(f, "{bin_repr}")
  }
}

pub fn parse_ip4(ip: &str) -> u32 {
  let ip_octets: Vec<_> = ip.split(".").map(|s| s.parse::<u32>().unwrap()).collect();

  if ip_octets.len() != 4 {
    panic!("IPv4 must have 4 octets");
  }

  (ip_octets.get(0).unwrap() << 24) + 
  (ip_octets.get(1).unwrap() << 16) + 
  (ip_octets.get(2).unwrap() << 8) + 
  ip_octets.get(3).unwrap()
}

pub fn dec_to_ip4(dec: u32) -> String {
  [ (dec >> 24) & 255, (dec >> 16) & 255, (dec >> 8) & 255, dec & 255 ]
    .iter()
    .map(|c| c.to_string())
    .collect::<Vec<String>>()
    .join(".")
}

