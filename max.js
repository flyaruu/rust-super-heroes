import http from 'k6/http';
import { check } from 'k6';
import { randomFight } from './k6/randomFight.js';

export const options = {
  thresholds: {
    http_req_duration: ['p(95)<50'],  // 95% of requests should be <50ms
    http_req_failed: ['rate<0.001'],    // error rate should be <0.1%
    dropped_iterations: ['count == 0'],  // no dropped iterations allowed
  },
  scenarios: {
    ramp_high_load: {
      executor: 'ramping-arrival-rate',
      startRate: 1,
      timeUnit: '1s',
      preAllocatedVUs: 20,
      stages: [
        { target: 1300, duration: '10s' },
        { target: 1300, duration: '50s' },
      ]
    },
  },
};

export default () => {
  var fight_result = randomFight();
  console.log(fight_result.winnerName);
}
