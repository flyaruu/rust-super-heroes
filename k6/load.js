import http from 'k6/http';
import { check } from 'k6';
import { randomFight } from './randomFight.js';

export const options = {
  thresholds: {
    // http_req_duration: ['p(95)<500'],  // 95% of requests should be <50ms
    http_req_failed: ['rate<0.001'],    // error rate should be <0.1%
    // dropped_iterations: ['count == 0'],  // no dropped iterations allowed
  },
  scenarios: {
    ramp_high_load: {
      executor: 'ramping-arrival-rate',
      startRate: 10,
      timeUnit: '1s',
      preAllocatedVUs: 20,
      maxVUs: 200,
      stages: [
        { target: __ENV.RAMPING_RATE ? parseInt(__ENV.RAMPING_RATE) : 500, duration: '5s' },
        { target: __ENV.RAMPING_RATE ? parseInt(__ENV.RAMPING_RATE) : 500, duration: '20s' },
        { target: 0, duration: '5s' },
      ]
    },
  },
};

export default () => {
  var fight_result = randomFight();
}
