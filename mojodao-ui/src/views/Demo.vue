<template>
  <div>
    <div class="box">
      <div>
        <h1 class="titlehead">
          basic user view

        </h1>
      </div>

      <div class="panel-text">
        <p>Time: <strong>{{Math.round(Date.now() / 3600000)}}</strong></p>
      </div>
      <div v-if="isConnected" class="panel-text"> address: {{ walletKey?.toString() }}</div>

    </div>

    <div v-if="isConnected">{{ getWalletString }}</div>
    <ProgramList v-if="isConnected" v-bind:programs="programs" v-bind:walletKey="walletKey" v-bind:progs="programs" v-bind:wall="wall"></ProgramList>
    <div class="box">.</div>
    <div class="box">.</div>
  </div>

</template>

<script lang="ts">
import {defineComponent, PropType, ref} from "vue";
import ProgramList from "@/components/ProgramList.vue";
import {PROGRAMS} from "@/utils/subscription-list";
import SolanaWalletAdapter from "@project-serum/sol-wallet-adapter";
import Wallet from "@project-serum/sol-wallet-adapter";
import {Connection, PublicKey, Transaction, TransactionInstruction} from "@solana/web3.js";
import {createConnection} from "@/plugins/solana3";
import {findAssociatedAccount, getDurationFromKey} from "@/utils/transactions";
import {ProgramInfo} from "@/utils/subscription-list";
import {Keypair} from "@solana/web3.js";

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

  methods: {
    async paySub() {
      // log the program address
      console.log('Subscribing to ' + this.programs[0].programKey.toString())

      // find the associated account to, this is definitely a shit way to do this.
      const GREETING_SEED = 'spotifysub';

      const greetingSeed = await PublicKey.createWithSeed(
          this.walletKey,
          GREETING_SEED,
          this.programs[0].programKey,
      )
      console.log(greetingSeed)

      // make the new transaction that we want to send to the programID
      const instruction = new TransactionInstruction({
        keys: [{pubkey: greetingSeed, isSigner: false, isWritable: true}],
        programId: this.programs[0].programKey,
        data: Buffer.alloc(0), // All instructions are hellos
      });

      // log that instruction
      console.log(instruction)

      // make a new transaction and add the instruction
      let transaction = new Transaction().add(instruction)

      console.log('here')

      // whatever this is, its needed
      let { blockhash } = await this.web3.getRecentBlockhash();
      transaction.recentBlockhash = blockhash;
      transaction.feePayer = this.walletKey;


      console.log(transaction)
      console.log('about to fail')

      let signed = await this.wall.signTransaction(transaction);
      console.log(signed)

      const txid = await this.web3.sendRawTransaction(signed?.serialize())
      console.log(txid);

      await this.web3.confirmTransaction(txid);
    },
  }
});
</script>

<style scoped>

</style>