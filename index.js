// For more comments about what's going on here, check out the `hello_world`
// example.
// import('./pkg/sir').catch(console.error);
// import * as sir from "./pkg/sir";
import { Virus, World, PopulationDistribution, Stats } from './rust/pkg/sir';

const virus = Virus.corona();
console.log(virus)
const distribution = PopulationDistribution.Random;
console.log(distribution)
const world = World.new(5000, 1280, 720, virus, distribution);
console.log(world);
let stats = null;
update();

function update() {
  world.update();
  world.render('canvas');
  stats = world.get_stats();
  console.log('Stats ', stats.susceptable, stats.infected, stats.recovered);
  setTimeout(update, 1);
}