<template>
  <div>
    <div class="program-list" v-for="program in progs" :key="program.id">
      {{ program.name }}, cost: ${{program.cost}}, {{ program.userInfo }}.
      <button @click="sub(program.id)">sub</button>
    </div>
    <div class="box">
      <button > try new sub </button>
    </div>
  </div>
</template>

<script lang="ts">
import {defineComponent, PropType, ref} from "vue";
import {createConnection} from "@/plugins/solana3";
import {Connection, Keypair, PublicKey, Transaction, TransactionInstruction} from "@solana/web3.js";
import {ProgramInfo} from "@/utils/subscription-list";
import Wallet from "@project-serum/sol-wallet-adapter";
import {findAssociatedAccount, getDurationFromKey} from "@/utils/transactions";
import {SystemProgram} from "@solana/web3.js";
import {GreetingAccount, GreetingSchema} from "@/plugins/solana3";
import * as borsh from "borsh";
import {Account} from "@solana/web3.js";
import { createToast } from 'mosha-vue-toastify';
import 'mosha-vue-toastify/dist/style.css'


export default defineComponent({
  name: "ProgramList",
  components: {},
  data() {
    return {
      GREETING_SEED: 'spotifysub',
      web3: createConnection('https://api.devnet.solana.com')
    }
  },
  setup() {
    const walletAddress = ref<PublicKey>()
    return{walletAddress}
  },
  props: {
    walletKey: {
      required: true,
      type: PublicKey
    },
    progs: {
      required: true,
      type: Array as PropType<ProgramInfo[]>
    },
    wall: {
      required: true,
      type: Wallet
    },
    programs: {
      required: true,
      type: Array as PropType<ProgramInfo[]>
    },

  },
  created() {
    this.logger()
    this.walletAddress = this.walletKey;
  },

  methods: {
    logger() {
      console.log(this.walletKey.toString())
    },
    async sub(i: number) {
      try {
        await this.paySub(i)
      } catch (e) {
        createToast('tx failed.', {type: 'danger', position: 'bottom-left'})
      }
    },
    async paySub(i: number) {
      // log the program address
      const programId = this.programs[i].programKey
      console.log('Subscribing to ' + this.programs[i].programKey.toString())

      // find the associated account to, this is definitely a shit way to do this.
      const GREETING_SEED = 'spotifysub';

      const greetingSeed = await PublicKey.createWithSeed(
          this.walletKey,
          GREETING_SEED,
          this.programs[i].programKey,
      )
      console.log(greetingSeed)

      const greetedAccount = await this.web3.getAccountInfo(greetingSeed);

      console.log(greetedAccount)

      // MAKE GREETING_SIZE
      const GREETING_SIZE = borsh.serialize(
          GreetingSchema,
          new GreetingAccount(),
      ).length;

      // lamports
      console.log('the wallet key: ' + this.walletKey)

      const lamports = await this.web3.getMinimumBalanceForRentExemption(GREETING_SIZE);

      console.log('lamports:' + lamports)

      if(greetedAccount == null) {
        const gtransaction = new Transaction().add(
            SystemProgram.createAccountWithSeed({
              fromPubkey: this.walletKey,
              basePubkey: this.walletKey,
              seed: GREETING_SEED,
              newAccountPubkey: greetingSeed,
              lamports,
              space: GREETING_SIZE,
              programId,
            }),
        );
        let { blockhash } = await this.web3.getRecentBlockhash();
        gtransaction.recentBlockhash = blockhash;
        gtransaction.feePayer = this.walletKey;

        console.log('creating new sub account')
        const gsigned = await this.wall.signTransaction(gtransaction)

        const txid = await this.web3.sendRawTransaction(gsigned?.serialize())
        console.log(txid);

        await this.web3.confirmTransaction(txid);
        await this.populateUserInfo()
      }
      // make the new transaction that we want to send to the programID
      const instruction = new TransactionInstruction({
        keys: [{pubkey: greetingSeed, isSigner: false, isWritable: true}],
        programId: this.programs[i].programKey,
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
      createToast("Success. TXID: " + txid, {type: 'success', position: 'bottom-left'})

      await this.populateUserInfo()
    },
    async populateUserInfo() {
      this.programs.forEach(e => {
        getDurationFromKey(this.web3, e.programKey, this.walletKey).then(n => { e.userInfo = n})
      })
      console.log(this.programs)
    },
  }
})
</script>

<style scoped>
.program-list {
  width: 40%;
  margin: auto;
  text-align: center;
  background: lightcoral;
  padding: 12px;
  margin-bottom: 15px;
}
.program-list img {
  height: 30px;
  size: 45px;
}
</style>