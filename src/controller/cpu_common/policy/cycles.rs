use std::{
    fs,
    path::Path,
    thread,
    time::{Duration, Instant},
};

use cpu_cycles_reader::{Cycles, CyclesReader};
use likely_stable::LikelyOption;
use yata::{
    methods::{DEMA, EMA},
    prelude::*,
};

use crate::{config::CONFIG, debug, ThisOption, ThisResult};

enum SpecEma {
    Ema(EMA),
    Dema(DEMA),
}

pub struct DiffReader {
    affected_cpus: Vec<i32>,
    ema: SpecEma,
    reader: CyclesReader,
}

impl SpecEma {
    fn next(&mut self, value: f64) -> f64 {
        match self {
            Self::Ema(e) => e.next(&value),
            Self::Dema(e) => e.next(&value),
        }
    }
}

impl DiffReader {
    pub fn new(path: &Path) -> Self {
        let affected_cpus: Vec<i32> = fs::read_to_string(path.join("affected_cpus"))
            .this_unwrap()
            .split_whitespace()
            .map(|cpu| cpu.parse::<i32>().this_unwrap())
            .collect();

        let window = CONFIG
            .get_conf("EMA_WIN")
            .and_then_likely(|d| u8::try_from(d.as_integer()?).ok())
            .unwrap_or(4);

        let ema = CONFIG
            .get_conf("EMA_TYPE")
            .and_then_likely(|d| match d.as_str()? {
                "EMA" => Some(SpecEma::Ema(EMA::new(window, &0.0).ok()?)),
                "DEMA" => Some(SpecEma::Dema(DEMA::new(window, &0.0).ok()?)),
                _ => None,
            })
            .this_unwrap();

        let reader = CyclesReader::new(affected_cpus.as_slice()).this_unwrap();

        Self {
            affected_cpus,
            ema,
            reader,
        }
    }

    pub fn read_diff(&mut self, cur_freq: Cycles) -> Cycles {
        self.reader.enable();
        let time = Instant::now();
        let cycles_former = self.reader.read().this_unwrap();

        thread::sleep(Duration::from_millis(75));

        let cycles_later = self.reader.read().this_unwrap();
        let time = time.elapsed();
        self.reader.disable();

        let cycles = self
            .affected_cpus
            .iter()
            .map(|cpu| *cycles_later.get(cpu).this_unwrap() - *cycles_former.get(cpu).this_unwrap())
            .max()
            .this_unwrap();

        let cycles = cycles.as_diff(time, cur_freq).this_unwrap();
        #[allow(clippy::cast_possible_truncation)]
        #[allow(clippy::cast_precision_loss)]
        let diff = Cycles::from_khz(self.ema.next(cycles.as_khz() as f64) as i64);

        debug! {
            println!("diff: {}", diff);
        }

        diff
    }
}