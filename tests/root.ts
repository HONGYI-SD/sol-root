import assert = require("assert");
import * as anchor from '@coral-xyz/anchor';
const { SystemProgram } = anchor.web3;

describe("root", () => {
  const provider = anchor.AnchorProvider.local();
  anchor.setProvider(provider);

  const counter = anchor.web3.Keypair.generate();
  const other = anchor.web3.Keypair.generate();
  const program = anchor.workspace.root;

  it("Init", async () => {
    await program.methods
      .initialize()
      .accounts({
        counter: counter.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([counter])
      .rpc();

    let counterAccount = await program.account.counter.fetch(counter.publicKey);
    assert.ok(counterAccount.authority.equals(provider.wallet.publicKey));
    assert.ok(counterAccount.initialized == true);
  });

  it("Insert", async () => {
    const slotHeight = new anchor.BN(1); // 示例槽高度
    const root1 = new Uint8Array(32).fill(1); // 示例根1
    const root2 = new Uint8Array(32).fill(2); // 示例根2

    // console.log(slotHeight, root1, root2)
    await program.methods
      .insert(slotHeight, root1, root2)
      .accounts({
        counter: counter.publicKey,
        authority: provider.wallet.publicKey,
      })
      .rpc();

    const counterAccount = await program.account.counter.fetch(counter.publicKey);
    assert.ok(counterAccount.merkleData.length === 1); // 验证数据是否存储
    assert.ok(counterAccount.merkleData[0].slotHeight.cmp(slotHeight) == 0);
    const hex1 = uint8ArrayToHex(root1);
    const hex2 = uint8ArrayToHex(root2);
    const hex3 = uint8ArrayToHex(counterAccount.merkleData[0].root1);
    const hex4 = uint8ArrayToHex(counterAccount.merkleData[0].root2);
    assert.ok(hex1 === hex3);
    assert.ok(hex2 === hex4);
  });

  it("Insert two", async () => {
    const slotHeight = new anchor.BN(2); // 示例槽高度
    const root1 = new Uint8Array(32).fill(3); // 示例根1
    const root2 = new Uint8Array(32).fill(4); // 示例根2

    // console.log(slotHeight, root1, root2)
    await program.methods
      .insert(slotHeight, root1, root2)
      .accounts({
        counter: counter.publicKey,
        authority: provider.wallet.publicKey,
      })
      .rpc();

    const counterAccount = await program.account.counter.fetch(counter.publicKey);
    assert.ok(counterAccount.merkleData.length === 2); // 验证数据是否存储
    assert.ok(counterAccount.merkleData[1].slotHeight.cmp(slotHeight) == 0);
    const hex1 = uint8ArrayToHex(root1);
    const hex2 = uint8ArrayToHex(root2);
    const hex3 = uint8ArrayToHex(counterAccount.merkleData[1].root1);
    const hex4 = uint8ArrayToHex(counterAccount.merkleData[1].root2);
    assert.ok(hex1 === hex3);
    assert.ok(hex2 === hex4);
  });
});



function uint8ArrayToHex(arr: Uint8Array): string {
  return arr.reduce((acc, val) => acc + val.toString(16).padStart(2, '0'), '');
}
