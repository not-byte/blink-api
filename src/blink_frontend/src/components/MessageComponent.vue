<script lang="ts" setup>
import { getTime } from "@/utils/util";
const props = defineProps<{
  sender: string;
  message: string;
  timestamp: number;
}>();
</script>

<template>
  <aside class="w-full h-fit flex flex-col items-end gap-6 lg:gap-2">
    <template v-if="props.sender === `You`">
      <p class="w-full flex justify-end gap-3 text-[2rem] lg:text-base text-lavender">
        <span class="font-semibold">{{ props.sender }}</span>
        <span>{{ getTime(props.timestamp) }}</span>
      </p>
      <aside class="w-full flex justify-end">
        <p
          class="w-fit px-9 py-6 lg:px-5 lg:py-3 rounded-2xl bg-smoke/5 backdrop-blur-sm border-2 lg:border border-smoke/10 text-[2rem] lg:text-base">
          {{ message }}
        </p>
      </aside>
    </template>
    <template v-else>
      <p class="w-full flex justify-start gap-3 text-[2rem] lg:text-base text-lavender">
        <span class="font-semibold">{{ props.sender }}</span>
        <span>{{ getTime(props.timestamp) }}</span>
      </p>
      <aside class="w-full flex justify-start">
        <p
          class="w-fit px-9 py-6 lg:px-5 lg:py-3 rounded-2xl bg-smoke/5 backdrop-blur-sm border-2 lg:border border-smoke/10 text-[2rem] lg:text-base">
          {{ message }}
        </p>
      </aside>
    </template>
  </aside>
</template>
