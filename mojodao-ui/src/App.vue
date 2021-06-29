<template>
  <h1> MojoDAO Ui V1</h1>
  <SubBox/>
</template>

<script>

import SubBox from './components/SubBox.vue'

export default {
  name: 'App',
  components: {
    SubBox,
  }
}
</script>

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin-top: 60px;
}
</style>
