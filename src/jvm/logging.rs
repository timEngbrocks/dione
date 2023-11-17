use anyhow::Result;
use log::LevelFilter;
use log4rs::append::rolling_file::policy::compound::roll::fixed_window::FixedWindowRoller;
use log4rs::append::rolling_file::policy::compound::trigger::Trigger;
use log4rs::append::rolling_file::policy::compound::CompoundPolicy;
use log4rs::append::rolling_file::LogFile;
use log4rs::append::rolling_file::RollingFileAppender;
use log4rs::config::Appender;
use log4rs::config::Logger;
use log4rs::config::Root;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::Config;

pub fn setup_logging() {
    let fixed_window_roller_instructions = FixedWindowRoller::builder()
        .build(".log/archive/instructions.{}.log", 5)
        .unwrap();
    let fixed_window_roller_parsed_methods = FixedWindowRoller::builder()
        .build(".log/archive/parsed_methods.{}.log", 5)
        .unwrap();
    let trigger_instructions = RolloverOnStartTrigger::new();
    let trigger_parsed_methods = RolloverOnStartTrigger::new();
    let compound_policy_instructions = CompoundPolicy::new(
        Box::new(trigger_instructions),
        Box::new(fixed_window_roller_instructions),
    );
    let compound_policy_parsed_methods = CompoundPolicy::new(
        Box::new(trigger_parsed_methods),
        Box::new(fixed_window_roller_parsed_methods),
    );
    let config = Config::builder()
        .appender(
            Appender::builder().build(
                "instructions",
                Box::new(
                    RollingFileAppender::builder()
                        .encoder(Box::new(PatternEncoder::new("[{d(%H:%M:%S)}]: {m}{n}")))
                        .build(
                            ".log/instructions.log",
                            Box::new(compound_policy_instructions),
                        )
                        .unwrap(),
                ),
            ),
        )
        .appender(
            Appender::builder().build(
                "parsed_methods",
                Box::new(
                    RollingFileAppender::builder()
                        .encoder(Box::new(PatternEncoder::new("{m}{n}")))
                        .build(
                            ".log/parsed_methods.log",
                            Box::new(compound_policy_parsed_methods),
                        )
                        .unwrap(),
                ),
            ),
        )
        .logger(
            Logger::builder()
                .appender("parsed_methods")
                .build("parsed_methods", LevelFilter::Trace),
        )
        .logger(
            Logger::builder()
                .appender("instructions")
                .build("instructions", LevelFilter::Trace),
        )
        .build(Root::builder().build(LevelFilter::Off))
        .unwrap();
    log4rs::init_config(config).unwrap();
}

#[derive(Debug)]
struct RolloverOnStartTrigger {
    id: usize,
}

impl RolloverOnStartTrigger {
    fn new() -> Self {
        let id = unsafe {
            ROLLOVER_ON_START_TRIGGER_IDS.push(true);
            ROLLOVER_ON_START_TRIGGER_IDS.len() - 1
        };
        RolloverOnStartTrigger { id }
    }
}

static mut ROLLOVER_ON_START_TRIGGER_IDS: Vec<bool> = Vec::new();

impl Trigger for RolloverOnStartTrigger {
    fn trigger(&self, _: &LogFile<'_>) -> Result<bool> {
        unsafe {
            match ROLLOVER_ON_START_TRIGGER_IDS.get(self.id) {
                Some(true) => {
                    ROLLOVER_ON_START_TRIGGER_IDS[self.id] = false;
                    Ok(true)
                }
                _ => Ok(false),
            }
        }
    }
}
