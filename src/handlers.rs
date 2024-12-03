use axum::{extract, response::Html, Json};
use std::fs;
use std::process::{Command, Stdio};
use std::time::Duration;
use tempfile::tempdir;
use wait_timeout::ChildExt;

use crate::models::{CodeInput, CodeOutput};

pub async fn home() -> Html<String> {
    Html(r#"
    <!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>Remmy's Playground</title>
        <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/themes/prism.min.css">
        <script src="https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/prism.min.js"></script>
        <style>
            body {
                font-family: "Helvetica Neue", Helvetica, Arial, sans-serif;
                line-height: 1.5;
                color: #000;
                background-color: #fff;
                margin: 0;
                padding: 0;
            }
            .nav-container {
                background-color: #e0e0e0;
                border-bottom: 1px solid #c0c0c0;
                padding: 10px 15px;
                display: flex;
                justify-content: space-between;
                align-items: center;
                flex-wrap: wrap;
            }
            .nav-container h1 {
                margin: 0;
                font-size: 24px;
                font-weight: 500;
            }
            .nav-container .subtitle {
                font-size: 14px;
                color: #4d4d4d;
            }
            .container {
                max-width: 960px;
                margin: 0 auto;
                padding: 20px;
                display: flex; 
                flex-wrap: wrap;
            }
            .sidebar {
                flex: 0 0 140px; 
                margin-right: 20px;
            }
            .content {
                 flex: 1;
            }
            h2 {
                font-size: 19px;
                font-weight: 400;
                border-bottom: 1px solid #e0e0e0;
                padding-bottom: 5px;
            }
            .menu {
                list-style-type: none;
                padding: 0;
                margin: 0;
            }
            .menu li {
                margin-bottom: 5px;
            }
            .menu a {
                color: #000;
                text-decoration: none;
            }
            .menu a:hover {
                text-decoration: underline;
            }
            pre {
                background-color: #f5f5f5;
                border: 1px solid #e0e0e0;
                padding: 10px;
                overflow-x: auto;
            }
            code {
                font-family: "Source Code Pro", Consolas, "Ubuntu Mono", Menlo, "DejaVu Sans Mono", monospace;
            }
            #code-input {
                width: 100%;
                height: 200px;
                font-family: "Source Code Pro", Consolas, "Ubuntu Mono", Menlo, "DejaVu Sans Mono", monospace;
                margin-bottom: 10px;
            }
            #output {
                white-space: pre-wrap;
                background-color: #f5f5f5;
                border: 1px solid #e0e0e0;
                padding: 10px;
                margin-top: 10px;
                font-family: "Source Code Pro", Consolas, "Ubuntu Mono", Menlo, "DejaVu Sans Mono", monospace;
            }
            button {
                background-color: #4CAF50;
                border: none;
                color: white;
                padding: 10px 20px;
                text-align: center;
                text-decoration: none;
                display: inline-block;
                font-size: 16px;
                margin: 4px 2px;
                cursor: pointer;
            }
            .dark-mode {
                background-color: #121212;
                color: white;
            }
            .dark-mode-link {
                color: white;
            }

            .dark-mode-output {
                background-color: #333;
                color: #fff;
            }
            #dark-mode-toggle {
                background-color: #4CAF50;
                color: white;
                border: none;
                padding: 5px 10px;
                font-size: 14px;
                cursor: pointer;
                border-radius: 4px;
            }

            #dark-mode-toggle:hover {
                background-color: #45a049;
            }        
            body.dark-mode {
                background-color: #121212;
                color: white;
            }

            body.dark-mode .nav-container {
                background-color: #333;
                border-bottom-color: #555;
            }

            body.dark-mode .nav-container .subtitle {
                color: #ccc;
            }

            body.dark-mode .menu a {
                color: #fff; /* Change to white for better visibility */
            }

            body.dark-mode .menu a:hover {
                text-decoration: underline;
                color: #bbb; /* Lighter color on hover */
            }

            body.dark-mode .content {
                background-color: #1e1e1e;
                color: #fff;
            }

            body.dark-mode pre {
                background-color: #2e2e2e;
                border: 1px solid #444;
            }

            body.dark-mode #code-input,
            body.dark-mode #output {
                background-color: #2e2e2e;
                color: #fff; /* Ensure output text is visible */
                border: 1px solid #555;
            }

            body.dark-mode button {
                background-color: #007bff;
                color: white;
            }

            body.dark-mode button:hover {
                background-color: #0056b3;
            }
            #loading {
                display: none;
                font-weight: bold;
                color: blue;
            }
            @media (max-width: 768px) {
                .nav-container {
                    flex-direction: column; /* Stack items vertically */
                    align-items: flex-start; /* Align items to the start */
                }

                .sidebar {
                    width: 100%; /* Full width on mobile */
                    margin-right: 0; /* Remove right margin */
                    margin-bottom: 20px; /* Add bottom margin */
                    flex: 0 0 120px; /* Shrink sidebar further on mobile */
                }

                .content {
                    width: 100%; /* Full width on mobile */
                }

                button {
                    width: 100%; /* Full width buttons */
                }
            }

            @media (max-width: 480px) {
                .nav-container h1 {
                    font-size: 20px; /* Smaller title on mobile */
                }

                .nav-container .subtitle {
                    font-size: 12px; /* Smaller subtitle on mobile */
                }

                h2 {
                    font-size: 18px; /* Smaller headings on mobile */
                }

                button {
                    font-size: 14px; /* Smaller button text */
                }
            }       
        </style>
    </head>
    <body>
        <div class="nav-container">
            <h1>Pape 🦀 <span class="subtitle">Fearless Concurrency & Ownership Rules!</span></h1>
            <button id="dark-mode-toggle" onclick="toggleDarkMode()">
                🌞 Switch to Night Mode 🌙
            </button>
        </div>        
        <div class="container">
            <div class="sidebar">
                <h2>Menu</h2>
                <ul class="menu">
                    <li><a href="/">Home</a></li>
                    <li><a href="/about">About</a></li>
                    <li><a href="/projects">Projects</a></li>
                </ul>
            </div>
            <div class="content">
                <h2>Welcome</h2>
                <p>Hello, My name is Remmy. I used Neovim & Arch BTW 💻 🐧</p>
                
                <h2>Rust Playground</h2>
                <textarea id="code-input" rows="10" style="width: 100%;"></textarea>
                <button onclick="runCode()">Run Code</button>
                <div id="loading">Loading...</div>
                <div id="output"></div>

                <h2>Projects</h2>
                <p>Check out my <a href="/projects">projects page</a> to see some of my work in Rust and web development.</p>
                
                <h2>About Me</h2>
                <p>Learn more about my background and skills on the <a href="/about">about page</a>.</p>
            </div>
        </div>
        <script>
        async function runCode() {
            const code = document.getElementById('code-input').value;
            document.getElementById('loading').style.display = 'block'; // Show loading indicator
            const response = await fetch('/run-code', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({ code }),
            });
            const result = await response.json();
            document.getElementById('output').textContent = result.output;
            document.getElementById('loading').style.display = 'none'; // Hide loading indicator
        }

         function toggleDarkMode() {
            const body = document.body;
            const isDarkMode = body.classList.toggle('dark-mode'); // Toggle dark mode class

            // Save the user's theme preference in localStorage
            if (isDarkMode) {
                localStorage.setItem('theme', 'dark');
                document.getElementById('dark-mode-toggle').innerText = '🌚 Switch to Light Mode 🌞';
            } else {
                localStorage.setItem('theme', 'light');
                document.getElementById('dark-mode-toggle').innerText = '🌞 Switch to Night Mode 🌙';
            }

            // Update the code output's color in dark mode
            document.getElementById('output').classList.toggle('dark-mode-output', isDarkMode);
        }

        // Check localStorage for the user's theme preference
        function loadThemePreference() {
            const savedTheme = localStorage.getItem('theme');

            if (savedTheme === 'dark') {
                document.body.classList.add('dark-mode');
                document.getElementById('dark-mode-toggle').innerText = '🌚 Switch to Light Mode 🌞';
                document.getElementById('output').classList.add('dark-mode-output');
            } else {
                document.getElementById('dark-mode-toggle').innerText = '🌞 Switch to Night Mode 🌙';
            }
        }

        // Load the theme preference when the page loads
        window.onload = function() {
            loadThemePreference();
        }

    // Update Prism.js highlighting on input change
        document.getElementById('code-input').addEventListener('input', function() {
            const code = this.value;
            // Update output for highlighting
            document.getElementById('output').innerHTML = `<pre><code class="language-rust">${Prism.highlight(code, Prism.languages.rust, 'rust')}</code></pre>`;
        });
        </script>
    </body>
    </html>
    "#.to_string())
}

