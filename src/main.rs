// Copyright (c) 2016 The Rouille developers
// Copyright (c) 2025 Erlend Tobiassen
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>,
// at your option. All files in the project carrying such
// notice may not be copied, modified, or distributed except
// according to those terms.
use rouille::cgi::CgiRun;
use std::env;
use std::io;
use std::process::Command;

fn main() {
    let mut arguments = std::env::args();

    let _ = arguments.next();

    let bind = arguments.next().unwrap_or("localhost".to_string());
    let port = arguments.next().unwrap_or("5555".to_string());

    let git_root = &*env::current_dir()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
        .leak();

    println!("Starting git server on http://{}:{}", bind, port);

    rouille::start_server(format!("{}:{}", bind, port), move |request| {
        rouille::log(request, io::stdout(), move || {
            // When a request is received, we invoke the `git http-backend` command through CGI.
            let mut cmd = Command::new("git");
            cmd.arg("http-backend");

            // We need to set some git-specific environment variables.
            cmd.env("GIT_PROJECT_ROOT", git_root);

            // This one is required to avoid security errors.
            cmd.env("GIT_HTTP_EXPORT_ALL", "");

            cmd.start_cgi(request).unwrap()
        })
    });
}
