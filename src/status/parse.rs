use lazy_static::lazy_static;
use regex::Regex;

use super::*;



fn read_value_str(key: &str, lines: &[&str]) -> Option<(String, usize)> {
    lazy_static! {
        static ref PREFIX: Regex = Regex::new(r"(?m).*\.\.\s(.*)").unwrap();
    }
    for (at, line) in lines.iter().enumerate() {
        let line = line.trim();
        if line.starts_with(key) {
            if let Some(m) = PREFIX.captures(line) {
                return Some((m[1].to_owned(), at));
            }
        }
    }
    None
}

macro_rules! field {
    (exec $lines: ident, $at: expr, [$fun: tt, $name: ident, $key: literal,]) => {
        let ($name, _) = field!($fun $lines, $at, $name, $key);
    };
    (exec $lines: ident, $at: expr, [$fun: tt, $name: ident, $key: literal,
        $next_fun: tt, $next_name: ident, $next_key: literal,
        $($rest: tt,)*]) => {
    //$($fun: to $name: ident, $key: literal,)*) => {
        let ($name, at) = field!($fun $lines, $at, $name, $key);
        field!{exec $lines, at, [$next_fun, $next_name, $next_key, $($rest,)*]}
    };

    (str $lines: ident, $at: expr, $name: ident, $key: literal) => {
        read_value_str($key, &$lines[$at..]).ok_or(anyhow::format_err!("Missing {}", $key))?
    };

    (parse $lines: ident, $at: expr, $name: ident, $key: literal) => {
        read_value_str($key, &$lines[$at..])
            .ok_or(anyhow::format_err!("Missing {}", $key))
            .and_then(|(t, at)| {
                let value = t
                    .split_ascii_whitespace()
                    .next()
                    .ok_or(anyhow::format_err!("Value is not in {}", $key))?;
                value
                    .parse()
                    .map(move |t| (t, at))
                    .map_err(anyhow::Error::from)
            })?
    };
}

impl Properties {
    fn from_pwrstat(lines: &[&str]) -> anyhow::Result<Self> {
        field!(exec lines, 0,
            [str, name, "Model Name",
            str,  firmware, "Firmware Number",
            parse,  rating_voltage, "Rating Voltage",
            parse,  rating_power, "Rating Power", ]
        );
        Ok(Self {
            name,
            firmware,
            rating_voltage,
            rating_power,
        })
    }
}

impl Status {
    pub fn from_pwrstat(lines: &[&str]) -> anyhow::Result<Self> {
        field!(exec lines, 0, [
            str   , state, "State",
            str   , power_supply_by, "Power Supply by",
            parse , utility_voltage, "Utility Voltage",
            parse , output_voltage, "Output Voltage",
            parse , battery_capacity, "Battery Capacity",
            parse , remaining_runtime_minutes, "Remaining Runtime",
            str   , load, "Load",
            str   , line_interaction, "Line Interaction",
            str   , test_result, "Test Result",
            str   , last_power_event, "Last Power Event",
            ]);
        lazy_static! {
            static ref LOAD: Regex = Regex::new(r"(?m)(\d*) Watt\((\d*) %\)").unwrap();
        }
        let (load_watts, load_percent) = if let Some(m) = LOAD.captures(&load) {
            (m[1].parse()?, m[2].parse()?)
        } else {
            anyhow::bail!("Failed to parse load: {}", load);
        };
        Ok(Self {
            state,
            power_supply_by,
            utility_voltage,
            output_voltage,
            battery_capacity,
            remaining_runtime_minutes,
            load_watts,
            load_percent,
            line_interaction,
            test_result,
            last_power_event,
        })
    }
}

impl Ups {
    pub fn from_pwrstat(lines: &[&str]) -> anyhow::Result<Self> {
        let properties = Properties::from_pwrstat(lines)?;
        let status = Status::from_pwrstat(lines)?;
        Ok(Self { properties, status })
    }
}
