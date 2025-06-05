use std::time::Duration;

use alumet::{
    agent::{
        self, config,
        plugin::{PluginSet, UnknownPluginInConfigPolicy},
    },
    pipeline, static_plugins,
};

use plugin_42::Example42Plugin;
use plugin_csv::CsvPlugin;

fn main() {
    // Initialize the logger (otherwise you won't see much).
    init_logger();

    // Load the plugin metadata
    let mut plugins = PluginSet::from(static_plugins![CsvPlugin, Example42Plugin]);

    // Load the configuration file.
    let general = || toml::Table::new(); // no default general options
    let default_config_provider = config::AutoDefaultConfigProvider::new(&plugins, general);
    let mut config = config::Loader::parse_file("alumet-config.toml")
        .or_default(default_config_provider, true) // if the config file does not exist, generate it
        .load() // load now
        .expect("could not load config file");

    // Apply the configuration to the plugins.
    plugins
        .extract_config(&mut config, true, UnknownPluginInConfigPolicy::Error)
        .expect("invalid plugins config");

    // Enable the example plugin "42" manually.
    // It is disabled by extract_config if it does not appear in the config, here's how to enable it by force.
    plugins.set_plugin_enabled("example-42", true);

    // Set up the measurement pipeline
    let mut pipeline = pipeline::Builder::new();
    pipeline.normal_threads(2); // Example setting: use 2 threads to run async pipeline elements

    // Build and start the agent
    let agent = agent::Builder::from_pipeline(plugins, pipeline)
        .build_and_start()
        .expect("startup failure");

    // Run until shutdown (you can use Ctrl+C to initiate shutdown from the terminal)
    agent
        .wait_for_shutdown(Duration::MAX)
        .expect("error while running");
}

pub fn init_logger() {
    use env_logger::{Builder, Env};

    Builder::from_env(Env::default().default_filter_or("info")).init();
}
