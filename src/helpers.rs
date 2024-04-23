use anyhow::Error;
use fluentci_pdk::dag;

pub fn setup_sqruff() -> Result<String, Error> {
    let stdout = dag()
        .pkgx()?
        .with_exec(vec![&format!(
            "type sqruff > /dev/null || pkgx curl -sSf https://raw.githubusercontent.com/tsirysndr/sqruff/main/install.sh | bash"
        )])?
        .stdout()?;

    Ok(stdout)
}
