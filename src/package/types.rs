use derive_more::From;
use getset::Getters;
use orion_variate::addr::{AddrType, GitAddr, HttpAddr, LocalAddr};
#[derive(Debug, Clone, Getters)]
pub struct BinPackage {
    #[getset(get = "pub")]
    name: String,
    #[getset(get = "pub")]
    addr: AddrType,
}

#[derive(Debug, Clone, Getters)]
pub struct GitPackage {
    #[getset(get = "pub")]
    name: String,
    #[getset(get = "pub")]
    addr: GitAddr,
}
#[derive(Debug, Clone, From)]
pub enum PackageType {
    Bin(BinPackage),
    Git(GitPackage),
}

pub fn  convert_addr(input: &str) -> AddrType {
    if input.starts_with("http") {
        if input.ends_with(".git") {
            AddrType::Git(GitAddr::from(input.to_string()))
        } else if input.ends_with(".tar.gz") {
            AddrType::Http(HttpAddr::from(input.to_string()))
        } else {
            panic!("Unsupported package type: {}", input);
        }
    } else if input.starts_with("git@") {
        AddrType::Git(GitAddr::from(input.to_string()))
    } else {
        if input.ends_with(".git") {
            AddrType::Git(GitAddr::from(input.to_string()))
        } else if input.ends_with(".tar.gz") {
            AddrType::Local(LocalAddr::from(input.to_string()))
        } else {
            panic!("Unsupported package type: {}", input);
        }
    }
}
// input :
// /Users/dayu/ds-build/mac-devkit-0.1.5.tar.gz
// https://github.com/galaxy-sec/galaxy-flow.git
// git@github.com:galaxy-sec/galaxy-flow.git
// https://github.com/galaxy-sec/galaxy-flow/releases/download/v0.8.4/galaxy-flow-v0.8.4-aarch64-apple-darwin.tar.gz
pub fn build_pkg(input: &str) -> PackageType {
    let addr_type = convert_addr(input);
    
    match addr_type {
        AddrType::Git(git_addr) => {
            let name = extract_name_from_url(input, ".git");
            PackageType::Git(GitPackage {
                name,
                addr: git_addr,
            })
        }
        AddrType::Http(http_addr) => {
            let name = extract_name_from_url(input, ".tar.gz");
            PackageType::Bin(BinPackage {
                name,
                addr: AddrType::Http(http_addr),
            })
        }
        AddrType::Local(local_addr) => {
            let name = extract_name_from_url(input, ".tar.gz");
            PackageType::Bin(BinPackage {
                name,
                addr: AddrType::Local(local_addr),
            })
        }
    }
}

fn extract_name_from_url(url: &str, suffix: &str) -> String {
    url.split('/')
        .last()
        .unwrap()
        .replace(suffix, "")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_pkg_bin_local() {
        let input = "/Users/dayu/ds-build/mac-devkit-0.1.5.tar.gz";
        let pkg = build_pkg(input);
        match pkg {
            PackageType::Bin(bin_pkg) => {
                assert_eq!(bin_pkg.name(), "mac-devkit-0.1.5");
                assert!(matches!(bin_pkg.addr(), AddrType::Local(_)));
            }
            _ => panic!("Expected BinPackage"),
        }
    }

    #[test]
    fn test_build_pkg_bin_remote() {
        let input = "https://github.com/galaxy-sec/galaxy-flow/releases/download/v0.8.4/galaxy-flow-v0.8.4-aarch64-apple-darwin.tar.gz";
        let pkg = build_pkg(input);
        match pkg {
            PackageType::Bin(bin_pkg) => {
                assert_eq!(bin_pkg.name(), "galaxy-flow-v0.8.4-aarch64-apple-darwin");
                assert_eq!(bin_pkg.addr(), &AddrType::from(HttpAddr::from(input)));
            }
            _ => panic!("Expected BinPackage"),
        }
    }

    #[test]
    fn test_build_pkg_git_https() {
        let input = "https://github.com/galaxy-sec/galaxy-flow.git";
        let pkg = build_pkg(input);
        match pkg {
            PackageType::Git(git_pkg) => {
                assert_eq!(git_pkg.name(), "galaxy-flow");
                assert_eq!(git_pkg.addr().repo(), input);
            }
            _ => panic!("Expected GitPackage"),
        }
    }

    #[test]
    fn test_build_pkg_git_ssh() {
        let input = "git@github.com:galaxy-sec/galaxy-flow.git";
        let pkg = build_pkg(input);
        match pkg {
            PackageType::Git(git_pkg) => {
                assert_eq!(git_pkg.name(), "galaxy-flow");
                assert_eq!(git_pkg.addr().repo(), input);
            }
            _ => panic!("Expected GitPackage"),
        }
    }

    #[test]
    #[should_panic(expected = "Unsupported package type")]
    fn test_build_pkg_unsupported() {
        let input = "invalid_input";
        build_pkg(input);
    }
}

    #[cfg(test)]
    mod convert_addr_tests {
        use super::*;

        #[test]
        fn test_convert_addr_local() {
            let input = "/Users/dayu/ds-build/mac-devkit-0.1.5.tar.gz";
            let addr = convert_addr(input);
            assert!(matches!(addr, AddrType::Local(_)));
        }

        #[test]
        fn test_convert_addr_http_tar() {
            let input = "https://github.com/galaxy-sec/galaxy-flow/releases/download/v0.8.4/galaxy-flow-v0.8.4-aarch64-apple-darwin.tar.gz";
            let addr = convert_addr(input);
            assert!(matches!(addr, AddrType::Http(_)));
        }

        #[test]
        fn test_convert_addr_https_git() {
            let input = "https://github.com/galaxy-sec/galaxy-flow.git";
            let addr = convert_addr(input);
            assert!(matches!(addr, AddrType::Git(_)));
        }

        #[test]
        fn test_convert_addr_ssh_git() {
            let input = "git@github.com:galaxy-sec/galaxy-flow.git";
            let addr = convert_addr(input);
            assert!(matches!(addr, AddrType::Git(_)));
        }

        #[test]
        fn test_convert_addr_local_git() {
            let input = "/home/user/repo.git";
            let addr = convert_addr(input);
            assert!(matches!(addr, AddrType::Git(_)));
        }

        #[test]
        #[should_panic(expected = "Unsupported package type")]
        fn test_convert_addr_unsupported() {
            let input = "invalid_input";
            convert_addr(input);
        }
    }
