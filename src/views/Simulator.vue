<template>
  <div class="home">
    <World />
  </div>
</template>

<script lang="ts">
import Vue from 'vue';
import World from '@/components/World.vue';

export default Vue.extend({
  name: 'Simulator',
  components: {
    World,
  },
});
</script>
