use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyTuple};
use std::fs;

use crate::utils::{log, tmp_sample_file};

pub struct TrackData {
    pub track: String,
    pub artist: String,
    pub coverart: String,
}

pub fn empty_track_data() -> TrackData {
    TrackData {
        track: "".to_string(),
        artist: "".to_string(),
        coverart: "".to_string(),
    }
}

pub fn recognize_track() -> TrackData {
    // Define essentials
    pyo3::prepare_freethreaded_python();
    let mut track_data = empty_track_data();

    // Execute and extract python script
    let exec_py: Result<TrackData, PyErr> = Python::with_gil(|py| {
        // Define essentials
        PyModule::import(py, "ShazamAPI")?;
        let bytes = fs::read(tmp_sample_file())?;
        let args = (PyBytes::new(py, &bytes),);
        let mut track_data = empty_track_data();

        // Execute script
        let script: Result<&PyTuple, PyErr> =
            PyModule::from_code(py, SHAZAM_CODE, "Shazam.py", "Shazam")?
                .getattr("recognize")?
                .call1(args)?
                .extract();
        match script {
            Ok(sh) => {
                if sh.len() == 3 {
                    track_data.track = sh.get_item(0)?.to_string();
                    track_data.artist = sh.get_item(1)?.to_string();
                    track_data.coverart = sh.get_item(2)?.to_string();
                }
            }
            Err(e) => log(
                "call_py",
                "exec_py",
                &format!("Error occured when recognizing sound: {}", e),
            ),
        };

        // Return track_data if ok
        Ok(track_data)
    });

    // Returning track_data or "null"
    match exec_py {
        Ok(sh) => {
            track_data = TrackData {
                track: sh.track,
                artist: sh.artist,
                coverart: sh.coverart,
            }
        }
        Err(e) => log(
            "call_py",
            "recognize_track",
            &format!("Error occured when recognizing sound: {}", e),
        ),
    };
    return track_data;
}

static SHAZAM_CODE: &str = "
###
# scripts/shazam_recognize.py
###
# MIT License
#
# Copyright (c) 2021 Numenorean
#
# Permission is hereby granted, free of charge, to any person obtaining a copy
# of this software and associated documentation files (the \"Software\"), to deal
# in the Software without restriction, including without limitation the rights
# to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
# copies of the Software, and to permit persons to whom the Software is
# furnished to do so, subject to the following conditions:
#
# The above copyright notice and this permission notice shall be included in all
# copies or substantial portions of the Software.
#
# THE SOFTWARE IS PROVIDED \"AS IS\", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
# IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
# FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
# AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
# LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
# OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
# SOFTWARE.
###
# Bör man gö såhär ??
###
# Kalla mig etisk
###

import ShazamAPI


def recognize(data: bytes):
    shazam = ShazamAPI.Shazam(
        data,
    )
    a = {'b': 'lol'}
    recognize_generator = list(shazam.recognizeSong())
    song = \"\"
    artist = \"\"
    coverart = \"\"

    song = recognize_generator[0][1]['track']['title']
    artist = recognize_generator[0][1]['track']['subtitle']
    coverart = recognize_generator[0][1]['track']['images']['coverart']

    return (song, artist, coverart)
";
