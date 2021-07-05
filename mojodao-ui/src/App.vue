<template>
  <div class="app">
    <h1>MojoDAO</h1>
    <div id="nav">
      <router-link to="/client">Client</router-link> |
      <router-link to="/about">About</router-link>   |
      <router-link to="/user">User</router-link>     |
      <router-link to="/demo">Demo</router-link>
    </div>
    <router-view />
  </div>
</template>

<script lang="ts">
import {defineComponent} from "vue";

export default defineComponent({
  name: 'App',
  components: {},
})
</script>
<style>
</style>
