use gix::bstr::ByteSlice;

fn main() {
    let flags = xflags::parse_or_exit! {
        /// Open the issue tracker (github and gitlab only)
        optional -i,--issues
        /// Open the pull request page (github only)
        optional -p,--pulls
        /// Open the wiki page (github only)
        optional -w,--wiki
        /// Open the contributors overview (github only)
        optional -c,--contributors
    };

    let (path, trust) = gix_discover::upwards(&std::env::current_dir().unwrap()).unwrap();
    assert_eq!(
        trust,
        gix::sec::Trust::Full,
        "this git repository at {path:?} seems suspicious"
    );

    let repo = gix::open(path.as_ref())
        .expect("git discover said a git repo would be here, but it isn't?");

    let remote = repo
        .find_default_remote(gix::remote::Direction::Fetch)
        .unwrap()
        .unwrap();

    let url = remote.url(gix::remote::Direction::Fetch).unwrap().clone();

    let site = if flags.issues {
        "/issues"
    } else if flags.pulls {
        "/pulls"
    } else if flags.wiki {
        "/wiki"
    } else if flags.contributors {
        "/graphs/contributors"
    } else {
        ""
    };

    let path = url.path_argument_safe().unwrap().to_str_lossy();
    let https_url = format!(
        "https://{host}{path}{site}",
        host = url.host_argument_safe().unwrap(),
        path = match path.strip_suffix(".git") {
            None => path,
            Some(s) => format!("/{s}").into(),
        }
    );

    println!("cloned using {scheme}", scheme = url.scheme);
    opener::open(https_url).unwrap();
}
