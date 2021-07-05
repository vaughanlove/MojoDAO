<template>
  <div class="program-list" v-for="program in programs" :key="program.id">
    <img :src="program.symbol.replace(' ', '-').toLowerCase()">
    {{ program.name }}, cost: ${{program.cost}}, {{ program.subscribedUntil == undefined ? ' user is not subscribed' : 'user is subscribed until ' + program.subscribedUntil + ' hours'}}.
    <button @click="saySub(program.id ,wallet, wallet.publicKey)">subscribe</button>
  </div>
</template>

<script lang="ts">
import {defineComponent, PropType} from "vue";
import { ProgramInfo } from "@/utils/subscription-list";
import Wallet from "@project-serum/sol-wallet-adapter";
import {Connection, PublicKey, Transaction, TransactionInstruction} from "@solana/web3.js";
import {findAssociatedAccount, getDurationFromKey} from "@/utils/transactions";

export default defineComponent({
  name: "ProgramList",
  components: {},
  props: {
    programs: {
      required: true,
      type: Array as PropType<ProgramInfo[]>
    },
    wallet: {
      required: true,
      type: Object as PropType<Wallet | undefined>
    },
    web3: {
      required: true,
      type: Object as PropType<Connection | undefined>
    }
  },
  methods: {
    async saySub(i: number, walletT: Wallet, walletAddress: PublicKey) {
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
      let { blockhash } = await this.web3!.getRecentBlockhash();
      transaction.recentBlockhash = blockhash;
      transaction.feePayer = walletAddress;
      console.log(transaction)

      const signed = await walletT.signTransaction(transaction)
      console.log(signed)
      const txid = await this.web3!.sendRawTransaction(signed.serialize())
      console.log(txid);

      // eslint-disable-next-line vue/no-mutating-props
      this.programs[i].subscribedUntil = await getDurationFromKey(this.web3!, this.programs[i].programKey, this.wallet!).then(e => {return e.counter})

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