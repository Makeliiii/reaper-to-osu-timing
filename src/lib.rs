static THOUSAND: f64 = 1000.0;
static TIME_IN_MILLISECONDS: f64 = THOUSAND * 60.0;

struct ReaperTimingPoint {
    time: f64, // Position in seconds
    bpm: f64 // BPM
}

impl ReaperTimingPoint {
    fn new(timing_point: &str) -> ReaperTimingPoint {
        let data: Vec<&str> = timing_point.split(' ').collect();
        let time: f64 = data[1].parse().unwrap();
        let bpm: f64 = data[2].parse().unwrap();

        ReaperTimingPoint {
            time,
            bpm
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
    meter: i32, // Amount of beats in a measure
    sample_set: i32, // (0 = default, 1 = normal, 2 = soft, 3 = drum)
    sample_index: i32, // Custom sample index for hit objects. 0 by default
    volume: i32, // Hitobject volume percentage
    uninherited: i32, // Whether or not the timing point is uninherited
    effects: i32, // Bit flags for random shit idk
}

impl OsuTimingPoint {
    fn new(timing_point: ReaperTimingPoint) -> OsuTimingPoint {
        let time = timing_point.convert_to_osu_time();
        let beat_length = timing_point.convert_to_osu_beat_length();

        OsuTimingPoint {
            time,
            beat_length,
            meter: 4,
            sample_set: 0,
            sample_index: 0,
            volume: 100,
            uninherited: 1,
            effects: 0,
        }
    }
}

fn filter_reaper_timing_points(contents: &str) -> Vec<&str> {
    contents.lines()
        .filter(|line| line.contains("PT "))
        .map(|line| line.trim()).collect()
}

fn read_reaper_timing_points(timing_points: Vec<&str>) -> Vec<ReaperTimingPoint> {
    timing_points.into_iter().map(ReaperTimingPoint::new).collect()
}

fn convert_reaper_to_osu(reaper_timing_points: Vec<ReaperTimingPoint>) -> Vec<OsuTimingPoint> {
    reaper_timing_points.into_iter().map(OsuTimingPoint::new).collect()
}

fn print_in_osu_format(osu_timing_points: Vec<OsuTimingPoint>) {
    for OsuTimingPoint { time, beat_length, meter, sample_set, sample_index, volume, uninherited, effects } in osu_timing_points {
        println!("{time},{beat_length},{meter},{sample_set},{sample_index},{volume},{uninherited},{effects}");
    }
}

pub fn run(contents: &str) {
    let filtered_points = filter_reaper_timing_points(contents);
    let reaper_points = read_reaper_timing_points(filtered_points);
    let osu_points = convert_reaper_to_osu(reaper_points);
    print_in_osu_format(osu_points);
}
