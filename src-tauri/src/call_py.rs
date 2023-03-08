use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyTuple};
use std::fs;

pub struct Shazam {
    pub track: String,
    pub artist: String,
    pub coverart: String,
}

impl Shazam {
    pub fn empty() -> Shazam {
        Shazam {
            track: String::from("n/a"),
            artist: String::from("n/a"),
            coverart: String::from("n/a"),
        }
    }
}

pub fn shazam() -> Shazam {
    pyo3::prepare_freethreaded_python();
    let mut def_rec = Shazam::empty();

    let exec_py: Result<Shazam, PyErr> = Python::with_gil(|py| {
        PyModule::import(py, "ShazamAPI")?;
        let bytes = fs::read("tmp/sine.wav").expect("Couldnt load /tmp/sine.wav");
        let args = (PyBytes::new(py, &bytes),);
        let fun: Result<&PyTuple, PyErr> =
            PyModule::from_code(py, SHAZAM_CODE, "Shazam.py", "Shazam")?
                .getattr("recognize")?
                .call1(args)?
                .extract();

        let mut def_rec = Shazam::empty();

        match fun {
            Ok(sh) => {
                if sh.len() == 3 {
                    def_rec.track = sh.get_item(0)?.to_string();
                    def_rec.artist = sh.get_item(1)?.to_string();
                    def_rec.coverart = sh.get_item(2)?.to_string();
                }
            }
            Err(b) => println!("Error occured when recognizing sound: {}", b),
        };

        Ok(def_rec)
    });

    match exec_py {
        Ok(sh) => def_rec = sh,
        Err(e) => println!("Error occured when recognizing sound: {}", e),
    };
    return def_rec;
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
    song = \"n/a\"
    artist = \"n/a\"
    coverart = \"n/a\"

    song = recognize_generator[0][1]['track']['title']
    artist = recognize_generator[0][1]['track']['subtitle']
    coverart = recognize_generator[0][1]['track']['images']['coverart']

    return (song, artist, coverart)
";
