/// WorldOptions are the options set to be used during generation.
#[derive(Debug, Clone)]
pub struct WorldOptions {
    pub seed: String,
    pub name: String,

    pub temp_avg: f64,
    pub temo_dev: f64,
    pub rsrc_rich: f64,
    pub rssc_abund: f64,
    pub flatness: f64,
    pub vegetation: f64,
    pub water: f64,

    pub proclensity: f64,
    pub hostile_density: f64,
    pub hostile_attack_rate: f64,
    pub hostile_intelligence: f64,
    pub hostile_evolution: f64,
    pub hostile_drop_rate: f64,

    pub dark_density: f64,
    pub dark_spread: f64,

    pub pollution_spread: f64,
    pub heat_spread: f64,

    /*
    pub civ_density: f64,
    pub civ_count: f64,
    pub civ_peace_factor: f64,
     */
}

impl Default for WorldOptions {
    fn default() -> Self {
        WorldOptions{
            seed: "mechanomancer".to_string(),
            name: "Camp Mechano".to_string(),
            temp_avg: 18.0,
            temo_dev: 2.0,
            rsrc_rich: 100.0,
            rssc_abund: 100.0,
            flatness: 50.0,
            vegetation: 100.0,
            water: 100.0,
            proclensity: 0.0,
            hostile_density: 100.0,
            hostile_attack_rate: 1.0,
            hostile_intelligence: 1.0,
            hostile_evolution: 0.05,
            hostile_drop_rate: 1.0,
            dark_density: 1.0,
            dark_spread: 0.3,
            pollution_spread: 0.3,
            heat_spread: 0.3,
        }
    }
}

#[derive(Debug, Default)]
pub struct StoryOptions {
    randomness: f64,
    severity: f64,
}

struct Generator {

}

impl Generator {
    pub fn run(o: &WorldOptions, s: &StoryOptions) {
    }
}