import { ApiRx, WsProvider } from "@polkadot/api";
import { Keyring } from "@polkadot/api";

import assert from "assert";

import { AVRO_GRAPH_CHANGE } from "./scaffolding/schemaTypes";
import { AVRO } from "./scaffolding/modelTypes";
import { ON_CHAIN } from "./scaffolding/payloadLocation";
import { filter, firstValueFrom, mergeMap, Observable } from "rxjs";
import { groupEventsByKey } from "./scaffolding/helpers";

describe("#createSchema", () => {
    let apiObservable: Observable<ApiRx>;
    let keys: any;

    beforeEach(() => {
        const provider = new WsProvider("ws://127.0.0.1:9944");
        apiObservable = ApiRx.create({ provider });
        const keyring = new Keyring({ type: "sr25519" });
        keys = keyring.addFromUri("//Alice");
    })

    it("should successfully create an Avro GraphChange schema", async () => {
        const chainEvents = await firstValueFrom(
            apiObservable.pipe(
                mergeMap((api) => api.tx.schemas.createSchema(JSON.stringify(AVRO_GRAPH_CHANGE), AVRO, ON_CHAIN).signAndSend(keys).pipe(
                filter(({status}) => status.isInBlock),
                groupEventsByKey()
            ))))
        assert.equal(chainEvents["system.ExtrinsicFailed"], undefined);
        assert.notEqual(chainEvents["system.ExtrinsicSuccess"], undefined);
        assert.notEqual(chainEvents["schemas.SchemaCreated"], undefined);
    }).timeout(10000);
})
