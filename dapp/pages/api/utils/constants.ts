import idl from "../idl/bark_staking_program.json";
import { Connection, PublicKey, clusterApiUrl } from "@solana/web3.js";

/* Constants for RPC Connection the Solana Blockchain */
export const commitmentLevel = "processed";
export const endpoint =
  process.env.NEXT_PUBLIC_ALCHEMY_RPC_URL || clusterApiUrl("devnet");
export const connection = new Connection(endpoint, commitmentLevel);

/* Constants for the Deployed "Hello World" Program */
export const barkStakingprogramId = new PublicKey(idl.metadata.address);
export const barkStakingprogramInterface = JSON.parse(JSON.stringify(idl));