pub async fn about() -> Html<&'static str> {
    Html("<html><body><h1>About Me</h1><p>I'm a passionate Rust developer.</p></body></html>")
}

pub async fn projects() -> Html<&'static str> {
    Html("<html><body><h1>My Projects</h1><p>Here are some of my projects...</p></body></html>")
}

pub async fn run_code(extract::Json(payload): extract::Json<CodeInput>) -> Json<CodeOutput> {
    let dir = tempdir().expect("Failed to create temp dir");
    let file_path = dir.path().join("main.rs");
    fs::write(&file_path, &payload.code).expect("Failed to write file");

    let compile_output = Command::new("rustc")
        .arg(&file_path)
        .arg("-o")
        .arg(dir.path().join("output"))
        .output()
        .expect("Failed to execute rustc command");

    if !compile_output.status.success() {
        let error_message = String::from_utf8_lossy(&compile_output.stderr).into_owned();
        return Json(CodeOutput {
            output: format!("Compilation error: {}", error_message),
        });
    }

    let mut child = Command::new(dir.path().join("output"))
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to execute command");

    let output = match child
        .wait_timeout(Duration::from_secs(5))
        .expect("Failed to wait")
    {
        Some(status) => {
            if status.success() {
                let output = child.wait_with_output().expect("Failed to get output");
                String::from_utf8_lossy(&output.stdout).into_owned()
            } else {
                let output = child.wait_with_output().expect("Failed to get output");
                format!(
                    "Execution failed: {}",
                    String::from_utf8_lossy(&output.stderr)
                )
            }
        }
        None => {
            child.kill().expect("Failed to kill process");
            child.wait().expect("Failed to wait for process");
            "Execution timed out after 5 seconds".to_string()
        }
    };

    Json(CodeOutput { output })
}
