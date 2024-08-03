<script lang="ts" setup>
import LogoIcon from "@/components/icon/LogoIcon.vue";
import HeaderComponent from "@/components/HeaderComponent.vue";
import { useAuthStore } from "@/stores/auth";

const auth = useAuthStore();
</script>

<template>
  <section class="w-full h-full flex flex-col gap-6 transition-root">
    <section class="w-full h-full flex-grow flex flex-col items-center justify-center gap-3 text-center">
      <LogoIcon class="w-80 mb-16" />
      <HeaderComponent title="BLINK" />
      <p>Decentralized Communicator</p>
    </section>
    <template v-if="auth.isAnonymous">
      <button class="w-full h-fit px-5 py-3 bg-white/5 backdrop-blur-sm border border-white/10 rounded-2xl text-center"
        disabled>
        Waiting for login...
      </button>
    </template>
    <template v-else>
      <router-link
        class="w-full h-fit px-5 py-3 bg-white/5 backdrop-blur-sm border border-white/10 rounded-2xl text-center"
        to="/messages">
        Join now
      </router-link>
    </template>
  </section>
</template>
