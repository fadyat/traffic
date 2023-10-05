The script, will fetch GitHub repository traffic data and store it in some files.

Every launch needs to be correctly merged with previous data (if any).
If in a previous run, there was something at the boundary of the fetch request,
it can be overwritten with new data, when he's not on the border.

Current traffic API: https://docs.github.com/en/rest/metrics/traffic?apiVersion=2022-11-28

`export $(cat .env | xargs) && envsubst < .config/config_template.yaml > .config/config.yaml`
