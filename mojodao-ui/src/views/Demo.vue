<template>
  <div>
    <div class="box">
      <div>
        <h1 class="titlehead">
          basic user view

        </h1>
      </div>

     <div v-if="!isConnected"  class="panel-text">
       please connect your wallet
     </div>
      <div v-if="isConnected" class="panel-text">
        address: {{ walletKey?.toString() }}
        <div class="panel-text">
          <p>Time: <strong>{{Math.round(Date.now() / 3600000)}}</strong></p>
        </div>
      </div>

    </div>

    <div v-if="isConnected">{{ getWalletString }}</div>
    <ProgramList v-if="isConnected" v-bind:programs="programs" v-bind:walletKey="walletKey" v-bind:progs="programs" v-bind:wall="wall"></ProgramList>
  </div>

</template>

<script lang="ts">
import {defineComponent, PropType, ref} from "vue";
import ProgramList from "@/components/ProgramList.vue";
import Wallet from "@project-serum/sol-wallet-adapter";
import {createConnection} from "@/plugins/solana3";
import {ProgramInfo} from "@/utils/subscription-list";
import {PublicKey} from "@solana/web3.js";

export default defineComponent({
  name: "Demo",
  data() {
    return{
      web3: createConnection('https://api.devnet.solana.com'),
    }
  },
  props: {
    wall: {
      required: true,
      type: Wallet
    },
    walletKey: {
      required:true,
      type: PublicKey
    },
    isConnected: {
      required:true,
    },
    programs: {
      required: true,
      type: Array as PropType<ProgramInfo[]>
    },
  },
  components: {ProgramList},
});
</script>

<style scoped>

</style>