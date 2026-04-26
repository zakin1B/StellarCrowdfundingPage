# Stellar Crowdfunding Page

## Project Description

Stellar Crowdfunding Page is a simple crowdfunding smart contract built on Stellar Testnet using Soroban.

The project allows a campaign owner to create a fundraising campaign and allows users to donate to that campaign. The contract tracks the total amount raised and each donor's contribution.

This project is built for Level 2 of the Stellar builder challenge, focusing on smart contract deployment, contract interaction, reading and writing contract data, and basic event tracking.

---

## Project Vision

The vision of this project is to create a transparent crowdfunding page where donations can be tracked on-chain.

In a real-world version, this idea can be used for:

- Student builder fundraising
- Community grant campaigns
- Open-source project donations
- Event or hackathon sponsorship pools
- Public charity donation tracking

By using Stellar, the donation progress can be transparent, fast, and easy to verify.

---
## Future Scope

In the future, this project can be improved with:

- Real XLM or token transfer support
- Multiple crowdfunding campaigns
- Campaign deadline
- Refund logic if the campaign does not reach its goal
- Frontend crowdfunding page
- Real-time donation progress bar
- Donor leaderboard
- Wallet connection through Freighter
- Campaign image and category
- Admin dashboard for campaign owners

## Key Features

- Create a crowdfunding campaign
- Store campaign title, description, owner, goal amount, and total raised amount
- Donate to the campaign
- Track total raised amount
- Track individual donor contribution
- Read campaign information from the contract
- Emit events when a campaign is created or when a donation is made
- Deployed and tested on Stellar Testnet

---
## Deployed Contract Details

Network: Stellar Testnet

Contract ID:

```text
CCFR67IVIOM4B3SEHUJHZEUVLHYBGIRKNM73KI27EGYOOWIJZMLACPG6
## Smart Contract Functions

### `create_campaign(owner, title, description, goal_amount)`

Creates a new crowdfunding campaign.

Parameters:

- `owner`: campaign owner address
- `title`: campaign title
- `description`: campaign description
- `goal_amount`: target fundraising amount

Example campaign:

```text
Title: Stellar Student Fund
Description: Support student builders on Stellar
Goal Amount: 1000

##Scrrenshot:
![screenshot](contract3.png)
