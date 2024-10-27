import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { ZkCounter } from "../target/types/zk_counter";
import {
  LightSystemProgram,
  NewAddressParams,
  Rpc,
  bn,
  createRpc,
  defaultStaticAccountsStruct,
  defaultTestStateTreeAccounts,
  deriveAddress,
  PackedMerkleContext,
  CompressedAccountWithMerkleContext,
} from "@lightprotocol/stateless.js";
import dotenv from "dotenv";
import {
  AccountMeta,
  ComputeBudgetProgram,
  Keypair,
  PublicKey,
} from "@solana/web3.js";
import { ZkCounterStruct } from "./helpers/types";
import {
  buildSignAndSendTransaction,
  deriveAddressSeed,
  formatRemainingAccounts,
  getNewAddressParams,
  packNew,
  packWithInput,
} from "./helpers/compression";
import * as borsh from "borsh";
import { expect } from "chai";

dotenv.config();

// Request more compute units
const modifyComputeUnits = ComputeBudgetProgram.setComputeUnitLimit({
  units: 1000000,
});

const addPriorityFee = ComputeBudgetProgram.setComputeUnitPrice({
  microLamports: 1,
});

describe("ZkCounter", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.ZkCounter as Program<ZkCounter>;
  const deployer = anchor.workspace.zk_counter.provider.wallet.payer as Keypair;

  const rpcUrl = process.env.RPC_URL;
  const connection: Rpc = createRpc(rpcUrl, rpcUrl, rpcUrl, {
    commitment: "confirmed",
  });

  const {
    accountCompressionAuthority,
    noopProgram,
    registeredProgramPda,
    accountCompressionProgram,
  } = defaultStaticAccountsStruct();
  const { addressTree } = defaultTestStateTreeAccounts();
  let merkleContext: PackedMerkleContext;
  let addressMerkleContext: {
    addressMerkleTreePubkeyIndex: number;
    addressQueuePubkeyIndex: number;
  };
  let addressMerkleTreeRootIndex: number;
  let remainingAccounts: PublicKey[];

  const payer = (provider.wallet as any).payer.publicKey;

  let addressSeed: Uint8Array;
  let address: PublicKey;

  before(async () => {
    addressSeed = deriveAddressSeed(
      [Buffer.from("counter"), payer.toBuffer()],
      program.programId,
      addressTree
    );

    address = await deriveAddress(addressSeed, addressTree);
  });

  it("Should create a new account", async () => {
    const proof = await connection.getValidityProof(undefined, [
      bn(address.toBytes()),
    ]);

    const newAddressParams = getNewAddressParams(addressSeed, proof);
    const outputCompressedAccounts =
      LightSystemProgram.createNewAddressOutputState(
        Array.from(address.toBytes()),
        program.programId
      );

    ({
      merkleContext,
      remainingAccounts,
      addressMerkleContext,
      addressMerkleTreeRootIndex,
    } = packNew(outputCompressedAccounts, [newAddressParams], proof));

    const parameters: ZkCounterStruct<"create"> = {
      inputs: [],
      proof: proof.compressedProof,
      merkleContext,
      merkleTreeRootIndex: 0,
      addressMerkleContext,
      addressMerkleTreeRootIndex,
    };

    const instructions = await program.methods
      .create(...(Object.values(parameters) as any))
      .accounts({
        signer: provider.wallet.publicKey,
        selfProgram: program.programId,
        lightSystemProgram: LightSystemProgram.programId,
        accountCompressionAuthority,
        noopProgram,
        accountCompressionProgram,
        registeredProgramPda,
        systemProgram: anchor.web3.SystemProgram.programId,
        cpiSigner: PublicKey.findProgramAddressSync(
          [Buffer.from("cpi_authority")],
          program.programId
        )[0],
      })
      .remainingAccounts(formatRemainingAccounts(remainingAccounts))
      .instruction();

    const txSignature = await buildSignAndSendTransaction(
      [modifyComputeUnits, addPriorityFee, instructions],
      deployer,
      connection
    );
    console.log("Your transaction signature", txSignature);
  });

  it("Check if the account is created", async () => {
    const accounts = await connection.getCompressedAccountsByOwner(
      program.programId
    );

    const compressedAccount = accounts.items[0];
    const counterAccount = compressedAccount.data;

    const CounterSchema = {
      struct: {
        owner: { array: { type: "u8", len: 32 } },
        counter: "u64",
      },
    };
    const decoded: any = borsh.deserialize(CounterSchema, counterAccount.data);
    expect(decoded.counter).to.eq(BigInt(0));
    expect(new PublicKey(Uint8Array.from(decoded.owner)).toBase58()).to.eq(
      provider.wallet.publicKey.toBase58()
    );
  });

  it("Should be able to increment the counter", async () => {
    const accounts = await connection.getCompressedAccountsByOwner(
      program.programId
    );

    const compressedAccount = accounts.items[0];

    const proof = await connection.getValidityProof(
      [bn(compressedAccount.hash)],
      undefined
    );

    ({
      merkleContext,
      addressMerkleContext,
      remainingAccounts,
      addressMerkleTreeRootIndex,
    } = packWithInput([compressedAccount], [], [], proof));

    const parameters: ZkCounterStruct<"increment"> = {
      inputs: [compressedAccount.data.data],
      proof: proof.compressedProof,
      merkleContext,
      merkleTreeRootIndex: proof.rootIndices[0],
      addressMerkleContext,
      addressMerkleTreeRootIndex,
    };

    const instructions = await program.methods
      .increment(...(Object.values(parameters) as any))
      .accounts({
        signer: provider.wallet.publicKey,
        selfProgram: program.programId,
        lightSystemProgram: LightSystemProgram.programId,
        accountCompressionAuthority,
        noopProgram,
        accountCompressionProgram,
        registeredProgramPda,
        systemProgram: anchor.web3.SystemProgram.programId,
        cpiSigner: PublicKey.findProgramAddressSync(
          [Buffer.from("cpi_authority")],
          program.programId
        )[0],
      })
      .remainingAccounts(formatRemainingAccounts(remainingAccounts))
      .instruction();

    const txSignature = await buildSignAndSendTransaction(
      [modifyComputeUnits, addPriorityFee, instructions],
      deployer,
      connection
    );
    console.log("Your transaction signature", txSignature);
  });

  it("Should have incremented the counter", async () => {
    const accounts = await connection.getCompressedAccountsByOwner(
      program.programId
    );

    const compressedAccount = accounts.items[0];
    const counterAccount = compressedAccount.data;

    const CounterSchema = {
      struct: {
        owner: { array: { type: "u8", len: 32 } },
        counter: "u64",
      },
    };
    const decoded: any = borsh.deserialize(CounterSchema, counterAccount.data);
    expect(decoded.counter).to.eq(BigInt(1));
  });

  it("Should be able to delete the account", async () => {
    const accounts = await connection.getCompressedAccountsByOwner(
      program.programId
    );

    const compressedAccount = accounts.items[0];

    const proof = await connection.getValidityProof(
      [bn(compressedAccount.hash)],
      undefined
    );

    ({
      merkleContext,
      addressMerkleContext,
      remainingAccounts,
      addressMerkleTreeRootIndex,
    } = packWithInput([compressedAccount], [], [], proof));

    const parameters: ZkCounterStruct<"delete"> = {
      inputs: [compressedAccount.data.data],
      proof: proof.compressedProof,
      merkleContext,
      merkleTreeRootIndex: proof.rootIndices[0],
      addressMerkleContext,
      addressMerkleTreeRootIndex,
    };

    const instructions = await program.methods
      .delete(...(Object.values(parameters) as any))
      .accounts({
        signer: provider.wallet.publicKey,
        selfProgram: program.programId,
        lightSystemProgram: LightSystemProgram.programId,
        accountCompressionAuthority,
        noopProgram,
        accountCompressionProgram,
        registeredProgramPda,
        systemProgram: anchor.web3.SystemProgram.programId,
        cpiSigner: PublicKey.findProgramAddressSync(
          [Buffer.from("cpi_authority")],
          program.programId
        )[0],
      })
      .remainingAccounts(formatRemainingAccounts(remainingAccounts))
      .instruction();

    const txSignature = await buildSignAndSendTransaction(
      [modifyComputeUnits, addPriorityFee, instructions],
      deployer,
      connection
    );
    console.log("Your transaction signature", txSignature);
  });

  it("Should not be able to find the deleted account", async () => {
    const accounts = await connection.getCompressedAccountsByOwner(
      program.programId
    );
    expect(accounts.items.length).to.eq(0);
  });
});
