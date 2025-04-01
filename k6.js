import http from 'k6/http';
import { check } from 'k6';
import { randomFight } from './k6/randomFight.js';

// const PerformanceTestConfig = {
//   fixedWorkload: [{
//     vus: 100,
//     iterations: 100000
//   }],
//   constantWorkload:  [
//       { duration: '10s', target: 100 }, // ramp up to 400 users
//       { duration: '2m', target: 100 }, // stay at 400 for ~4 hours
//       { duration: '10s', target: 0 }, // scale down. (optional)
//   ],
//   lowWorkloadWithSpike: [
//     { duration: '2m', target: 2 }, // low load for a few minutes
//     { duration: '2s', target: 200 }, // rapidly ramp up
//     { duration: '30s', target: 200 }, // high load for 30s
//     { duration: '1s', target: 1 }, // quickly ramp down
//     { duration: '2m', target: 1 }, // take it easy for a few minutes
//   ],
//   smoke: [{
//     vus: 1,
//     iterations: 10
//   }]
// }

// const stages = PerformanceTestConfig[__ENV.WORKLOAD] || PerformanceTestConfig['smoke'];

// console.log("Config: "+JSON.stringify(stages));
export const options = {
  scenarios: {
    max: {
      executor: 'shared-iterations',
      vus: 200,
      iterations: 100000
    },
  },
};

export default () => {
  var fight_result = randomFight();
  console.log(fight_result.winnerName);
}
