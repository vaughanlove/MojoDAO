<template>
  <div class="app">
    <h1 class="titlehead">mojodao ui</h1>
    <div class="panel-div">
      <p>Your parent (payer) account: <strong>{{ this.ownerAccountString }}</strong></p>
    </div>

    <div class="">
      <p>the current time: <strong>{{Math.round(Date.now() / 3600000)}}</strong></p>
    </div>
    <div>
      {{this.greeting}}
    </div>

    <ProgramList :programs="this.lister" :wallet="this.wallet" :web3="this.web3"/>

    <button @click="connectWallet()">connect</button>
    <div></div><div></div>
    <button @click="readKey(this.web3, this.derivedSubAccount)">read duration</button>
    <div></div><div></div>
    <button @click="updatePDAS(this.lister[0].programKey)">programs</button>
    <div></div><div></div>
    <button @click="saySub(0)">subscribe</button>
    <div></div><div></div>
    <button @click="findAndVerifySpecificPDA(0)">find specific PDA for owneracc</button>
  </div>
</template>

<script lang="ts">
import {defineComponent} from 'vue';
import {createConnection, GreetingAccount, GreetingSchema} from './plugins/solana3'
import {PublicKey, TransactionInstruction} from "@solana/web3.js";
import * as borsh from 'borsh';
import { ref } from 'vue'
import { Connection, Transaction } from '@solana/web3.js';
import { sendAndConfirmTransaction, } from "@solana/web3.js";
import Wallet from '@project-serum/sol-wallet-adapter'
import SolanaWalletAdapter from '@project-serum/sol-wallet-adapter'
import {ProgramInfo, PROGRAMS} from "@/utils/subscription-list";
import ProgramList from "@/components/ProgramList.vue";
import { getDurationFromKey, findAssociatedAccount, derivePDAS } from "@/utils/transactions";

export default defineComponent({
  name: 'App',
  components: {ProgramList},

  data() {
    return {
      web3: createConnection('https://api.devnet.solana.com'),
      subUntil: 0,
    }
  },

  setup(){
    // this derived sub account should be calculated on wallet connection
    const derivedSubAccount = new PublicKey('5wenbx6kdzuhNykrvrHTFdiAhRRwPg1XrnQENerF6zVz')

    // maybe should be just type? Not sure. Test more.
    const wallet = new SolanaWalletAdapter('https://www.sollet.io', "https://api.devnet.solana.com")

    // I really like how clean and organized this is. ++Has built in reactivity.
    const ownerAccount = ref<PublicKey>()
    const ownerAccountString = ref<string>()
    const lister = ref<ProgramInfo[]>(PROGRAMS)
    const greeting = ref<string>('')
    const GREETING_SEED = ref<string>('spotifysub')

    return {derivedSubAccount, wallet,ownerAccountString, ownerAccount, lister, greeting, GREETING_SEED}
  },

  methods: {
    async connectWallet(){
      await this.wallet.connect()
      this.ownerAccount = this.wallet.publicKey!
      this.ownerAccountString = this.wallet.publicKey!.toString()
      for (let i = 0; i < this.lister.length; i++) {
        await this.findAndVerifySpecificPDA(i)
      }
    },
    async updatePDAS() {
      //console.log(this.lister.length)
      await derivePDAS(this.lister, this.web3)
    },

    viewPrograms() {
      this.lister.forEach(e => {console.log(e)})
    },
    async findAndVerifySpecificPDA(a: number){
      const key = await PublicKey.createWithSeed(this.wallet.publicKey!, this.GREETING_SEED,this.lister[a].programKey).then(e => {return e})
      //console.log(key.toString())
      const greetedAccount = await this.web3.getAccountInfo(key);
      console.log(greetedAccount?.data)
      if (greetedAccount?.data == undefined) {
        console.log('account is not subscribed')
      } else {
        const duration = borsh.deserialize(
            GreetingSchema,
            GreetingAccount,
            greetedAccount?.data,
        )
        console.log(duration)
        this.lister[a].subscribedUntil = duration.counter
      }
    },
  }
});
</script>
<style>
</style>
