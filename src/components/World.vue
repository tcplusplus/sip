<template>
  <div class="hello">
    <canvas id="canvas" width="1280" height="720" /> {{ fps }} fps
    <canvas ref="chart" width="1280" height="200"></canvas>
  </div>
</template>

<script lang="ts">
import Vue from 'vue';
import { Virus, World, PopulationDistribution, Stats } from '@/rust/pkg/sir';
import Chart from 'chart.js';

export default Vue.extend({
  name: 'World',
  data: () => ({
    world: null as World | null,
    data: {} as any,
    options: {} as any,
    chart: null as null | Chart,
    fps: 12,
  }),
  mounted() {
    const virus = Virus.corona();
    const distribution = PopulationDistribution.Random;
    this.world = World.new(5000, 1280, 720, virus, distribution);
    this.data = {
      labels: [],
      datasets: [
        {
          data: [],
          label: 'Susceptible',
          borderColor: '#0f0',
          fill: false,
        },
        {
          data: [],
          label: 'Infected',
          borderColor: '#f00',
          fill: false,
        },
        {
          data: [],
          label: 'Recovered',
          borderColor: '#00f',
          fill: false,
        },
      ],
      options: {
        title: {
          display: true,
          text: 'Percentage of population',
        },
      },
    };
    for (let i = 0; i < 1280; ++i) {
      this.data.labels.push(this.data.labels.length);
    }
    this.chart = new Chart(
      this.$refs.chart as HTMLCanvasElement,
      {
        type: 'line',
        data: this.data,
      },
    );
    this.update();
  },
  methods: {
    update() {
      if (this.world === null || this.chart === null) {
        return;
      }
      const now = (new Date()).getTime();
      this.world.update();
      this.world.render('canvas');
      const stats = this.world.get_stats();
      this.data.datasets[0].data.push(stats.susceptable * 100);
      this.data.datasets[1].data.push(stats.infected * 100);
      this.data.datasets[2].data.push(stats.recovered * 100);
      this.chart.update();
      const after = (new Date()).getTime();
      this.fps = Math.floor(1000 / (after - now));
      //
      setTimeout(this.update, 0);
    },
  },
});
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
h3 {
  margin: 40px 0 0;
}
ul {
  list-style-type: none;
  padding: 0;
}
li {
  display: inline-block;
  margin: 0 10px;
}
a {
  color: #42b983;
}
</style>
