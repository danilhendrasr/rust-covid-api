<p align="center" style="font-size: 5px">
  🚧 This repo currently undergoes a major refactoring effort, don't expect things to function normally 🚧 
</p>

---

<p align="center">
  <h1 align="center">Rust COVID-19 API</h1>

  <p align="center">
    A simple containerized COVID-19 API built using Rust & Actix Web
  </p>
</p>

<div align="center">
  
[![MIT License][license-shield]][license-url]
[![LinkedIn][linkedin-shield]][linkedin-url]
  
</div>

<!-- ABOUT THE PROJECT -->
## About The Project
In order to be able to participate in Nodeflux's internship program as a software engineer, 
applicants are required to complete a technical assessment where they are tasked to build a containerized COVID-19 API using either C++, Rust, or Python.
This project is made to complete the said technical assessment. 

### Why Rust?
I've tinker around with C++, Rust, and Python in the past but never really got too deep, so I still consider myself a beginner in those three languages. 
I also have a lingering interest in system programming since a while back and Rust has caught my interest and curiosity too. 
So based on these, I figured I might as well just pick Rust to build this project with. 
Besides, I can treat it as my Rust learning playground.

<!-- GETTING STARTED -->
## Getting Started
### Using Docker
The docker image is available to pull from in the following url: [https://hub.docker.com/r/danilhendrasr/rust-covid-api](https://hub.docker.com/r/danilhendrasr/rust-covid-api).
#### Steps
```bash
# Pull the image from docker hub
docker pull danilhendrasr/rust-covid-api

# Run the container, will be accessible through localhost:8081
docker run -d --name rust-covid-api -p 8081:8081 danilhendrasr/rust-covid-api
```

### Build from source
#### Prerequisites
- Rust 1.57 (used to write this program)
- pkg-config (for linux system)
- libssl-dev (for linux system)

#### Steps
```bash
# clone the repo
git clone https://github.com/danilhendrasr/nodeflux-technical-assessment rust-covid-api

# change the current directory
cd rust-covid-api

# build program and dependencies
cargo build --release

# run the program, the app will be accessible through localhost:8081
cargo run --release
```

## API Contract
The API contract mostly stays the same as written in the technical assessment document, with only several minor additions such as:
- The API will respond with `400 (Bad request)` response with `Invalid query parameter(s)` text body if invalid query parameter(s) are supplied to the following routes:
  - `/yearly`
  - `/monthly`
  - `/monthly/<year>`
  - `/daily`
  - `/daily/<year>`
  - `/daily/<year>/<month>`
- The API will respond with `500 (Internal server error)` response with a short description in the body if it fails to fetch or process the data.

<!-- LICENSE -->
## License

Distributed under the MIT License. See [LICENSE](https://github.com/danilhendrasr/nodeflux-technical-assessment/blob/main/LICENSE) for more information.


<!-- ACKNOWLEDGEMENTS -->
## Acknowledgements
* [Nodeflux](https://nodeflux.io)
* [Rust](https://github.com/rust-lang/rust)
* [Actix Web](https://github.com/rust-lang/rust)



<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->
[contributors-shield]: https://img.shields.io/github/contributors/othneildrew/Best-README-Template.svg?style=for-the-badge
[contributors-url]: https://github.com/othneildrew/Best-README-Template/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/othneildrew/Best-README-Template.svg?style=for-the-badge
[forks-url]: https://github.com/othneildrew/Best-README-Template/network/members
[stars-shield]: https://img.shields.io/github/stars/danilhendrasr/yali4j.svg?style=for-the-badge
[stars-url]: https://github.com/danilhendrasr/yali4j/stargazers
[issues-shield]: https://img.shields.io/github/issues/danilhendrasr/yali4j.svg?style=for-the-badge
[issues-url]: https://github.com/danilhendrasr/yali4j/issues
[license-shield]: https://img.shields.io/github/license/danilhendrasr/yali4j.svg?style=for-the-badge
[license-url]: https://github.com/danilhendrasr/yali4j/blob/main/LICENSE
[linkedin-shield]: https://img.shields.io/badge/-LinkedIn-black.svg?style=for-the-badge&logo=linkedin&colorB=555
[linkedin-url]: https://linkedin.com/in/danilhendrasr
[product-screenshot]: images/screenshot.png
