# BARK Staking Program (nontested)

The BARK Staking Program is a decentralized application (dApp) built on the Solana blockchain that allows users to stake their tokens and earn rewards. The program is designed to provide a secure and efficient mechanism for users to participate in staking activities and earn rewards based on their contributions to the network.

## Features

- **Stake BARK Tokens**: Users can stake BARK tokens and earn rewards based on their staked amount.
- **Referral Bonuses**: Users receive additional rewards for successful referrals, encouraging network growth.
- **Flexible Withdrawal**: Users can withdraw their staked tokens at any time, providing flexibility and liquidity.

### Reward Distribution

1. **Accrual**: As network operations generate rewards, they accrue in the staking contract's reward pool.
2. **Distribution**: Periodically, the accrued rewards are distributed among the stakers based on their contribution to the network. This process is triggered by calling the `distribute_rewards` function.
3. **Proportional Allocation**: Rewards are distributed proportionally to each staker based on their share of the total staked tokens.
4. **Batched Token Transfers**: To optimize gas usage, token transfers for reward distribution are batched, reducing transaction costs and improving efficiency.

## Architecture

The BARK Staking Program consists of the following components:

1. **Staking Contract**: The core smart contract deployed on the Solana blockchain. It manages staked tokens, rewards accrual, and distribution.
2. **Token Accounts**: Solana token accounts hold the staked tokens and reward distributions for each user.
3. **Token Mint**: The token mint represents the token's source of issuance and defines its properties such as total supply and decimals.
4. **Solana Blockchain**: The underlying blockchain network where all transactions and contract interactions take place.

## Program Flow

1. Users interact with the BARK Staking Program by calling its functions using Solana-compatible wallets or applications.
2. Upon staking tokens, the user's tokens are transferred to the staking contract, and their staking information is updated.
3. As network operations generate rewards, they accrue in the staking contract's reward pool.
4. Periodically, the accrued rewards are distributed among the stakers based on their contribution to the network.
5. Stakers can claim their rewards or restake them to further increase their staking balance and rewards.

## Summary

The BARK Staking Program provides a secure, efficient, and user-friendly platform for token holders to stake their tokens and earn rewards. By leveraging the Solana blockchain's capabilities, it offers low transaction costs, fast transaction confirmation times, and high scalability, making it an ideal choice for staking activities.

## Getting Started

To get started with the BARK Staking Program, follow these steps:

1. Clone the repository:

```bash
git clone https://github.com/bark-community/bark-staking-program.git
```

2. Navigate to the project directory:

```bash
cd programs && staking
```

3. Build the program:

```bash
anchor build
```

4. Deploy the program to the Solana blockchain.

## Usage

To use the BARK Staking Program, interact with it through a Solana wallet or integrate it into your own applications using the provided program ID.

## Contributing

Contributions to the BARK Staking Program are welcome! If you'd like to contribute, please fork the repository, make your changes, and submit a pull request.

## License

[MIT License](LICENSE).
