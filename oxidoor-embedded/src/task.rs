// Copyright 2025 AprilNEA LLC
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
//
// SPDX-License-Identifier: Apache-2.0

use embassy_time::{Duration, Timer};
use esp_idf_svc::hal::gpio::{Gpio5, Gpio6, Gpio7, Input, Output, PinDriver};

#[embassy_executor::task]
pub async fn door_control(
    mut door_lock: PinDriver<'static, Gpio5, Output>,
    mut button1: PinDriver<'static, Gpio6, Input>,
    mut button2: PinDriver<'static, Gpio7, Input>,
) {
    loop {
        // Asynchronous wait for any button pressed (low level)
        embassy_futures::select::select(button1.wait_for_low(), button2.wait_for_low()).await;

        let _ = door_lock.set_low(); // Unlock
        
        // Asynchronous delay 3 seconds
        Timer::after(Duration::from_secs(3)).await;

        let _ = door_lock.set_high(); // Re-lock

        // Wait for button release
        loop {
            if button1.is_low() && button2.is_low() {
                break;
            }
            Timer::after_millis(10).await;
        }
    }
}
