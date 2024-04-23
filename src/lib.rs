use extism_pdk::*;
use fluentci_pdk::dag;

use crate::helpers::setup_sqruff;

pub mod helpers;

#[plugin_fn]
pub fn setup() -> FnResult<String> {
    let stdout = setup_sqruff()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn lint(args: String) -> FnResult<String> {
    setup_sqruff()?;

    let stdout = dag()
        .pipeline("lint")?
        .with_exec(vec!["sqruff", "lint", &args])?
        .stdout()?;

    Ok(stdout)
}

#[plugin_fn]
pub fn fix(args: String) -> FnResult<String> {
    setup_sqruff()?;

    let stdout = dag()
        .pipeline("fix")?
        .with_exec(vec!["sqruff", "fix", &args])?
        .stdout()?;

    Ok(stdout)
}
