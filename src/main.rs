use expectrl::spawn;

#[cfg(unix)]
const SHELL: &str = "sh";

#[cfg(windows)]
const SHELL: &str = "pwsh";

fn main() {

    let mut p = spawn("docker run --rm -it kalilinux/kali-rolling:latest").expect("failed to spawn");
    
    #[cfg(not(feature = "async"))]
    p.interact().expect("Failed to start interact");

    #[cfg(feature = "async")]
    futures_lite::future::block_on(p.interact()).expect("Failed to start interact");
}

mod tests {
    use expectrl::spawn;

    #[test]
    fn test() {
        let mut p = spawn("docker run --rm -it kalilinux/kali-rolling:latest echo hello").expect("failed to spawn");
        let m = p.expect("hello").expect("no hello there");
        assert_eq!(m.get(0).unwrap(), b"hello");
    }
}