// Migrations are an early feature. Currently, they're nothing more than this
// single deploy script that's invoked from the CLI, injecting a provider
// configured from the workspace's Anchor.toml.

const anchor = require("@coral-xyz/anchor");

module.exports = async function (provider) {
  try {
    // Configure client to use the provider.
    anchor.setProvider(provider);

    // Load the program from the compiled Anchor IDL.
    const program = anchor.workspace.BARKStakingProgram;

    // Check if the program is already deployed.
    const programId = program.programId;
    const programInfo = await provider.connection.getAccountInfo(programId);
    if (programInfo) {
      console.log("Program already deployed:", programId.toBase58());
      return;
    }

    // Deploy the program.
    const publicKey = await deployProgram(provider, program);
    
    console.log("Program deployed to:", publicKey.toBase58());
  } catch (error) {
    console.error("Error deploying program:", error);
  }
};

async function deployProgram(provider, program) {
  const programId = program.programId;
  const space = 16384; // Adjust based on program size.
  const lamports = await provider.connection.getMinimumBalanceForRentExemption(space);
  
  const [programAddress] = await anchor.web3.PublicKey.findProgramAddress(
    [programId.toBuffer()],
    programId
  );

  const transaction = new anchor.web3.Transaction().add(
    anchor.web3.SystemProgram.createAccount({
      fromPubkey: provider.wallet.publicKey,
      newAccountPubkey: programAddress,
      space,
      lamports,
      programId,
    })
  );

  const options = {
    commitment: "singleGossip",
    preflightCommitment: "singleGossip",
  };

  await provider.send(transaction, [provider.wallet], options);

  return programAddress;
}
