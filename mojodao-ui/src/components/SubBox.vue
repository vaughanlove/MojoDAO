<template>
  <div class="ba">
    <button v-on:click="subber()">this is a test sentence</button>
  </div>
</template>

<script>
// eslint-disable-next-line no-unused-vars
const solanaWeb3 = require('@solana/web3.js');

function subber() {
  console.log('adad');
}

export default {
  name: "SubBox",
  methods: {
    subber,
  }
}

</script>

<style scoped>
.ba {
  text-align: left;
}
</style>