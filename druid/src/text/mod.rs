// Copyright 2018 The Druid Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Text editing utilities.

mod editable_text;
pub use self::editable_text::{EditableText, EditableTextCursor, StringCursor};

pub mod selection;
pub use self::selection::Selection;

pub mod movement;
pub use self::movement::{movement, Movement};

pub mod backspace;
pub use self::backspace::offset_for_delete_backwards;

mod text_input;
pub use self::text_input::{BasicTextInput, EditAction, MouseAction, TextInput};

mod text_buffer;
