import '@frequency-chain/api-augment';
import assert from 'assert';
import { base64UrlToUint8Array, createAndFundKeypair, getNonce } from '../scaffolding/helpers';
import { KeyringPair } from '@polkadot/keyring/types';
import { ExtrinsicHelper } from '../scaffolding/extrinsicHelpers';
import { getFundingSource } from '../scaffolding/funding';
import { SubmittableExtrinsic } from '@polkadot/api/types';
import { ISubmittableResult } from '@polkadot/types/types';

const fundingSource = getFundingSource('passkey-proxy');

describe('Passkey Pallet Tests', function () {
  let fundedKeys: KeyringPair;
  let receiverKeys: KeyringPair;

  before(async function () {
    fundedKeys = await createAndFundKeypair(fundingSource, 50_000_000n);
    receiverKeys = await createAndFundKeypair(fundingSource, 0n);
  });

  describe('proxy', function () {
    it('should fail due to unsupported call', async function () {
      const accountPKey = fundedKeys.publicKey;
      const passkeyPublicKey = new Uint8Array(33);
      const nonce = await getNonce(fundedKeys);
      const accountSignature = fundedKeys.sign(passkeyPublicKey);
      
      // base64URL encoded strings
      const badPassKeySignatureBase64URL = Buffer.from("badPassKeySignature").toString('base64url');
      const authenticatorDataBase64URL = Buffer.from("authenticatorData").toString('base64url');
      const clientDataJsonBase64URL = Buffer.from("clientDataJson").toString('base64url');
      
      const remarksCalls = ExtrinsicHelper.api.tx.system.remark("passkey-test");

      const payload = createPayload(
        accountPKey,
        passkeyPublicKey,
        nonce,
        accountSignature,
        badPassKeySignatureBase64URL,
        authenticatorDataBase64URL,
        clientDataJsonBase64URL,
        remarksCalls,
      );

      const passkeyProxy = ExtrinsicHelper.executePassKeyProxy(fundedKeys, payload);
      assert.rejects(passkeyProxy.fundAndSendUnsigned(fundingSource));
    });

    it('should fail to transfer balance due to bad signature', async function () {
      const accountPKey = fundedKeys.publicKey;
      const passkeyPublicKey = new Uint8Array(33);
      const nonce = await getNonce(fundedKeys);
      const accountSignature = fundedKeys.sign(passkeyPublicKey);
      
      // base64URL encoded strings
      const badPassKeySignatureBase64URL = Buffer.from("badPassKeySignature").toString('base64url');
      const authenticatorDataBase64URL = Buffer.from("authenticatorData").toString('base64url');
      const clientDataJsonBase64URL = Buffer.from("clientDataJson").toString('base64url');
      
      const transferCalls = ExtrinsicHelper.api.tx.balances.transferAllowDeath(receiverKeys.publicKey, 1000000);

      const payload = createPayload(
        accountPKey,
        passkeyPublicKey,
        nonce,
        accountSignature,
        badPassKeySignatureBase64URL,
        authenticatorDataBase64URL,
        clientDataJsonBase64URL,
        transferCalls,
      );

      const passkeyProxy = ExtrinsicHelper.executePassKeyProxy(fundedKeys, payload);
      assert.rejects(passkeyProxy.fundAndSendUnsigned(fundingSource));
    });
})});

function createPayload(
  accountPKey: Uint8Array,
  passkeyPublicKey: Uint8Array,
  nonce: number,
  accountSignature: Uint8Array,
  passkeySignature: string,
  authenticatorData: string,
  clientDataJson: string,
  call: SubmittableExtrinsic<"rxjs", ISubmittableResult>,
) {
  const passkeyCall = {
    accountId: accountPKey,
    accountNonce: nonce,
    accountOwnershipProof: {
      Sr25519: accountSignature,
    },
    call: call,
  };

  let passkeyCallType = ExtrinsicHelper.api.createType('PalletPasskeyPasskeyCall', passkeyCall);
  const passkeyPayload = {
    passkeyPublicKey: Array.from(passkeyPublicKey),
    verifiablePasskeySignature: {
      signature: Array.from(base64UrlToUint8Array(passkeySignature)),
      authenticatorData: Array.from(base64UrlToUint8Array(authenticatorData)),
      clientDataJson: Array.from(base64UrlToUint8Array(clientDataJson)),
    },
    passkeyCall: passkeyCallType,
  };

  return passkeyPayload;
}
