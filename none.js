import http from 'k6/http';
import { check } from 'k6';
import { randomFight } from './k6/randomFight.js';

export const options = {
  scenarios: {
    low_load: {
      executor: 'constant-arrival-rate',
      duration: '50s',
      rate: 1,
      timeUnit: '1h',
      preAllocatedVUs: 20,
      maxVUs: 200,
    },
  },
};

export default () => {
  var fight_result = randomFight();
  console.log(fight_result.winnerName);
}
