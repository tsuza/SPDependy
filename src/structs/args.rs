use clap::Parser;

const CONFIG_DEFAULT_PATH: &str = "sp.toml";
const OUTPUT_DEFAULT_PATH: &str = ".temp/include/";

#[derive(Parser)]
#[clap(author, version, about)]
pub struct SPDependyArgs {
    /// Github token. https://github.com/settings/tokens
    #[arg(short, long)]
    pub token: String,

    /// Path to the file where the dependencies ( includes ) are listed.
    #[arg(short, long, default_value_t = CONFIG_DEFAULT_PATH.to_string())]
    pub dependencies_config_path: String,

    /// Path to the ouput folder where you want the dependencies ( includes ) to be downloaded.
    #[arg(short, long, default_value_t = OUTPUT_DEFAULT_PATH.to_string())]
    pub output_path: String,
}
