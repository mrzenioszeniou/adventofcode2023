use crate::utils::lcm_many;
use anyhow::Context;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::fmt::Debug;
use std::hash::Hash;
use std::str::FromStr;

pub fn solve() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("res/day20.txt")?;

    let mut modules = BTreeMap::new();
    for line in input.lines() {
        let module = Module::from_str(line)?;
        modules.insert(module.id().clone(), module);
    }

    // prime the conjuction modules
    for id in modules.keys().cloned().collect::<Vec<_>>() {
        let dests = modules.get(&id).context("wat")?.dest().to_vec();

        for dest in dests {
            if dest == "output" {
                continue;
            }

            if let Some(Module::Conjuction { values, .. }) = modules.get_mut(&dest) {
                values.insert(id.clone(), PulseValue::Low);
            }
        }
    }

    let mut bus = VecDeque::new();
    let mut low_pulses = 0;
    let mut high_pulses = 0;

    let mut nand_freqs = HashMap::from([
        ("sk".to_string(), None),
        ("sv".to_string(), None),
        ("dr".to_string(), None),
        ("qz".to_string(), None),
    ]);

    'outer: for i in 0.. {
        // send button pulse
        bus.push_back(Pulse {
            from: "button".to_string(),
            to: "broadcaster".to_string(),
            value: PulseValue::Low,
        });

        // handle all subsequent pulses
        while let Some(pulse) = bus.pop_front() {
            if i < 1_000 {
                match pulse.value {
                    PulseValue::Low => low_pulses += 1,
                    PulseValue::High => high_pulses += 1,
                }
            }

            if matches!(pulse.value, PulseValue::Low) && nand_freqs.get(&pulse.from) == Some(&None)
            {
                println!("{} LOW after {} presses", pulse.from, i + 1);
                nand_freqs.insert(pulse.from.clone(), Some(i + 1));
                if nand_freqs.values().all(|nand| nand.is_some()) {
                    break 'outer;
                }
            }

            if let Some(dest_module) = modules.get_mut(&pulse.to) {
                dest_module.handle_pulse(pulse, &mut bus);
            }
        }
    }

    let part_1 = low_pulses * high_pulses;

    let freqs = nand_freqs.values().filter_map(|v| *v).collect::<Vec<_>>();
    let part_2 = lcm_many(&freqs);

    println!("Part 1: {part_1}\nPart 2: {part_2}");
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Module {
    FlipFlop {
        id: ModuleId,
        value: PulseValue,
        dest: Vec<ModuleId>,
    },

    Conjuction {
        id: ModuleId,
        values: BTreeMap<ModuleId, PulseValue>,
        dest: Vec<ModuleId>,
    },

    Broadcast {
        id: ModuleId,
        dest: Vec<ModuleId>,
    },
}

impl Module {
    fn id(&self) -> &ModuleId {
        match self {
            Module::FlipFlop { id, .. }
            | Module::Conjuction { id, .. }
            | Module::Broadcast { id, .. } => id,
        }
    }

    fn dest(&self) -> &[ModuleId] {
        match self {
            Module::FlipFlop { dest, .. }
            | Module::Conjuction { dest, .. }
            | Module::Broadcast { dest, .. } => dest,
        }
    }

    fn handle_pulse(&mut self, pulse: Pulse, bus: &mut VecDeque<Pulse>) {
        assert_eq!(
            &pulse.to,
            self.id(),
            " module '{}' received pulse meant for '{}': {pulse:?}",
            self.id(),
            pulse.from
        );

        match self {
            Module::FlipFlop { id, value, dest } => {
                if matches!(pulse.value, PulseValue::Low) {
                    value.flip();
                    for dest in dest.iter() {
                        bus.push_back(Pulse {
                            from: id.clone(),
                            to: dest.clone(),
                            value: *value,
                        });
                    }
                }
            }

            Module::Conjuction { id, values, dest } => {
                values.insert(pulse.from, pulse.value);

                let value = values.values().any(|v| matches!(v, PulseValue::Low)).into();

                for dest in dest.iter() {
                    bus.push_back(Pulse {
                        from: id.clone(),
                        to: dest.clone(),
                        value,
                    });
                }
            }

            Module::Broadcast { id, dest } => {
                for dest in dest.iter() {
                    bus.push_back(Pulse {
                        from: id.clone(),
                        to: dest.clone(),
                        value: pulse.value,
                    });
                }
            }
        }
    }
}

impl FromStr for Module {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(" -> ");

        let module_str = split.next().context("missing module string")?;
        let dest_str = split.next().context("missing destination string")?;

        let dest = dest_str.split(", ").map(|s| s.to_string()).collect();

        if module_str == "broadcaster" {
            Ok(Self::Broadcast {
                id: module_str.to_string(),
                dest,
            })
        } else if let Some(id) = module_str.strip_prefix('%') {
            Ok(Self::FlipFlop {
                id: id.to_string(),
                value: PulseValue::Low,
                dest,
            })
        } else if let Some(id) = module_str.strip_prefix('&') {
            Ok(Self::Conjuction {
                id: id.to_string(),
                values: Default::default(),
                dest,
            })
        } else {
            anyhow::bail!("unexpected module string: '{s}'");
        }
    }
}

impl std::fmt::Display for Module {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FlipFlop { value, .. } => write!(f, "{value}")?,
            Self::Conjuction { .. } | Self::Broadcast { .. } => {}
        }

        Ok(())
    }
}

#[derive(Debug)]
struct Pulse {
    from: ModuleId,
    to: ModuleId,
    value: PulseValue,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum PulseValue {
    Low,
    High,
}

impl From<&PulseValue> for usize {
    fn from(value: &PulseValue) -> Self {
        match value {
            PulseValue::Low => 0,
            PulseValue::High => 1,
        }
    }
}

impl From<bool> for PulseValue {
    fn from(value: bool) -> Self {
        match value {
            true => Self::High,
            false => Self::Low,
        }
    }
}

impl From<PulseValue> for bool {
    fn from(value: PulseValue) -> Self {
        match value {
            PulseValue::Low => false,
            PulseValue::High => true,
        }
    }
}

impl PulseValue {
    fn flip(&mut self) {
        match self {
            Self::Low => *self = Self::High,
            Self::High => *self = Self::Low,
        }
    }
}

impl std::fmt::Display for PulseValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PulseValue::Low => write!(f, "0"),
            PulseValue::High => write!(f, "1"),
        }
    }
}

type ModuleId = String;
