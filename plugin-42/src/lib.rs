use std::time::Duration;

use alumet::{
    measurement::{MeasurementAccumulator, MeasurementPoint, Timestamp},
    metrics::TypedMetricId,
    pipeline::{
        Source,
        elements::{error::PollError, source::trigger::TriggerSpec},
    },
    plugin::rust::AlumetPlugin,
    resources::{Resource, ResourceConsumer},
    units::Unit,
};

pub struct Example42Plugin {}

impl AlumetPlugin for Example42Plugin {
    fn name() -> &'static str {
        "example-42"
    }

    fn version() -> &'static str {
        // Get the version number from Cargo.toml
        env!("CARGO_PKG_VERSION")
    }

    fn init(_config: alumet::plugin::ConfigTable) -> anyhow::Result<Box<Self>> {
        Ok(Box::new(Self {}))
    }

    fn default_config() -> anyhow::Result<Option<alumet::plugin::ConfigTable>> {
        Ok(None)
    }

    fn start(&mut self, alumet: &mut alumet::plugin::AlumetPluginStart) -> anyhow::Result<()> {
        let metric = alumet.create_metric(
            "answer_metric",
            Unit::Unity,
            "the answer to life and everything",
        )?;
        let source = ExampleSource { metric };
        let trigger_spec = TriggerSpec::at_interval(Duration::from_secs(1)); // poll every 1 second
        alumet.add_source("the-answer", Box::new(source), trigger_spec)?;
        Ok(())
    }

    fn stop(&mut self) -> anyhow::Result<()> {
        Ok(())
    }
}

struct ExampleSource {
    metric: TypedMetricId<u64>,
}

impl Source for ExampleSource {
    fn poll(
        &mut self,
        measurements: &mut MeasurementAccumulator,
        timestamp: Timestamp,
    ) -> Result<(), PollError> {
        let value = 42;
        measurements.push(MeasurementPoint::new(
            timestamp,
            self.metric,
            Resource::LocalMachine,
            ResourceConsumer::LocalMachine,
            value,
        ));
        Ok(())
    }
}
