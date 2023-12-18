use clap::builder::Str;
use rust_abf::abf::Abf;
use ndarray::Array;
use serde::Serialize;
use csv::Writer;

use crate::channel_selector::ChannelSelector;

#[derive(Serialize)]
pub struct AnalysisResult {
    name: String,
    means: f32,
    stds: f32,
    uoms: String,
}

impl AnalysisResult {
    fn new(name: String, means: Vec<f32>, stds: Vec<f32>, uoms: Vec<&str>) -> Self {
        Self { name, means: means[0], stds:stds[0], uoms: uoms.iter().map(|s|s.to_string()).collect() }
    } 

    pub fn serialize(&self) -> String {
        let mut wtr = Writer::from_writer(vec![]);
        wtr.serialize(self).unwrap();
        String::from_utf8(wtr.into_inner().unwrap()).unwrap()
    }
}

// todo will disappear when rust_abf will implement a get_name
pub struct MyAbf {
    abf: Abf,
    name: String,
}

impl MyAbf {
    pub fn new(abf: Abf, name: &str) -> Self {
        let name = name.to_string();
        Self { abf, name }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_mean(&self, cs: &ChannelSelector) -> Option<Vec<f32>> {
        match cs {
            // ChannelSelector::All => ,
            ChannelSelector::Channel(ch_idx) => {
                let ch_idx = * ch_idx;
                let ch = self.abf.get_channel(ch_idx as u32).unwrap();
                let data = Array::from_iter(ch.get_sweep(0).unwrap());
                match data.mean() {
                    Some(m) => Some(vec![m]),
                    _ => None 
                }
            },
            _ => None
        }
    }

    fn get_std_dev(&self, cs: &ChannelSelector) -> Option<Vec<f32>> {
        match cs {
            // ChannelSelector::All => ,
            ChannelSelector::Channel(ch_idx) => {
                let ch_idx = * ch_idx;
                let ch = self.abf.get_channel(ch_idx as u32).unwrap();
                let data = Array::from_iter(ch.get_sweep(0).unwrap());
                Some(vec![data.std(0.)])
            },
            _ => None
        }
    }

    fn get_uom(&self, cs: &ChannelSelector) -> Option<Vec<&str>> {
        match cs {
            // ChannelSelector::All => ,
            ChannelSelector::Channel(ch_idx) => {
                let ch_idx = * ch_idx;
                let ch = self.abf.get_channel(ch_idx as u32).unwrap();
                Some(vec![ch.get_uom()])
            },
            _ => None
        }
    }

    pub fn get_analysis_result(&self, cs: &ChannelSelector) -> Option<AnalysisResult> {
        let name = self.get_name();
        let Some(uoms) = self.get_uom(cs) else {
            return None;
        };
        let Some(stds) = self.get_std_dev(cs) else {
            return None;
        };
        let Some(means) = self.get_mean(cs) else {
            return None;
        };
        Some(AnalysisResult::new(name, means, stds, uoms))
    }
}
