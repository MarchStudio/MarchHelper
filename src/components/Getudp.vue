<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const udpinf = ref("");

async function listen_udp() {
  udpinf.value = await invoke("listen_udp");
  console.log(udpinf)
}

</script>

<template>
  <form class="row" @submit.prevent="listen_udp">
    <button type="submit">get</button>
  </form>

  <p>{{ udpinf }}</p>
</template>
