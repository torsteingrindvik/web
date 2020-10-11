use anyhow::{anyhow, Result};

pub fn random_name() -> Result<String> {
    Ok(names::Generator::with_naming(names::Name::Numbered)
        .next()
        .ok_or(anyhow!("Couldn't get a random name"))?)
}
