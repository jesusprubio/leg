/**
 * Copyright (c) 2019, Jesús Rubio <jesusprubio@gmail.com>
 *
 * This source code is licensed under the MIT License found in
 * the LICENSE.txt file in the root directory of this source tree.
 */
use leg::info;

#[async_std::main]
async fn main() {
    info("Informational message", None, None).await;
}
