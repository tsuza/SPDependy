use async_recursion::async_recursion;
use clap::Parser;
use regex::Regex;
use reqwest::{self};
use std::error;
use std::fs;
use std::io::{self};
use std::path;

mod structs;

use crate::structs::args::SPDependyArgs;
use crate::structs::gh_repo_api::RepositoryLayout;
use crate::structs::sourcepawn_dependencies::{Dependency, DependencyConfigFile};

const GITHUB_LINK_REGEX_EXPRESSION: &str = "https://github.com/(?P<username>[^/]+)/(?P<repository>[^/]+)(?:/(?:tree|blob)/(?P<branch>[^/]+)/?)?(?P<path>.*)?$";

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    let args = SPDependyArgs::parse();

    let sourcepawn_dependencies: Vec<Dependency> =
        parse_dependencies(&args.dependencies_config_path)?;

    for dependency in &sourcepawn_dependencies {
        let api_url = get_api_url(&dependency.url)?;
        let output_path = format!("{}{}", args.output_path, dependency.path);

        download_dependency(&args.token, &api_url, &output_path).await?;
    }

    Ok(())
}

fn parse_dependencies(config_path: &String) -> Result<Vec<Dependency>, io::Error> {
    let file_contents =
        fs::read_to_string(config_path)?;

    let dependencies: DependencyConfigFile = toml::from_str(&file_contents).unwrap();

    Ok(dependencies.dependency)
}

#[async_recursion]
async fn download_dependency(
    token: &String,
    url: &String,
    directory: &String,
) -> Result<(), Box<dyn error::Error>> {
    let client = reqwest::Client::new();

    let request = client
        .get(url)
        .header(reqwest::header::AUTHORIZATION, token)
        .header(reqwest::header::USER_AGENT, "SPDependy")
        .build()?;

    let response = client.execute(request).await?;

    let repository_layout: RepositoryLayout = response.json().await?;

    let repository_layout = repository_layout.parse();

    if !path::Path::new(&directory).try_exists()? {
        fs::create_dir_all(directory)?;
    }

    for file in repository_layout.iter() {
           let new_file_path = format!("{}/{}", directory, file.name);

        match file.file_type.as_str() {
            "dir" => {
                download_dependency(token, &file.links.link, &new_file_path).await?;

                continue;
            }
            "file" => {
                if !file.name.ends_with(".inc") {
                    continue;
                }

                let request = client
                    .get(&file.download_url)
                    .header(reqwest::header::AUTHORIZATION, token)
                    .header(reqwest::header::USER_AGENT, "SPDependy")
                    .build()?;

                let response = client.execute(request).await?;

                let file_contents = response.text().await?;

                fs::write(new_file_path, file_contents)?;
            }
            _ => (),
        };
    }

    Ok(())
}

fn get_api_url(url: &str) -> Result<String, Box<dyn error::Error>> {
    let re = Regex::new(GITHUB_LINK_REGEX_EXPRESSION)?; // See if you can make this global, or at least created only once.

    let test = re.captures(url).unwrap();

    let username = test.name("username").unwrap().as_str();
    let repository = test.name("repository").unwrap().as_str();
    let path = test.name("path").unwrap().as_str();
    let branch = test.name("branch");

    let mut api_url = format!(
        "https://api.github.com/repos/{}/{}/contents/{}",
        username, repository, path
    );

    if branch.is_some() {
        api_url = format!("{}?ref={}", api_url, branch.unwrap().as_str());
    }

    Ok(api_url)
}
