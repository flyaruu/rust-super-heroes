import http from 'k6/http';
import { check } from 'k6';
import { randomFight } from './k6/randomFight.js';

export const options = {
  scenarios: {
    ramp_high_load: {
      executor: 'ramping-arrival-rate',
      startRate: 1,
      timeUnit: '1s',
      preAllocatedVUs: 20,
      stages: [
        { target: 300, duration: '10s' },
        { target: 300, duration: '50s' },
      ]
    },
  },
};

export default () => {
  var fight_result = randomFight();
  console.log(fight_result.winnerName);
}
