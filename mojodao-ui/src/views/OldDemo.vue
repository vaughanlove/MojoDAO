<template>
  <div v-if="isConnected">
    <div class="program-list" v-for="program in programs" :key="program.id">
      <img :src="program.symbol.replace(' ', '-').toLowerCase()">
      {{ program.name }}, cost: ${{program.cost}}, {{ program.subscribedUntil == undefined ? ' user is not subscribed' : 'user is subscribed until ' + program.subscribedUntil + ' hours'}}.
      <button @click="saySub(program.id ,wallet, wallet.publicKey, web3)">subscribe</button>
    </div>
  </div>
</template>

<script lang="ts">
import {defineComponent, PropType, ref} from "vue";
import {PROGRAMS} from "@/utils/subscription-list";
import Wallet from "@project-serum/sol-wallet-adapter";
import {Connection, PublicKey, Transaction, TransactionInstruction} from "@solana/web3.js";
import {findAssociatedAccount, getDurationFromKey} from "@/utils/transactions";
import SolanaWalletAdapter from "@project-serum/sol-wallet-adapter";
import * as borsh from "borsh";
import {createConnection, GreetingAccount, GreetingSchema} from "@/plugins/solana3";

export default defineComponent({
  name: "ProgramList",
  components: {},
  data() {
    return {
      web3: createConnection('https://api.devnet.solana.com'),
      programs: PROGRAMS,
    }
  },
  setup(){
    const ownerAccount = ref<PublicKey>()
    const ownerAccountString = ref<string>()
    const GREETING_SEED = ref<string>('spotifysub')
    const isConnected = ref<boolean>(false)
    return {ownerAccount, ownerAccountString, GREETING_SEED, isConnected}
  },
  methods: {
    async findAndVerifySpecificPDA(a: number){
      const key = await PublicKey.createWithSeed(this.wallet.publicKey!, this.GREETING_SEED,this.programs[a].programKey).then(e => {return e})
      //console.log(key.toString())
      const greetedAccount = await this.web3!.getAccountInfo(key);
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
        this.programs[a].subscribedUntil = duration.counter
      }
    },
    async saySub(i: number, walletT: Wallet, walletAddress: PublicKey, web3: Connection) {
      console.log('Subscribing to ' + this.programs[i].programKey.toString())
      const temp = await findAssociatedAccount(this.programs[i].programKey, walletT).then(a => {return a})
      console.log(temp.toString())
      const instruction = new TransactionInstruction({
        keys: [{pubkey: temp!, isSigner: false, isWritable: true}],
        programId: this.programs[i].programKey,
        data: Buffer.alloc(0), // All instructions are hellos
      });
      console.log(instruction)

      let transaction = new Transaction().add(instruction)
      console.log('here')
      let { blockhash } = await web3.getRecentBlockhash();
      transaction.recentBlockhash = blockhash;
      transaction.feePayer = walletAddress;
      console.log(transaction)

      // fails here
      const signed = await walletT.signTransaction(transaction)
      console.log(signed)
      const txid = await web3.sendRawTransaction(signed.serialize())
      console.log(txid);

      this.programs[i].subscribedUntil = await getDurationFromKey(web3, this.programs[i].programKey, walletAddress).then(e => {return e.counter})

      await this.web3!.confirmTransaction(txid);
    },

  }
})
</script>

<style scoped>
.program-list {
  background: aquamarine;
  padding: 12px;
  margin-bottom: 15px;
}
.program-list img {
  height: 30px;
  size: 45px;
}
</style>