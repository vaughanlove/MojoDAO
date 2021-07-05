<template>
  <button @click="onHit()">subscribe</button>
</template>

<script lang="ts">
import { defineComponent, PropType} from "vue";
import Wallet from "@project-serum/sol-wallet-adapter";

export default defineComponent({
  name: "SubscribeButton",
  props: {
    wallet: {
      required: true,
      type: Object as PropType<Wallet | undefined>
    }
  },
  methods: {

  }
})
</script>

<style scoped>

</style>