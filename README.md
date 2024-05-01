# BARK Staking DApp

![BARK Staking DApp Screenshot](https://github.com/bark-community/bark-staking-dapp/.github/blob/main/screenshot.png)

## Overview

The BARK Staking DApp is a decentralized application (DApp) built on the Solana blockchain. It allows $BARK token holders to stake their tokens and earn rewards through a staking mechanism. This repository provides an overview of the project, including its features, setup instructions, and usage guidelines.

## Features

- **Staking Platform**: Users can connect their wallets and stake $BARK tokens.
- **Integration with BARK Token Program**: The DApp integrates with the client's existing $BARK token program.
- **Transfer Fee Mechanism**: A 5% transfer fee is applied to staking and unstaking transactions, with fees collected in a treasury wallet.
- **Reward Distribution**: Rewards are distributed weekly to stakers.
- **Referral System**: Users can benefit from a 15% rebate through a referral program.
- **APY Cap**: The APY is capped at 100% to ensure sustainable growth.
- **Frontend**: A user-friendly interface displays staking information and allows for easy interaction.
- **Error Handling and Logging**: Proper error handling, input validation, and event logging ensure the reliability and security of the DApp.

## Technologies Used

- Rust
- Anchor
- Solana CLI
- Alchemy RPC URL for building and deploying
- Solana Playground for initial testing and development

## Setup Instructions

1. Clone the repository to your local machine.
2. Install Rust and Solana CLI if not already installed.
3. Run `anchor build` to build the project.
4. Deploy the smart contract using Solana CLI.
5. Set up the frontend by installing necessary dependencies and configuring the connection to the deployed smart contract.
6. Launch the frontend application.

## Usage

1. Connect your wallet to the DApp.
2. Stake $BARK tokens using the provided interface.
3. Monitor staking performance and rewards earned.
4. Refer others to the platform to earn additional rewards.
5. Withdraw staked tokens and rewards as needed.

## License

[Apache 2.0 License](LICENSE).

## References

- [Solana Documentation](https://docs.solana.com/)
- [Anchor Programming Language](https://www.anchor-lang.com/docs)
- [Rust Programming Language](https://www.rust-lang.org/)
- [Solana Labs](https://solana.com/)