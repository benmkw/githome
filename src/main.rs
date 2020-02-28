// https://docs.rs/git2/0.9.2/git2/struct.Repository.html#method.discover

fn main() {
    use git2::Repository;

    let repo = match Repository::discover("./") {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };

    let remote = repo
        .find_remote("origin")
        .or_else(|_| repo.find_remote("gh"))
        .expect("no remote named origin or gh");

    if let Some(url) = remote.url() {
        let mut url = url.to_string();
        // convert "git@github.com:benmkw/githome.git" to https
        // a crate would be better here
        if let Some(i) = url.find("@") {
            let (_, end) = url.split_at(i + 1);
            url = format!("https://{}", end.replace(":", "/")).to_string();
        }

        std::process::Command::new("sh")
            .arg("-c")
            .arg("open ".to_string() + &url)
            .output()
            .expect("failed to execute process");
    }
}
