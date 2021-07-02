<template>
  <div class="app">
    <h1>welcome to the mojodao ui</h1>
    <p>Your parent (payer) account: <strong>{{parentAccount.toString()}}</strong></p>
    <p> Your subscriptions (programID)</p>
    <ul>
      <li><strong>{{programKey.toString()}}</strong> with the PDA <strong>{{derivedSubAccount.toString()}}</strong> is currently subscribed until {{subUntil}}</li>
    </ul>

    <p>the current time: <strong>{{Math.round(Date.now() / 3600000)}}</strong></p>

    <button @click="readPubkey()">test</button>
  </div>
</template>

<script lang="ts">
import {defineComponent} from 'vue';
import {createConnection, GreetingAccount, GreetingSchema} from './plugins/solana3'
import {PublicKey} from "@solana/web3.js";
import * as borsh from 'borsh';

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
    const derivedSubAccount = new PublicKey('EDTtxSugqhzghWB8aJtHoqktTfnBuTtbHFWYDGs6A8Rs')
    const programKey = new PublicKey('5wenbx6kdzuhNykrvrHTFdiAhRRwPg1XrnQENerF6zVz')
    return {parentAccount, derivedSubAccount, programKey}
  },
  mounted() {
    this.readPubkey()
  },
  methods: {
    printConnection() {
      console.log(this.web3)
    },
    async readPubkey(){
      const accountJInfo = await this.web3.getAccountInfo(new PublicKey('5wenbx6kdzuhNykrvrHTFdiAhRRwPg1XrnQENerF6zVz')).then(result => {return result})
      //const programInfo = await this.web3.getAccountInfo(programId).then(result => {return result})
      //console.log(accountJInfo!.data)
      //console.log(programInfo)

      const greeting = borsh.deserialize(
          GreetingSchema,
          GreetingAccount,
          accountJInfo!.data,
      )
      console.log(greeting.counter)
      this.subUntil = greeting.counter
    }

  }
});
</script>
<style>

</style>
