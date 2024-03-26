use clap::Parser;

#[derive(Parser, Debug)]
#[clap()]
pub struct Opts {
    //#[clap(short = 'a', long = "args")]
    //pub args: Vec<String>,
    #[clap(short = 'd', long = "dataset")]
    pub dataset: Option<String>,
    #[clap(short = 'r', long = "range")]
    pub range: Option<String>,
    #[clap(short = 'f', long = "fields",num_args(0..))]
    pub fields: Option<Vec<String>>,
    #[clap(short = 'o', long = "options", num_args(0..))]
    pub options: Option<Vec<String>>,
}
