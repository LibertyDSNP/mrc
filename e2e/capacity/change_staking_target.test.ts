import "@frequency-chain/api-augment";
import assert from "assert";
import { ExtrinsicHelper, } from "../scaffolding/extrinsicHelpers";
import { getFundingSource } from '../scaffolding/funding';
import {
  createKeys, createMsaAndProvider,
  stakeToProvider,
  CENTS, DOLLARS, createAndFundKeypair, createProviderKeysAndId, getNonce,
} from '../scaffolding/helpers';
import { KeyringPair } from '@polkadot/keyring/types';

const fundingSource = getFundingSource('capacity-replenishment');

describe("Capacity: change_staking_target", function() {
  const tokenMinStake: bigint = 1n * CENTS;
  const capacityMin: bigint = tokenMinStake / 50n;

  it("happy path succeeds", async function() {
      const providerBalance = 2n * DOLLARS;
      const stakeKeys = createKeys("staker");
      const oldProvider = await createMsaAndProvider(fundingSource, stakeKeys, "Provider1", providerBalance);
      const [_bar, newProvider] = await createProviderKeysAndId(fundingSource, providerBalance);

      await assert.doesNotReject(stakeToProvider(fundingSource, stakeKeys, oldProvider, tokenMinStake*3n));

      const call = ExtrinsicHelper.changeStakingTarget(stakeKeys, oldProvider, newProvider, tokenMinStake);
      const events = await call.signAndSend();
      assert.notEqual(events, undefined);
  });

  // not intended to be exhaustive, just check one error case
  it("fails if 'to' is not a Provider", async function() {
    const providerBalance = 2n * DOLLARS;
    const stakeKeys = createKeys("staker");
    const oldProvider = await createMsaAndProvider(fundingSource, stakeKeys, "Provider2", providerBalance);

    await assert.doesNotReject(stakeToProvider(fundingSource, stakeKeys, oldProvider, tokenMinStake*6n));
    const notAProvider = 9999;
    const call = ExtrinsicHelper.changeStakingTarget(stakeKeys, oldProvider, notAProvider, tokenMinStake*2n);
    await assert.rejects(call.signAndSend(),
      (err) => {
        assert. strictEqual(err?.name, 'InvalidTarget', `expected InvalidTarget, got ${err?.name}`);
        return true;
    });
  });

  it("foo", async function(){
    const fundedKeys: KeyringPair  =  await createAndFundKeypair(fundingSource, 100_000_000n);
    const receiverKeys: KeyringPair = await createAndFundKeypair(fundingSource);
    const accountPKey = fundedKeys.publicKey;
    const nonce = await getNonce(fundedKeys);

    const transferCalls = ExtrinsicHelper.api.tx.balances.transferKeepAlive(receiverKeys.publicKey, 100n);
    transferCalls.signAndSend(fundedKeys);
  });
});
