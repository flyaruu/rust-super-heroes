import http from 'k6/http';

export const options = {
    vus: 100,
    iterations: 100000
};

export default() => {

    const json_post_header = {
        headers: {
          'Content-Type': 'application/json',
        },
    };
    
    var response_body = JSON.parse(http.get("http://localhost:8082/api/fights/randomfighters").body);
    var hero = response_body.hero;
    var villain = response_body.villain;
    var location = JSON.parse(http.get("http://localhost:8082/api/fights/randomlocation").body);
    // console.log("Loqqq: "+ JSON.stringify(location));
    var fight_request = { hero: hero, villain: villain, location: location};
    // console.log("Reqqq: "+ JSON.stringify(fight_request));
    var response = http.post("http://localhost:8082/api/fights", JSON.stringify(fight_request), json_post_header).body;
    // console.log("Unparsed: +"+JSON.stringify(response));
    var fight_result = JSON.parse(response);
    // console.log(fight_result);
}
