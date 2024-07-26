use anyhow::Error;
use fluentci_pdk::dag;

pub fn setup_sqruff() -> Result<String, Error> {
    let gh_token = dag().get_env("GITHUB_ACCESS_TOKEN")?;
    dag().set_envs(vec![("GITHUB_ACCESS_TOKEN".into(), gh_token)])?;

    let stdout = dag()
        .pkgx()?
        .with_exec(vec![&format!(
            "type sqruff > /dev/null || pkgx curl -sSf https://raw.githubusercontent.com/tsirysndr/sqruff/main/install.sh | bash"
        )])?
        .stdout()?;

    Ok(stdout)
}
