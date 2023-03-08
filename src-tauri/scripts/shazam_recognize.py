###
# MIT License
#
# Copyright (c) 2021 Numenorean
#
# Permission is hereby granted, free of charge, to any person obtaining a copy
# of this software and associated documentation files (the "Software"), to deal
# in the Software without restriction, including without limitation the rights
# to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
# copies of the Software, and to permit persons to whom the Software is
# furnished to do so, subject to the following conditions:
#
# The above copyright notice and this permission notice shall be included in all
# copies or substantial portions of the Software.
#
# THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
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


from ShazamAPI import Shazam
import json


def recognize(data: bytes):
    shazam = Shazam(
        data,
    )
    a = {'b': 'lol'}
    recognize_generator = list(shazam.recognizeSong())
    song = "n/a"
    artist = "n/a"
    coverart = "n/a"

    print(json.dump(list(recognize_generator),
          open("./tmp/test.json", "w"), indent=2))
    song = recognize_generator[0][1]['track']['title']
    artist = recognize_generator[0][1]['track']['subtitle']
    coverart = recognize_generator[0][1]['track']['images']['coverart']

    print(song, artist, coverart)


if __name__ == '__main__':
    recognize(open(
        'C:/Users/Sebastian/Documents/Projects/line-in/src-tauri/tmp/sine.wav', 'rb').read())
