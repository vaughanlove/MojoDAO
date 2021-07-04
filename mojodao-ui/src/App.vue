<template>
  <div class="app">
    <h1>welcome to the mojodao ui</h1>
    <p>Your parent (payer) account: <strong>{{ this.ownerAccountString }}</strong></p>
    <p> Your subscriptions (programID)</p>
    <ul>
      <li><strong>{{programKey.toString()}}</strong> with the PDA <strong>{{derivedSubAccount.toString()}}</strong> is currently subscribed until {{subUntil}}</li>
    </ul>

    <p>the current time: <strong>{{Math.round(Date.now() / 3600000)}}</strong></p>

    <button @click="connectWallet()">connect</button>
    <div></div>
    <button @click="findAssociatedAccount()">SEARCH</button>
    <div></div>
    <button @click="derivePDA()">lets see</button>
    <div></div>
    <button @click="readPubkey()">read key</button>
    <div></div>
    <button @click="saySub()">subscribe</button>
    <div></div>
    <button @click="viewPrograms()">programs</button>
  </div>
</template>

<script lang="ts">
import {defineComponent} from 'vue';
import {createConnection, GreetingAccount, GreetingSchema} from './plugins/solana3'
import {PublicKey, TransactionInstruction} from "@solana/web3.js";
import * as borsh from 'borsh';
import { ref } from 'vue'
import { Transaction } from '@solana/web3.js';
import { sendAndConfirmTransaction} from "@solana/web3.js";
import Wallet from '@project-serum/sol-wallet-adapter'
import SolanaWalletAdapter from '@project-serum/sol-wallet-adapter'
import {ProgramInfo, PROGRAMS} from "@/utils/subscription-list";

export default defineComponent({
  name: 'App',
  components: {},

  data() {
    return {
      web3: createConnection('https://api.devnet.solana.com'),
      subUntil: 0,
    }
  },

  setup(){
    const parentAccount = new PublicKey('8jybqN8p1oBYLPMouTKX8RrBXXR78yzvzBzzTgN7vySM')
    // this program key is constant because its the only deployed, but ultimately i need to have a list of objects containing them + names
    const programKey = new PublicKey('EDTtxSugqhzghWB8aJtHoqktTfnBuTtbHFWYDGs6A8Rs')

    // this derived sub account should be calculated on wallet connection
    const derivedSubAccount = new PublicKey('5wenbx6kdzuhNykrvrHTFdiAhRRwPg1XrnQENerF6zVz')

    // maybe should be just type? Not sure. Test more.
    const wallet = new SolanaWalletAdapter('https://www.sollet.io', "https://api.devnet.solana.com")

    // I really like how clean and organized this is. ++Has built in reactivity.
    const ownerAccount = ref<PublicKey>()
    const ownerAccountString = ref<string>()
    const subscriptionList = ref<Array<ProgramInfo>>()

    return {parentAccount, derivedSubAccount, programKey, wallet,ownerAccountString, ownerAccount, subscriptionList}
  },
  methods: {
    async connectWallet(){
      await this.wallet.connect()
      this.ownerAccount = this.wallet.publicKey!
      this.ownerAccountString = this.wallet.publicKey!.toString()
      //console.log(this.wallet)
    },
    printConnection() {
      console.log(this.web3)
    },
    async fuckAround() {
      console.log(this.wallet.publicKey!.toString())
      const message = "Please sign this message for proof of address ownership.";
      const data = new TextEncoder().encode(message);
      let trs = await this.wallet.sign(data, 'utf8')
      console.log(trs)
    },
    async findAssociatedAccount(){
      const GREETING_SEED = 'hello';
      const greetedPubkey = await PublicKey.createWithSeed(
          this.wallet.publicKey!,
          GREETING_SEED,
          this.programKey,
      );
      console.log(greetedPubkey.toString())

    },
    async derivePDA() {
      await this.web3.getProgramAccounts(this.programKey).then(result => {
        result.forEach(c => {console.log(c.pubkey.toString())})
      })
    },
    async readPubkey(){
      const accountJInfo = await this.web3.getAccountInfo(this.derivedSubAccount).then(result => {return result})
      console.log(accountJInfo)

      const greeting = borsh.deserialize(
          GreetingSchema,
          GreetingAccount,
          accountJInfo!.data,
      )
      console.log(greeting)
      this.subUntil = greeting
    },
    async saySub() {
      console.log('Subscribing.')
      const instruction = new TransactionInstruction({
        keys: [{pubkey: this.derivedSubAccount, isSigner: false, isWritable: true}],
        programId: this.programKey,
        data: Buffer.alloc(0), // All instructions are hellos
      });
      console.log(instruction)
      /*await sendAndConfirmTransaction(
          this.web3,
          new Transaction().add(instruction),
          [this.wallet],
      );*/
      let transaction = new Transaction().add(instruction)
      let { blockhash } = await this.web3.getRecentBlockhash();
      transaction.recentBlockhash = blockhash;
      transaction.feePayer = this.wallet.publicKey!;
      console.log(transaction)

      const signed = await this.wallet.signTransaction(transaction)
      console.log(signed)
      const txid = await this.web3.sendRawTransaction(signed.serialize())
      console.log(txid);

      await this.web3.confirmTransaction(txid);
    },
    viewPrograms() {
      console.log(PROGRAMS)
    }
  }
});
</script>
<style>

</style>
