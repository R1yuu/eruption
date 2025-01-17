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

    Copyright (c) 2019-2022, The Eruption Development Team
*/

use std::path::Path;

use crate::mapping::KeyMappingTable;

pub mod lua;
pub mod native;

pub type Result<T> = std::result::Result<T, eyre::Error>;

pub trait Backend {
    fn generate(&self, table: &KeyMappingTable) -> Result<String>;
    fn write_to_file<P: AsRef<Path>>(&self, path: P, table: &KeyMappingTable) -> Result<()>;
}
