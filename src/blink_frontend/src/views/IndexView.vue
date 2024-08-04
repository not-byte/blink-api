<script lang="ts" setup>
import LogoIcon from "@/components/icon/LogoIcon.vue";
import { useAuthStore } from "@/stores/auth";
import { computed, Ref, ref } from "vue";
import { useRouter } from "vue-router";

const auth = useAuthStore();
const username: Ref<string> = ref("");
const avatar: Ref<string> = ref("");
const router = useRouter();

const isAnon = computed(() => auth.isAnonymous);
const hasAccount: Ref<boolean> = ref(false);

async function createUser() {
  try {
    if (username.value === "") {
      throw new Error("Can't create user with empty name");
    }

    if (avatar.value === "") {
      await auth.addUser(username.value);
    } else {
      await auth.addUser(username.value, avatar.value);
    }
    await router.push("/messages");
  } catch (e) {
    console.groupCollapsed("Error in creating a user");
    console.error(e);
    if (e.message === "User already exists") {
      hasAccount.value = true;
    }
    console.groupEnd();
  }
}

auth.$subscribe(async () => {
  let user = await auth.getUser;
  console.warn(user);
  if (user !== undefined) {
    hasAccount.value = true;
  }
})
</script>

<template>
  <section class="w-full h-full flex flex-col gap-6 transition-root">
    <section class="w-full h-full flex-grow flex flex-col items-center justify-center gap-3 text-center">
      <LogoIcon class="w-[36rem] mb-[4rem] lg:w-80 lg:mb-16" />
      <h3 class="text-[8rem] lg:text-4xl font-semibold">BLINK</h3>
      <p class="text-[2rem] lg:text-base">Decentralized Communicator</p>
    </section>
    <div class="flex flex-col gap-6 transition-root relative">
      <template v-if="!hasAccount">
        <aside class="w-full h-fit px-5 py-3 bg-white/5 backdrop-blur-sm border border-white/10 rounded-2xl">
          <input class="w-full bg-transparent outline-none placeholder:text-white" placeholder="Username" type="text"
            v-model="username" />
        </aside>
        <aside class="w-full h-fit px-5 py-3 bg-white/5 backdrop-blur-sm border border-white/10 rounded-2xl">
          <input class="w-full bg-transparent outline-none placeholder:text-white"
            placeholder="Link to avatar (Optional)" type="text" v-model="avatar" />
        </aside>

        <button
          class="w-full h-fit px-5 py-3 bg-white/5 backdrop-blur-sm border border-white/10 rounded-2xl text-center"
          @click="createUser">
          Join now
        </button>
      </template>
      <template v-else>
        <router-link
          class="text-[2rem] lg:text-base w-full h-fit px-9 py-6 lg:px-5 lg:py-3 bg-smoke/5 backdrop-blur-sm border-2 lg:border border-smoke/10 rounded-2xl text-center"
          to="/messages">
          Continue
        </router-link>
      </template>

      <template v-if="isAnon">
        <div class="backdrop-blur-[2px] w-full h-full absolute rounded-xl">
          <h1 class="fixed -top-10 left-1/2 -translate-x-1/2 -translate-y-1/2 text-2xl whitespace-nowrap">Waiting for
            login...</h1>
        </div>
      </template>
    </div>
  </section>
</template>
