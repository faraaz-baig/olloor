# Olloor

<br />
<p align="center">
  <h3 align="center">Olloor</h3>

  <p align="center">
    "a tool that lets you write smart bookmarks in [rust] and then share them across all your browsers..."
    <br />
    <a href="https://github.com/fbsamples/rusty-bunny#demo">View Demo</a>
  </p>
</p>

<!-- ABOUT THE PROJECT -->

## About the Project

Olloor is a tool that lets you write smart bookmarks in rust and then share them across all your browsers and with a group of people or the whole world.

## Demo

![rusty-bunny demo][product-screenshot]

This is what `rusty-bunny` looks like in action.

### Built With

- [Rust](https://www.rust-lang.org/)
- [Rocket](https://rocket.rs/)

### Manual Setup

#### Prerequisites

Make sure you have Rust installed.

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Rocket uses the nightly version of Rust so make sure you use that. If you'd like to only use nightly for this project, you can run this from the root of the project after cloning.

```sh
# from the root of the project
rustup override set nightly
```

#### Installation

1. Clone the rusty-bunny

```sh
git clone https://github.com/faraaz-baig/olloor.git
```

2. Make sure you're using nightly

```sh
cargo --version
```

3. Build the project

```sh
cargo build
```

4. Follow the instructions in the [Running](#running) section.

### VSCode Dev Container Setup

#### Prerequisites

This requires VSCode, Docker and the Remote Development extension pack. For more details see [the official docs](https://code.visualstudio.com/docs/remote/containers#_system-requirements).

#### Spinning Up The Environment

- Follow [the official guide](https://code.visualstudio.com/docs/remote/containers#_quick-start-open-a-git-repository-or-github-pr-in-an-isolated-container-volume) to open this repository inside a dev container. VSCode will read the [config file](.devcontainer/devcontainer.json) provided to auto-install relevant dependencies and extensions.
- To run terminal commands, use the [integrated terminal](https://code.visualstudio.com/docs/editor/integrated-terminal) functionality.

### Running

1. Run the project

```sh
cargo run
```

2. Visit [localhost:8000](http://localhost:8000/)
3. To test a command, go to [localhost:8000/search?cmd=tw](http://localhost:8000/search?cmd=tw) and you should be redirected to Twitter

### Testing

Run the following command

```sh
cargo test
```

<!-- USAGE EXAMPLES -->

## Usage

To test out a command, type in http://localhost:8000/search?cmd= followed by your command.

The following commands are supported by `rusty-bunny`:

- "tw" -> redirects to twitter.com
- "tw @username" -> redirects to twitter.com/username
- "gh" -> redirects to github.com
- "gh username" -> redirects to github.com/username
- "gh username/repo" -> redirects to github.com/username/repo

Everything else redirects to a google search with your query.

<!-- CONTRIBUTING -->

## Contributing

Contributions are what make the open source community such an amazing place to be learn, inspire, and create. Any contributions you make are **greatly appreciated**. See [`CONTRIBUTING`](CONTRIBUTING.md) for more information.

<!-- LICENSE -->

## License

Distributed under the MIT License. See [`LICENSE`](LICENSE) for more information.

<!-- CONTACT -->

## Contact

If you have questions or thoughts on this project, feel free to send them my way by @faraazofficial'ing me on Twitter or shooting me a DM.

Faraaz Baig - [@faraazofficial](https://twitter.com/faraazofficial)

<!-- ACKNOWLEDGEMENTS -->

## Acknowledgements

- [The Rust Community](https://www.rust-lang.org/community)
- [Rocket.rs](https://rocket.rs/)

<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->

[product-screenshot]: demo.gif
