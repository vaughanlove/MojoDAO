import * as borsh from "borsh";
import {GreetingAccount, GreetingSchema} from "@/plugins/solana3";
import {Connection, PublicKey} from "@solana/web3.js";
import Wallet from "@project-serum/sol-wallet-adapter";
import {ProgramInfo} from "@/utils/subscription-list";

export async function getDurationFromKey(web3: Connection, programID: PublicKey, walletKey: PublicKey){
    const GREETING_SEED = 'spotifysub'
    const key = await PublicKey.createWithSeed(walletKey, GREETING_SEED, programID).then(e => {return e})
    const greetedAccount = await web3.getAccountInfo(key);
    if (greetedAccount != null){
        const result =  borsh.deserialize(
            GreetingSchema,
            GreetingAccount,
            greetedAccount!.data,
        )
        return 'user subbed until ' + result.counter + ' hours'
    } else {
        const result  = 'user not subscribed'
        return result
    }
}

export async function findAssociatedAccount(programkey: PublicKey, wallet: Wallet){
    const GREETING_SEED = 'spotifysub';
    const greetingSeed = await PublicKey.createWithSeed(
        wallet.publicKey!,
        GREETING_SEED,
        programkey,
    ).then(e => {return e});
    return greetingSeed
}

export async function derivePDAS(keys: ProgramInfo[], web3: Connection) {
    for (const e of keys) {
        const result = await web3.getProgramAccounts(new PublicKey(e.programKey))
        result.forEach(e => {console.log(e.pubkey.toString())})
    }
}