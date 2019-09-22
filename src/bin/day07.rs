use std::io;
use std::io::prelude::*;

fn find_bab_outside(ip: &str, bab: &Vec<char>) -> bool {
    for s in ip.split(']') {
        let s: Vec<char> = s.chars().collect();
        for i in 0..s.len() - 2 {
            if s[i + 2] == '[' {
                break;
            }
            if s[i] == bab[0] && s[i + 1] == bab[1] && s[i + 2] == bab[2] {
                return true;
            }
        }
    }
    false
}

fn supports_ssl(ip: &str) -> bool {
    let mut i = 0;
    for s in ip.split('[') {
        if i != 0 {
            let s: Vec<char> = s.chars().collect();
            for i in 0..s.len() - 2 {
                if s[i + 2] == ']' {
                    break;
                }
                if s[i + 2] == s[i] {
                    let bab = vec![s[i + 1], s[i], s[i + 1]];
                    if find_bab_outside(ip, &bab) {
                        return true;
                    }
                }
            }
        }
        i += 1;
    }
    false
}

fn abba_outside(ip: &str) -> bool {
    for s in ip.split(']') {
        let s: Vec<char> = s.chars().collect();
        for i in 0..s.len() - 3 {
            if s[i + 3] == '[' {
                break;
            }
            if s[i + 3] == s[i] && s[i + 2] == s[i + 1] && s[i] != s[i + 1] {
                return true;
            }
        }
    }
    false
}

fn abba_inside(ip: &str) -> bool {
    let mut i = 0;
    for s in ip.split('[') {
        if i != 0 {
            let s: Vec<char> = s.chars().collect();
            for i in 0..s.len() - 3 {
                if s[i + 3] == ']' {
                    break;
                }
                if s[i + 3] == s[i] && s[i + 2] == s[i + 1] && s[i] != s[i + 1] {
                    return true;
                }
            }
        }
        i += 1;
    }
    false
}

fn supports_tls(ip: &str) -> bool {
    !abba_inside(ip) && abba_outside(ip)
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    let input = &input[..];

    let tls = input.split('\n').filter(|s| supports_tls(s)).count();
    println!("Number of IPs that support TLS: {}", tls);

    let ssl = input.split('\n').filter(|s| supports_ssl(s)).count();
    println!("Number of IPs that support SSL: {}", ssl);

    Ok(())
}
