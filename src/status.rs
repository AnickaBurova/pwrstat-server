use serde::Serialize;

#[derive(Serialize)]
pub struct Properties {
    name: String,
    firmware: String,
    rating_voltage: u32,
    rating_power: u32,
}

#[derive(Serialize)]
pub struct Status {
    state: String,
    power_supply_by: String,
    utility_voltage: u32,
    output_voltage: u32,
    battery_capacity: u32,
    remaining_runtime_minutes: u32,
    load_watts: u32,
    load_percent: u32,
    line_interaction: String,
    test_result: String,
    last_power_event: String,
}

#[derive(Serialize)]
pub struct Ups {
    properties: Properties,
    status: Status,
}


mod parse;


#[cfg(test)]
mod test {
    #[test]
    fn test_ups() {
        let lines = "The UPS information shows as following:
	Properties:
		Model Name................... OLS3000E
		Firmware Number.............. CPS-USB CE08
		Rating Voltage............... 230 V
		Rating Power................. 2700 Watt

	Current UPS status:
		State........................ Normal
		Power Supply by.............. Utility Power
		Utility Voltage.............. 234 V
		Output Voltage............... 229 V
		Battery Capacity............. 100 %
		Remaining Runtime............ 37 min.
		Load......................... 405 Watt(15 %)
		Line Interaction............. None
		Test Result.................. Unknown
		Last Power Event............. Blackout at 2020/06/14 16:04:34"
            .lines()
            .collect::<Vec<_>>();
        match super::Ups::from_pwrstat(&lines[..]) {
            Ok(_) => (),
            Err(err) => panic!("Parse failed: {}", err),
        }
    }
}
