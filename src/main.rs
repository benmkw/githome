fn main() {
    // https://docs.rs/git2/0.9.2/git2/struct.Repository.html#method.discover
    let repo = match ::git2::Repository::discover("./") {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };

    let remote = repo
        .find_remote("origin")
        .or_else(|_| repo.find_remote("github"))
        .or_else(|_| repo.find_remote("gitlab"))
        .or_else(|_| repo.find_remote("gh"))
        .expect("no remote named origin, github, gitlab or gh found");

    if let Some(git_url) = remote.url() {
        let mut final_url = git_url.to_string();

        // convert
        // git@github.com:benmkw/githome.git to
        // https://github.com/benmkw/githome.git
        if !git_url.contains("https://") {
            // git urls are scp-like urls and do not conform to URL RFC
            // see https://github.com/servo/rust-url/issues/220

            // maybe would be better to only replace the last ":"
            // but unicode indexing, yagni
            let ssh_like_url = final_url.replace(":", "/");
            let ssh_url = format!("ssh://{}", ssh_like_url);

            let parsed = ::url::Url::parse(&ssh_url).unwrap();

            final_url = format!("https://{}{}", &parsed.host().unwrap(), &parsed.path());
        }

        ::opener::open(&final_url).unwrap();
    }
}
