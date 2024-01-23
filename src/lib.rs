static THOUSAND: f64 = 1000.0;
static TIME_IN_MILLISECONDS: f64 = THOUSAND * 60.0;
static RANDOM_METER_VALUE: i64 = 65536;
static METER_DENOMINATOR: i64 = 4;

struct ReaperTimingPoint {
    time: f64, // Position in seconds
    bpm: f64, // BPM
    meter: i64 // time signature
}

impl ReaperTimingPoint {
    fn new(timing_point: Vec<&str>) -> ReaperTimingPoint {
        let time: f64 = timing_point[1].parse().unwrap();
        let bpm: f64 = timing_point[2].parse().unwrap();
        let meter: i64 = timing_point[4].parse().unwrap();
        
        ReaperTimingPoint {
            time,
            bpm,
            meter
        }
    }

    fn convert_to_osu_time(&self) -> i64 {
        (self.time * THOUSAND) as i64
    }

    fn convert_to_osu_beat_length(&self) -> f64 {
        1.0 / self.bpm * TIME_IN_MILLISECONDS
    }

    fn convert_to_osu_meter(&self) -> i64 {
        self.meter - RANDOM_METER_VALUE * METER_DENOMINATOR
    }
}

struct OsuTimingPoint {
    time: i64, // Position in milliseconds
    beat_length: f64, // beatLength = 1 / BPM * 1000 * 60
    meter: i64, // Amount of beats in a measure
    sample_set: i64, // (0 = default, 1 = normal, 2 = soft, 3 = drum)
    sample_index: i64, // Custom sample index for hit objects. 0 by default
    volume: i64, // Hitobject volume percentage
    uninherited: i64, // Whether or not the timing point is uninherited
    effects: i64, // Bit flags for random shit idk
}

impl OsuTimingPoint {
    fn new(timing_point: ReaperTimingPoint, volume: i64) -> OsuTimingPoint {
        let time = timing_point.convert_to_osu_time();
        let beat_length = timing_point.convert_to_osu_beat_length();
        let meter = timing_point.convert_to_osu_meter();

        OsuTimingPoint {
            time,
            beat_length,
            meter,
            sample_set: 0,
            sample_index: 0,
            volume,
            uninherited: 1,
            effects: 0,
        }
    }
}

fn convert_to_osu(contents: &str, volume: i64) -> Vec<OsuTimingPoint> {
    let mut old_meter: &str = "";

    contents.lines()
        .filter(|line| line.contains("PT "))
        .map(|line| {
            let trimmed_line = line.trim();
            let mut split_line: Vec<&str> = trimmed_line.split(' ').collect();

            if split_line.len() >= 5 {
                old_meter = split_line[4];
            }

            split_line.push(old_meter);
            split_line
        }) // "inherit" previous meter if no change, fuck you REAPER
        .map(ReaperTimingPoint::new) // convert to reaper timing points
        .map(|reaper_point| OsuTimingPoint::new(reaper_point, volume)) // convert to osu timing point format
        .collect()
}

fn print_in_osu_format(osu_timing_points: Vec<OsuTimingPoint>) {
    for OsuTimingPoint { time, beat_length, meter, sample_set, sample_index, volume, uninherited, effects } in osu_timing_points {
        println!("{time},{beat_length},{meter},{sample_set},{sample_index},{volume},{uninherited},{effects}");
    }
}

pub fn run(contents: &str, volume: i64, write_path: Option<String>) {
    if let Some(_write_path) = write_path {
        eprintln!("Writing to file not implemented");
        return
    }

    let osu_points = convert_to_osu(contents, volume);
    print_in_osu_format(osu_points)
}










