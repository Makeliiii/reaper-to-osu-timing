static THOUSAND: f64 = 1000.0;
static TIME_IN_MILLISECONDS: f64 = THOUSAND * 60.0;
static RANDOM_METER_VALUE: i64 = 65536;
static METER_DENOMINATOR: i64 = 4;

struct ReaperTimingPoint {
    time: f64, // Position in seconds
    bpm: f64, // BPM
    meter: Option<i64> // time signature
}

impl ReaperTimingPoint {
    fn new(timing_point: &str) -> ReaperTimingPoint {
        let data: Vec<&str> = timing_point.split(' ').collect();
        let time: f64 = data[1].parse().unwrap();
        let bpm: f64 = data[2].parse().unwrap();
        
        if data.len() >= 5 {
            let meter: Option<i64> = data[4].parse().ok();
            return ReaperTimingPoint {
                time,
                bpm,
                meter
            }
        }

        ReaperTimingPoint {
            time,
            bpm,
            meter: None
        }
    }

    fn convert_to_osu_time(&self) -> i32 {
        (self.time * THOUSAND) as i32
    }

    fn convert_to_osu_beat_length(&self) -> f64 {
        1.0 / self.bpm * TIME_IN_MILLISECONDS
    }
}

struct OsuTimingPoint {
    time: i32, // Position in milliseconds
    beat_length: f64, // beatLength = 1 / BPM * 1000 * 60
    meter: i64, // Amount of beats in a measure
    sample_set: i32, // (0 = default, 1 = normal, 2 = soft, 3 = drum)
    sample_index: i32, // Custom sample index for hit objects. 0 by default
    volume: i32, // Hitobject volume percentage
    uninherited: i32, // Whether or not the timing point is uninherited
    effects: i32, // Bit flags for random shit idk
}

impl OsuTimingPoint {
    fn new(timing_point: ReaperTimingPoint, meter: i64) -> OsuTimingPoint {
        let time = timing_point.convert_to_osu_time();
        let beat_length = timing_point.convert_to_osu_beat_length();

        OsuTimingPoint {
            time,
            beat_length,
            meter,
            sample_set: 0,
            sample_index: 0,
            volume: 100,
            uninherited: 1,
            effects: 0,
        }
    }
}

fn convert_meter(meter: i64) -> i64 {
    meter - RANDOM_METER_VALUE * METER_DENOMINATOR
}

fn convert_to_osu(contents: &str) -> Vec<OsuTimingPoint> {
    let mut old_meter: i64 = 0;

    contents.lines()
        .filter(|line| line.contains("PT "))
        .map(|line| line.trim()) // parse reaper timing points and trim whitespace
        .map(ReaperTimingPoint::new) // convert to reaper timing points
        .map(|timing_point| {
            if let Some(meter) = timing_point.meter {
                old_meter = convert_meter(meter);
            }

            OsuTimingPoint::new(timing_point, old_meter)
        }).collect() // convert to osu timing point format
}

fn print_in_osu_format(osu_timing_points: Vec<OsuTimingPoint>) {
    for OsuTimingPoint { time, beat_length, meter, sample_set, sample_index, volume, uninherited, effects } in osu_timing_points {
        println!("{time},{beat_length},{meter},{sample_set},{sample_index},{volume},{uninherited},{effects}");
    }
}

pub fn run(contents: &str) {
    let osu_points = convert_to_osu(contents);
    print_in_osu_format(osu_points);
}
