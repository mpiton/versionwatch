<!-- PROJECT SHIELDS -->
[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![MIT License][license-shield]][license-url]

<!-- PROJECT LOGO -->
<br />
<div align="center">
  <a href="https://github.com/mpiton/versionwatch">
    <img src="images/logo.png" alt="Logo" width="80" height="80">
  </a>

<h3 align="center">VersionWatch</h3>

  <p align="center">
    A full-stack application to track and visualize software versions.
    <br />
    <a href="https://github.com/mpiton/versionwatch"><strong>Explore the docs »</strong></a>
    <br />
    <br />
    <a href="https://github.com/mpiton/versionwatch/issues">Report Bug</a>
  </p>
</div>

<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
      <ul>
        <li><a href="#built-with">Built With</a></li>
      </ul>
    </li>
    <li>
      <a href="#getting-started">Getting Started</a>
      <ul>
        <li><a href="#prerequisites">Prerequisites</a></li>
        <li><a href="#installation">Installation</a></li>
      </ul>
    </li>
    <li><a href="#usage">Usage</a></li>
    <li><a href="#contributing">Contributing</a></li>
    <li><a href="#license">License</a></li>
  </ol>
</details>

<!-- ABOUT THE PROJECT -->
## About The Project

VersionWatch is a full-stack application designed to monitor software versions and their lifecycles. It combines a powerful Rust backend for data collection with a modern React frontend for visualization.

[![VersionWatch Dashboard Screenshot][product-screenshot]](images/demo.png)

For a detailed explanation of the system's design, see the [Architecture Document](ARCHITECTURE.md).

Key features include:
*   **Automated Data Collection**: The Rust backend features a modular system of "collectors" that fetch version data from official sources (APIs, Git repositories, etc.).
*   **High-Performance ETL**: The backend uses Polars DataFrames for efficient data transformation and processing.
*   **Interactive Dashboard**: A React and TypeScript frontend provides charts and tables to visualize software versions, release dates, and more.
*   **REST API**: The backend, built with Axum, exposes a REST API to serve the collected data to the frontend.

### Built With

*   [![Rust][Rust-shield]][Rust-url]
*   [![React][React-shield]][React-url]
*   [![PostgreSQL][PostgreSQL-shield]][PostgreSQL-url]
*   [![Docker][Docker-shield]][Docker-url]

<!-- GETTING STARTED -->
## Getting Started

Follow these steps to get a local copy up and running.

### Prerequisites

*   **Rust**: Make sure you have the Rust toolchain installed. You can get it from [rust-lang.org](https://www.rust-lang.org/tools/install).
*   **Node.js**: Node.js is required for the React frontend. Get it from [nodejs.org](https://nodejs.org/).
*   **Docker**: Docker is required to run the PostgreSQL database. Get it from [docker.com](https://www.docker.com/products/docker-desktop).

### Installation

1.  **Clone the repository**:
    ```sh
    git clone https://github.com/mpiton/versionwatch.git
    cd versionwatch
    ```
2.  **Install frontend dependencies**:
    ```sh
    cd frontend
    npm install
    cd ..
    ```
3.  **Set up your environment**:
    Create a `.env` file from the example. If you need to fetch data from sources that require authentication (like the GitHub API), add your tokens here.
    ```sh
    cp .env.example .env
    # Example: echo "GITHUB_PAT=your_github_personal_access_token" >> .env
    ```

## Usage

The easiest way to run the entire application (frontend and backend) is with the provided development script.

```sh
./start-dev.sh
```

This will:
1.  Launch the PostgreSQL database in Docker.
2.  Apply any pending database migrations.
3.  Start the backend server.
4.  Build and serve the frontend application.

Once running, you can access the dashboard at **http://127.0.0.1:3000**.

## Scripts and Deployment

The project includes scripts to help manage the application lifecycle.

### Production Build

To create an optimized production build, run the following script:
```sh
./build-production.sh
```
This script will:
1.  Build the React frontend for production.
2.  Build the Rust backend in release mode.
3.  Package the compiled frontend, backend binary, and configuration into a `dist/` folder, ready for deployment.

### Stopping the Server

To manually stop the server process, you can use the `stop-server.sh` script:
```sh
./stop-server.sh
```

## Development

### Running Frontend and Backend Separately

You can also run the frontend and backend services independently.

**Start the Backend Server:**
```sh
# This also starts the database and applies migrations
./start-dev.sh
```
Then, stop the server with `Ctrl+C` but leave the database running. Now you can run the backend in watch mode:
```sh
cargo watch -x "run --bin versionwatch-cli -- serve"
```

**Start the Frontend Dev Server:**
```sh
cd frontend
npm run dev
```
The frontend will now be available with hot-reloading.

### Frontend Architecture

The frontend is built with a component-based architecture. For a detailed breakdown of the components, data flow, and styling, please see the [Frontend Architecture Document](frontend/COMPONENTS.md).

## Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

➡️ **To add support for a new software (collector), see:** [How to Add a Software Collector](docs/add_software_collector.md)

1.  Fork the Project
2.  Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3.  Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4.  Push to the Branch (`git push origin feature/AmazingFeature`)
5.  Open a Pull Request

<!-- LICENSE -->
## License

Distributed under the MIT License. See `LICENSE.txt` for more information.

<!-- MARKDOWN LINKS & IMAGES -->
[product-screenshot]: images/demo.png
[contributors-shield]: https://img.shields.io/github/contributors/mpiton/versionwatch.svg?style=for-the-badge
[contributors-url]: https://github.com/mpiton/versionwatch/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/mpiton/versionwatch.svg?style=for-the-badge
[forks-url]: https://github.com/mpiton/versionwatch/network/members
[stars-shield]: https://img.shields.io/github/stars/mpiton/versionwatch.svg?style=for-the-badge
[stars-url]: https://github.com/mpiton/versionwatch/stargazers
[issues-shield]: https://img.shields.io/github/issues/mpiton/versionwatch.svg?style=for-the-badge
[issues-url]: https://github.com/mpiton/versionwatch/issues
[license-shield]: https://img.shields.io/github/license/mpiton/versionwatch.svg?style=for-the-badge
[license-url]: https://github.com/mpiton/versionwatch/blob/master/LICENSE.txt
[Rust-shield]: https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white
[Rust-url]: https://www.rust-lang.org/
[React-shield]: https://img.shields.io/badge/React-20232A?style=for-the-badge&logo=react&logoColor=61DAFB
[React-url]: https://reactjs.org/
[PostgreSQL-shield]: https://img.shields.io/badge/PostgreSQL-316192?style=for-the-badge&logo=postgresql&logoColor=white
[PostgreSQL-url]: https://www.postgresql.org/
[Docker-shield]: https://img.shields.io/badge/Docker-2496ED?style=for-the-badge&logo=docker&logoColor=white
[Docker-url]: https://www.docker.com/ 