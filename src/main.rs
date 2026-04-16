use std::fs;
use std::path::PathBuf;
use dialoguer::{Confirm, FuzzySelect, Input};
use dialoguer::theme::ColorfulTheme;
use licer::{LicenseDetail, LicenseManifest};


#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let license_list: LicenseManifest = reqwest::get("https://raw.githubusercontent.com/spdx/license-list-data/main/json/licenses.json")
        .await?
        .json()
        .await?;

    println!("Fetched license list version: {}", license_list.license_list_version);

    let items: Vec<String> = license_list.licenses.iter().map(|l| format!("{} ({})", l.name.clone(), l.license_id.clone())).collect();

    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Pick a license")
        .items(&items)
        .interact()?;

    let license = &license_list.licenses[selection];

    let file_name: PathBuf = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose a file name")
        .default("LICENSE.txt".into())
        .interact_text()?.into();




    if file_name.exists()  {
        let overwrite_file = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Overwrite existing file?")
            .interact()?;

        if !overwrite_file  {
            println!("Cancelling...");
            return Ok(());
        }
    }

    let license_detail: LicenseDetail = reqwest::get(&license.details_url)
        .await?
        .json()
        .await?;

    fs::write(&file_name, license_detail.license_text)?;
    println!("Successfully written {} to {}", license.name, file_name.display());

    return Ok(())
}
