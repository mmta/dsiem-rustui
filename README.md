# Dsiem Rust UI

[![CI](https://github.com/mmta/dsiem-rustui/actions/workflows/publish.yml/badge.svg)](https://github.com/mmta/dsiem-rustui/actions/workflows/publish.yml)

This is an experimental [Yew](https://yew.rs/)-based WebAssembly app for managing alarms in [Dsiem](https://github.com/defenxor/dsiem). The aim is to replace the existing Angular [web UI](https://github.com/defenxor/dsiem/blob/master/docs/web_interfaces.md#dsiem-web-ui) with something that will have fewer dependabot warnings over time.

This app has fewer functionalities (and much smaller download) than the existing web UI. The only available functions are:
- Single alarm view: linking alarm with its associated events in one page.
- Alarm update: `tag` and `status` update through drop-down menu.
- Alarm delete: remove an alarm and all of its entries in `siem_alarm_events-*`.

That said, this tool is fully usable for those doing most of their analysis in Kibana/Opensearch Dashboard, and only pivoting to Dsiem web UI for deleting alarms, or changing their `tag` and `status`.

Example screenshot:

![Screenshot](./screenshot.png)

## Setup & Integration

- Setup [Rust](https://www.rust-lang.org/tools/install) development environment on Linux.
- Install minimal [NodeJs](https://nodejs.org/en/download/) tools too, they'll be used to get CSS files.
- Install `trunk`:
    ```shell
    $ cargo install --locked trunk
    ```
- build `dsiem-rustui`, a container image based on `defenxor/dsiem` with updated UI:
    ```shell
    $ ./scripts/build.sh prod
    ```
- In your Dsiem deployment, replace `defenxor/dsiem` image with `dsiem-rustui`.
- Update `Dsiem Link` scripted fields in Kibana to replace `#/data/alarm-detail/` with `/?`, so that the URL template becomes something like this: `http://localhost:8080/ui/?{{value}}`. Example of how to do this through console in Dsiem repo:

    ```shell
    $ sed 's/#\/data\/alarm-detail\//?/' ./deployments/kibana/dashboard-siem.json > /tmp/dashboard-siem.json

    $ ./scripts/kbndashboard-import.sh localhost /tmp/dashboard-siem.json 

    ```
  After that, clicking on `Dsiem Link` for any alarm in Kibana/Opensearch Dashboard should open them on this app as shown in the example screenshot above.

## Development

First setup Dsiem demo environment:
- Clone [dsiem](https://github.com/defenxor/dsiem) main repo, and start the demo environment using `/demo/run.sh`.
- Generate alarms through **Exploit target** option on the demo env web interface.

Next update `Dsiem Link` scripted field in Kibana to use `dsiem-rustui`. Assuming that trunk will listen on port 9000, the commands will be (do these on dsiem main repo, ignore error that says siem_alarm_events.json doesnt exist):

```shell
$ sed 's/#\/data\/alarm-detail\//?/' ./deployments/kibana/dashboard-siem.json > /tmp/dashboard-siem.json
$ sed -i 's/localhost:8080/localhost:9000/' /tmp/dashboard-siem.json
$ ./scripts/kbndashboard-import.sh localhost /tmp/dashboard-siem.json 
```

Execute `trunk serve` on that port:

```shell
trunk serve --port 9000
```
Dsiem links from Kibana should now open on `dsiem-rustui`, and any code changes should trigger live reload.
