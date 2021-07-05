<template>
  <div>
  <h1 class="titlehead">mojodao ui</h1>


  <div class="">
    <p>the current time: <strong>{{Math.round(Date.now() / 3600000)}}</strong></p>
  </div>


  <ProgramList></ProgramList>
  </div>
</template>

<script lang="ts">
import {createConnection, GreetingAccount, GreetingSchema} from "@/plugins/solana3";
import {PublicKey} from "@solana/web3.js";
import SolanaWalletAdapter from "@project-serum/sol-wallet-adapter";
import {ref} from "vue";
import {PROGRAMS} from "@/utils/subscription-list";
import {derivePDAS} from "@/utils/transactions";
import * as borsh from "borsh";
import {defineComponent} from "vue";
import ProgramList from "@/components/ProgramList.vue";
import Wallet from "@project-serum/sol-wallet-adapter";

export default defineComponent({
  name: "Demo",
  components: {ProgramList},
  data() {
    return {
      lister: PROGRAMS,
      subUntil: 0,
    }
  },

  setup(){
    // this derived sub account should be calculated on wallet connection
    const derivedSubAccount = new PublicKey('5wenbx6kdzuhNykrvrHTFdiAhRRwPg1XrnQENerF6zVz')

    // maybe should be just type? Not sure. Test more.

    // I really like how clean and organized this is. ++Has built in reactivity.

    const greeting = ref<string>('')
    return {derivedSubAccount, greeting}
  },

  methods: {
    viewPrograms() {
      this.lister.forEach(e => {console.log(e)})
    },
  }
});
</script>

<style scoped>

</style>