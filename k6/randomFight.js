import http from 'k6/http';
import { check } from 'k6';

export function randomFight() {
  const json_post_header = {
    headers: {
      'Content-Type': 'application/json',
    },
  };
  var response = http.get("http://localhost:8082/api/fights/randomfighters");
  check(response, {
    'random fighters status is 200': (r) => r.status === 200,
  });
  var response_body = JSON.parse(response.body);

  var hero = response_body.hero;
  var villain = response_body.villain;
  check(location, {
    'hero is not fallback': (r) => !hero.name.toLowerCase().includes("fallback"),
    'villain is not fallback': (r) => !villain.name.toLowerCase().includes("fallback")
  })

  var location_response = http.get("http://localhost:8082/api/fights/randomlocation");
  check(response, {
    'location status is 200': (r) => r.status === 200,
  });
  var location = JSON.parse(location_response.body);
  check(location, {
    'location is not fallback': (r) => !location.name.toLowerCase().includes("fallback")
  })
  var fight_request = { hero: hero, villain: villain, location: location };
  var fight_response = http.post("http://localhost:8082/api/fights", JSON.stringify(fight_request), json_post_header);
  // console.log(fight_response);
  check(fight_response, {
    'fight result is 200': (r) => r.status === 200
  })
  var fight_result = JSON.parse(fight_response.body);
  // console.log("===============");

  // console.log(fight_result);

  return fight_result
}
