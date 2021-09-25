/*
    This file is part of Eruption.

    Eruption is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    Eruption is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with Eruption.  If not, see <http://www.gnu.org/licenses/>.
*/

/// Eruption daemon audio data UNIX domain socket
pub const AUDIO_SOCKET_NAME: &str = "/run/eruption/audio.sock";

/// The capacity of the buffer used for sending audio samples
pub const BUFFER_CAPACITY: usize = 512;

// /// Timeout of D-Bus operations
// pub const DBUS_TIMEOUT_MILLIS: u64 = 5000;

/// Backoff sleep timer, in case an error occurred
pub const SLEEP_TIME_MILLIS: u64 = 2000;