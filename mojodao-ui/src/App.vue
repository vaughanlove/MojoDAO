<template>
  <div class="app">
    <img class="logo-img" src="@/assets/subSite-minimal-logo.svg"/>
    <h1 class="intro">Mojo</h1>

    <div id="nav" class="navbar">
      <router-link :class="{ 'highlighted-divider': activeClass === 'about' }"
                   @click="activeClass = 'about'"
                    class="divider" tag="button" to="/about">About</router-link>
      <router-link :class="{ 'highlighted-divider': activeClass === 'client' }"
                   @click="activeClass = 'client'"
                   class="divider" tag="button" to="/client">Analytics</router-link>
      <router-link :class="{ 'highlighted-divider': activeClass === 'demo' }"
                   @click="activeClass = 'demo'"
                   class="divider" tag="button" to="/demo">Demo</router-link>
      <router-link :class="{ 'highlighted-divider': activeClass === 'token' }"
                   @click="activeClass = 'token'"
                   class="divider" tag="button" to="/token">$Mojo</router-link>
    </div>
    <button v-if="!isConnected" class="connect" @click="connecter()">connect</button>
    <button v-if="isConnected" class="connected" disabled @click="connecter()">connected</button>


    <router-view v-bind:wall="wall" v-bind:walletKey="walletKey" v-bind:isConnected="isConnected" v-bind:programs="programs"/>


  </div>
</template>

<script lang="ts">
import {defineComponent, ref} from "vue";
import SolanaWalletAdapter from "@project-serum/sol-wallet-adapter";
import {PublicKey, Transaction, TransactionInstruction} from "@solana/web3.js";
import {ProgramInfo, PROGRAMS} from "@/utils/subscription-list";
import {createConnection} from "@/plugins/solana3";
import Wallet from "@project-serum/sol-wallet-adapter";
import {getDurationFromKey} from "@/utils/transactions";


export default defineComponent({
  name: 'App',
  components: {},
  data() {
    return {
      isActive: false,
      activeClass: '',
      web3: createConnection('https://api.devnet.solana.com')
    }
  },
  setup() {
    const programs = ref<ProgramInfo[]>(PROGRAMS)
    const wall =  new Wallet('https://www.sollet.io', "https://api.devnet.solana.com")
    const walletKey = ref<PublicKey>(new PublicKey('11111111111111111111111111111111'))
    const isConnected = ref<boolean>(false)
    return {programs, wall, walletKey, isConnected}
  },
  methods: {
    async connecter(){
      await this.wall.connect()
      if (this.wall.connected) {
        console.log(this.wall.publicKey?.toBase58())
        this.walletKey = this.wall.publicKey!
        this.isConnected = true
        await this.populateUserInfo()
      }
      else {
        console.log('wallet connection unsuccessful')
      }
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
<style>
@import url(https://fonts.googleapis.com/css?family=Satisfy);
.logo-img {
  max-width: 100%;
  margin:  auto;
  display: block;
}
.navbar {
  padding: 20px;
  text-align: center;
}
a:visited {
  color: darkred;
}
a:link {
  color: darkred;
}
a:hover {
  color: red;
}
.intro {
  margin-top: -125px;
  color: rgb(26, 25, 25);
  display: block;
  font-family: 'Satisfy', cursive;
  font-size: 64px;
  text-shadow: 1px 1px black;
  transform: rotate(0deg);
}
.divider {
  position: relative;
  text-decoration: none;
  font-weight: 0;
  font-size: xx-large;
  margin-right: 50px;
}
.highlighted-divider {
  text-decoration: underline;
  position: relative;
  font-weight: 1000;
  font-size: xx-large;
  margin-right: 50px;
}

</style>
