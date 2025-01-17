use clap::{Parser, ValueEnum};
use clap_complete::Shell;

use std::path::PathBuf;

#[derive(Debug, Parser)]
pub struct PrettyData {
    #[clap(short, long)]
    /// if true, write directly into file
    pub in_place: bool,

    #[clap(flatten)]
    pub input: BaseInputData,
}

#[derive(Debug, Parser)]
pub struct BaseInputData {
    #[clap(long)]
    /// do not use type mappings
    pub no_map: bool,
    #[clap(long = "tm", long)]
    /// A file containing type mappings.
    ///
    /// If a file with the same name as the script file, but with the extension tym, it
    /// will be used automatically.
    /// e.g.: If there is a file `/generator/script.rhai` and a corresponding
    /// `/generator/script.tym`, it will get used automatically.
    pub typemap: Option<PathBuf>,
    /// which file to use.
    pub file: PathBuf,
}

#[derive(Debug, Parser)]
pub struct BaseOutputData {
    #[clap(long, short)]
    /// The file which should get written with the output from the generator.
    pub out: Option<PathBuf>,
}

#[derive(Clone, Debug, ValueEnum)]
pub enum DataFormat {
    Json,
    JsonPretty,
    Yaml,
    Toml,
    TomlPretty,
}

#[derive(Debug, Parser)]
pub struct DataParameters {
    /// The wasm plugin to use to generate the file.
    pub format: DataFormat,
    #[clap(flatten)]
    pub input: BaseInputData,
    #[clap(flatten)]
    pub out: BaseOutputData,
}

#[cfg(feature = "wasm")]
#[derive(Debug, Parser)]
pub struct WasmParameters {
    /// The wasm plugin to use to generate the file.
    pub wasm: PathBuf,
    #[clap(flatten)]
    pub input: BaseInputData,
    #[clap(flatten)]
    pub out: BaseOutputData,
}

#[derive(Debug, Parser)]
pub enum Generator {
    /// Use a rhai based generator.
    Rhai(RhaiParameters),
    /// Use a handlebars based template.
    /// https://handlebarsjs.com/
    #[cfg(feature = "handlebars")]
    #[clap(aliases=["hbs"])]
    Handlebars(TemplateParameters),
    /// Use a tera based template.
    /// https://tera.netlify.app/
    #[cfg(feature = "tera")]
    Tera(TeraParameters),
    /// Use a liquid based templates.
    /// https://shopify.github.io/liquid/
    #[cfg(feature = "liquid")]
    #[clap(aliases=["lqd"])]
    Liquid(TemplateParameters),
    /// Use a wasm based generator
    #[cfg(feature = "wasm")]
    Wasm(WasmParameters),
    /// Output as serialized data for external use
    Data(DataParameters),
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    /// Print debug representation of the parsed file.
    Debug(BaseInputData),
    /// Pretty print the parsed file.
    Pretty(PrettyData),
    /// Generate source code.
    #[command(subcommand)]
    Generate(Generator),
    /// Print script engine metadata (function definitions, etc.) as json.
    RhaiMetadata,
    /// Write language server file.
    #[clap(hide = true)]
    LanguageServer { out: PathBuf },
    /// Print shell completions.
    #[clap(hide = true)]
    Completions { shell: Shell },
}

#[derive(Debug, Parser)]
pub struct RhaiParameters {
    /// The script to use to generate the file.
    pub script: PathBuf,
    #[clap(long, short)]
    /// Enables debug mode (print and debug function in the script).
    pub debug: bool,
    #[clap(flatten)]
    pub input: BaseInputData,
    #[clap(flatten)]
    pub out: BaseOutputData,
}

#[cfg(any(feature = "liquid", feature = "handlebars"))]
#[derive(Debug, Parser)]
pub struct TemplateParameters {
    /// The template to use to generate the file.
    pub template: PathBuf,
    #[clap(flatten)]
    pub input: BaseInputData,
    #[clap(flatten)]
    pub out: BaseOutputData,
}

#[cfg(feature = "tera")]
#[derive(Debug, Parser)]
pub struct TeraParameters {
    /// Glob path for where to search for templates.
    pub template_dir: String,
    /// The template to use to generate the file.
    pub template_name: String,
    #[clap(long = "tm", long)]
    /// A file containing type mappings.
    pub typemap: Option<PathBuf>,
    /// which file to use.
    pub file: PathBuf,
    #[clap(flatten)]
    pub out: BaseOutputData,
}
