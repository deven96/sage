// Copyright 2021 Victor I. Afolabi
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

//! `sage::DType`'s datetime handler.
//!
//! By default `sage::DType::DateTime` uses Utc timezone.
//!

use chrono::prelude::*;
// Confusing `sage::DateTime` & `chrono::DateTime`.
use chrono::DateTime as ChronoDateTime;

#[derive(Clone, Debug, Eq, PartialEq)]

/*
 * +----------------------------------------------------------------------+
 * | +------------------------------------------------------------------+ |
 * | | `DateTime`.
 * | +------------------------------------------------------------------+ |
 * +----------------------------------------------------------------------+
*/

pub struct DateTime {
  d: DateTimeImpl,
}

// Default timezone is Utc.
type DateTimeImpl = ChronoDateTime<Utc>;
