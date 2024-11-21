use clap::{Parser, ValueEnum};


/// This program runs graphical tests for APIs, such as OpenGL or Vulkan. 
/// It run a set of test cases and compares the generated images with a reference image to ensure correctness.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Run single test
    #[arg(short, long, default_value_t = String::from("All"))]
    pub name: String,

    /// Execute tests for a specific graphic API
    #[arg(value_enum, short, long, default_value_t = GraphicApi::All)]
    pub graphic_api: GraphicApi,
}



#[derive(ValueEnum, Clone, Debug)]
pub enum GraphicApi {
    All,
    Opengl,
    Vulkan
}