# My Rust Project

A simple Rust project demonstrating core Rust functionalities and job search logic.
IM GRATEFULL , consider buying me a coffee! Your support is greatly appreciated.

[![Buy Me a Coffee](https://img.shields.io/badge/Donate-Buy%20Me%20a%20Coffee-yellow)](https://paypal.me/barki0)

## Features

- **Core Job Search Logic**: Implemented in Rust with functions for keyword search, location filtering, and company filtering.
- **Integration**: Easy integration with other languages like Python or JavaScript through FFI or WebAssembly.

## Getting Started

### Prerequisites

- **Rust and Cargo**: Ensure that Rust and Cargo are installed.
  ```bash
  rustc --version
  cargo --version
Installation
Clone the Repository:

bash
git clone https://github.com/akaday/my-rust-project.git
cd my-rust-project
Build the Project:

bash
cargo build
Run the Project:

bash
cargo run
Usage
Running the Example
The main Rust project demonstrates job search functionalities:

rust
extern crate job_search;

fn main() {
    let jobs = vec![
        "Software Engineer at Google",
        "Data Scientist at Facebook",
        "Backend Developer at Amazon",
    ];

    let keyword = "Engineer";
    let filtered_jobs = job_search::search_jobs(keyword, jobs.iter().map(AsRef::as_ref).collect());

    println!("Filtered Jobs for '{}': {:?}", keyword, filtered_jobs);
}
Using the Library
You can use the job_search library in other projects by adding it to your Cargo.toml:

toml
[dependencies]
job_search = { path = "path/to/job_search" }

### Examples of Using New Functions

#### Filtering Jobs by Salary Range

```rust
extern crate job_search;

fn main() {
    let jobs = vec![
        ("Software Engineer at Google", 120000, "Full-time", "Mid-level"),
        ("Data Scientist at Facebook", 130000, "Full-time", "Senior"),
        ("Backend Developer at Amazon", 110000, "Part-time", "Junior"),
    ];

    let min_salary = 115000;
    let max_salary = 125000;
    let filtered_by_salary = job_search::filter_jobs_by_salary_range(min_salary, max_salary, jobs.iter().map(|(title, salary, _, _)| (*title, *salary)).collect());

    println!("Filtered Jobs by Salary Range ({} - {}): {:?}", min_salary, max_salary, filtered_by_salary);
}
```

#### Filtering Jobs by Job Type

```rust
extern crate job_search;

fn main() {
    let jobs = vec![
        ("Software Engineer at Google", 120000, "Full-time", "Mid-level"),
        ("Data Scientist at Facebook", 130000, "Full-time", "Senior"),
        ("Backend Developer at Amazon", 110000, "Part-time", "Junior"),
    ];

    let job_type = "Full-time";
    let filtered_by_job_type = job_search::filter_jobs_by_job_type(job_type, jobs.iter().map(|(title, _, jt, _)| (*title, *jt)).collect());

    println!("Filtered Jobs by Job Type ({}): {:?}", job_type, filtered_by_job_type);
}
```

#### Filtering Jobs by Experience Level

```rust
extern crate job_search;

fn main() {
    let jobs = vec![
        ("Software Engineer at Google", 120000, "Full-time", "Mid-level"),
        ("Data Scientist at Facebook", 130000, "Full-time", "Senior"),
        ("Backend Developer at Amazon", 110000, "Part-time", "Junior"),
    ];

    let experience_level = "Senior";
    let filtered_by_experience = job_search::filter_jobs_by_experience_level(experience_level, jobs.iter().map(|(title, _, _, exp)| (*title, *exp)).collect());

    println!("Filtered Jobs by Experience Level ({}): {:?}", experience_level, filtered_by_experience);
}
```

### Running the Backend Server

To run the backend server, follow these steps:

1. Ensure that you have Rust and Cargo installed.
2. Clone the repository and navigate to the project directory.
3. Build the project using `cargo build`.
4. Run the backend server using `cargo run`.

The backend server will start and listen for incoming requests on `127.0.0.1:8080`.

### API Endpoints

The backend server provides the following API endpoints for job search and filtering:

- **Search Jobs**: `POST /search_jobs/{keyword}`
  - Request Body: JSON array of job titles (e.g., `["Software Engineer", "Data Scientist", "Backend Developer"]`)
  - Response: JSON array of filtered job titles

- **Filter Jobs by Salary Range**: `POST /filter_jobs_by_salary/{min_salary}/{max_salary}`
  - Request Body: JSON array of job titles and salaries (e.g., `[["Software Engineer", 120000], ["Data Scientist", 130000], ["Backend Developer", 110000]]`)
  - Response: JSON array of filtered job titles and salaries

- **Filter Jobs by Job Type**: `POST /filter_jobs_by_job_type/{job_type}`
  - Request Body: JSON array of job titles and job types (e.g., `[["Software Engineer", "Full-time"], ["Data Scientist", "Full-time"], ["Backend Developer", "Part-time"]]`)
  - Response: JSON array of filtered job titles and job types

- **Filter Jobs by Experience Level**: `POST /filter_jobs_by_experience/{experience_level}`
  - Request Body: JSON array of job titles and experience levels (e.g., `[["Software Engineer", "Mid-level"], ["Data Scientist", "Senior"], ["Backend Developer", "Junior"]]`)
  - Response: JSON array of filtered job titles and experience levels

Contributing
Contributions are welcome! Please submit a pull request or open an issue to get started.

License
This project is licensed under the MIT License.

Support
If you find this project helpful, consider buying me a coffee! Your support is greatly appreciated.


Happy coding! 😊🚀✨
