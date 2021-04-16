use gtk::prelude::*;
use gtk::{Label, Entry, Switch, Builder};
use std::cell::RefCell;
use std::path::PathBuf;

use super::helper::HelperFileSettings;
use configparser::ini::Ini;



// Audio Options:
//   --add-codec=name    Manually add codec (default is to enable all)                  unused
//   --dis-codec=name    Disable codec (can be specified multiple times)                unused
//   --clock-rate=N      Override conference bridge clock rate                          hardcoded
//   --snd-clock-rate=N  Override sound device clock rate                               hardcoded
//   --stereo            Audio device and conference bridge opened in stereo mode       hardcoded
//   --null-audio        Use NULL audio device                                          unimplemented
//   --play-file=file    Register WAV file in conference bridge.                        unimplemented
//                       This can be specified multiple times.                          unimplemented
//   --play-tone=FORMAT  Register tone to the conference bridge.
//                       FORMAT is 'F1,F2,ON,OFF', where F1,F2 are
//                       frequencies, and ON,OFF=on/off duration in msec.
//                       This can be specified multiple times.
//   --auto-play         Automatically play the file (to incoming calls only)           unimplemented
//   --auto-play-hangup  Automatically hangup the file after file play completes        unimplemented
//   --auto-loop         Automatically loop incoming RTP to outgoing RTP                unimplemented
//   --auto-conf         Automatically put calls in conference with others              unimplemented
//   --rec-file=file     Open file recorder (extension can be .wav or .mp3              unimplemented
//   --auto-rec          Automatically record conversation                              unimplemented
//   --quality=N         Specify media quality (0-10, default=8)                        optional
//   --ptime=MSEC        Override codec ptime to MSEC (default=specific)                optional
//   --no-vad            Disable VAD/silence detector (default=vad enabled)             optional
//   --ec-tail=MSEC      Set echo canceller tail length (default=200)
//   --ec-opt=OPT        Select echo canceller algorithm (0=default,
//                         1=speex, 2=suppressor, 3=WebRtc)
//   --ilbc-mode=MODE    Set iLBC codec mode (20 or 30, default is 30)
//   --capture-dev=id    Audio capture device ID (default=-1)
//   --playback-dev=id   Audio playback device ID (default=-1)
//   --capture-lat=N     Audio capture latency, in ms (default=100)
//   --playback-lat=N    Audio playback latency, in ms (default=140)
//   --snd-auto-close=N  Auto close audio device when idle for N secs (default=1)
//                       Specify N=-1 to disable this feature.
//                       Specify N=0 for instant close when unused.
//   --no-tones          Disable audible tones
//   --jb-max-size       Specify jitter buffer maximum size, in frames (default=-1)
//   --extra-audio       Add one more audio stream

#[derive(Clone)]
pub struct SettingsAudioStorage {

}

impl SettingsAudioStorage {
    pub fn new(gtk_builder: &Builder) -> Self {
        SettingsAudioStorage {

        }
    }
}

#[derive(Clone)]
pub struct SettingsAudioWidget {
    ctx: RefCell<SettingsAudioStorage>
}

impl SettingsAudioWidget {
    pub fn new(gtk_builder: &Builder) -> Self {
        let result = SettingsAudioWidget{
            ctx: RefCell::new(SettingsAudioStorage::new(gtk_builder)),
        };

        result
    }

    pub fn reset(&self) {
        todo!();
    }
}


impl HelperFileSettings for SettingsAudioWidget {
    fn load(&self, path: PathBuf) {
        todo!()
    }

    fn save(&self, path: PathBuf) {
        todo!()
    }
}

