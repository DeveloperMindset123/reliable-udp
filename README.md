<h1>Reliable User Datagram Protocol (UDP)</h1>
<p>A simple rust based project that will send variable number of requests from the client side to the server side. The request that is being sent in this case, for the purpose of simplicity is a simple "ping" message, and the server upon recieving the message will respond with "pong", alongside the current sequence number as well as the round trip time. Round trip time is used to calculate the time it takes for a client to send a request and to recieve a response back. </p>

<p>The program sends 10 <b>Ping ➡️</b> messages back-to-back, and ensures that 10 <b>Pong ⬅️</b> messages are sent back.</p>


### Built With
* ![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)

## Getting Started
If you would like to locally rn the project, please follow the instructions below

### Prerequisites
Ensure that rust is installed on your operating system if this is your first time working with Rust. If you are fmiliar with Rust, you can skip subsequent steps below.
* Install Rust 
  ```sh
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh 
  ```
  [Reference Link](https://www.rust-lang.org/tools/install)

* Open up a new terminal and ensure that `rustc`, `cargo` and `rustup` are installed, otherwise, you will need to configure them as part of your `PATH` variable.
  ```sh
  rustc --version
  ```

  ```sh
  cargo --version
  ```

  ```
  rustup --version
  ```

  ### Installation

1. Setup your bot and retrieve it's token following the guide and add the server to a server for testing [Setup Instructions](https://discordjs.guide/preparations/setting-up-a-bot-application.html)
2. Clone the repository
   ```sh
   https://github.com/DeveloperMindset123/reliable-udp.git
   ```
3. Move to the directory where it is intalled
   ```sh
   cd reliable-udp
   ```
4. Run the following command to ensure that all the relevant dependancies are installed:
   ```sh
   cargo build
   ```
6. Change git remote url to avoid accidental pushes to base project
   ```sh
   git remote set-url origin github_username/repo_name
   git remote -v # confirm the changes
   ```

### Demo
