# League Stats Checker

League Stats Checker is a Rust project that allows you to retrieve information about a League of Legends summoner using the Riot Games API. This project demonstrates the usage of asynchronous programming, interacting with web APIs, and handling JSON data in Rust.

## Prerequisites

Before running the League Stats Checker, ensure you have the following:

- Rust programming language installed on your system.
- An API key from Riot Games. You can obtain one by creating an account on the [Riot Developer Portal](https://developer.riotgames.com/).

## Setup

1. Clone the repository or download the source code for the League Stats Checker project.
2. Create a file named `.env` in the project root directory.
3. Open the `.env` file and add the following line, replacing `<YOUR_API_KEY>` with your actual Riot Games API key:

   ```
   API_KEY=<YOUR_API_KEY>
   ```

4. Save the `.env` file.

## Usage

1. Open a terminal or command prompt and navigate to the project directory.
2. Build the project by running the following command:

   ```shell
   cargo build
   ```

3. Run the League Stats Checker using the following command:

   ```shell
   cargo run
   ```

4. You will be prompted to enter the summoner's name. Type the name and press Enter.
5. Next, choose the server for which you want to retrieve the summoner's information. Type the server code (e.g., `euw1`) and press Enter.
6. The program will then load the summoner's stats and display the retrieved information, including the summoner's name, ID, and level.

## Supported Servers

League Stats Checker supports the following servers:

- Brazil (BR1)
- Europe Nordic & East (EUN1)
- Europe West (EUW1)
- Latin America North (LA1)
- Latin America South (LA2)
- North America (NA1)
- Oceania (OCE/OC1)
- Russia (RU1)
- Turkey (TR1)
- Japan (JP1)
- Republic of Korea (KR)
- The Philippines (PH2)
- Singapore, Malaysia, & Indonesia (SG2)
- Taiwan, Hong Kong, and Macao (TW2)
- Thailand (TH2)
- Vietnam (VN2)

Make sure to enter the server code exactly as shown in the list.

## Error Handling

- If the server code you entered is not valid, the program will display an error message and exit.
- If the summoner is not found, the program will display an appropriate message.

## Contributing

Contributions to the League Stats Checker project are welcome. If you encounter any issues or have suggestions for improvements, please submit them through the project's issue tracker on GitHub.

## License

League Stats Checker is released under the [MIT License](LICENSE). Feel free to modify and distribute the code according to the terms of the license.

## Acknowledgments

This project was created as a learning exercise and as an example of Rust programming using asynchronous requests and JSON handling. It relies on the following external crates:

- `reqwest` - For making HTTP requests.
- `dotenv` - For loading environment variables from the `.env` file.
- `serde_json` - For parsing and handling JSON data.
- `colored` - For colorful console output.

Please refer to the documentation of these crates for more information.
