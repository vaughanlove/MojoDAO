<template>
  <div class="app">
    <img class="logo-img" src="@/assets/subSite-minimal-logo.svg"/>
    <h1 class="intro">MojoDAO</h1>

    <div id="nav" class="navbar">
      <router-link :class="{ 'highlighted-divider': activeClass === 'about' }"
                   @click="activeClass = 'about'"
                    class="divider" tag="button" to="/about">About</router-link>
      <router-link :class="{ 'highlighted-divider': activeClass === 'client' }"
                   @click="activeClass = 'client'"
                   class="divider" tag="button" to="/client">Client</router-link>
      <router-link :class="{ 'highlighted-divider': activeClass === 'user' }"
                   @click="activeClass = 'user'"
                   class="divider" tag="button" to="/user">User</router-link>
      <router-link :class="{ 'highlighted-divider': activeClass === 'demo' }"
                   @click="activeClass = 'demo'"
                   class="divider" tag="button" to="/demo">Demo</router-link>
    </div>
    <router-view />
  </div>
</template>

<script lang="ts">
import {defineComponent} from "vue";


export default defineComponent({
  name: 'App',
  components: {},
  data() {
    return {
      isActive: false,
      activeClass: ''
    }
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
  transform: rotate(-10deg);
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
