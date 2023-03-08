<!-- 
If you want to attach some badges, comment them out. You need to adapt the parts in [[ ]])

[![Crates.io](https://img.shields.io/crates/v/bandwidth-monitor?label=bandwidth-monitor)](https://crates.io/crates/bandwidth-monitor)
[![Crates.io](https://img.shields.io/crates/v/bandwidth-monitor-lib?label=bandwidth-monitor-lib)](https://crates.io/crates/bandwidth-monitor-lib)
[![GitHub](https://img.shields.io/github/license/[[user]]/bandwidth-monitor)](https://github.com/[[user]]/bandwidth-monitor/blob/main/LICENSE)
![test](https://github.com/[[user]]/bandwidth-monitor/workflows/test/badge.svg)

-->

# bandwidth-monitor

This uses Ookla to run speed tests and store the results in a google sheet

## Install

You need to install [speedtest-cli from ookla](https://www.speedtest.net/de/apps/cli) first. You also 
need to run it once and accept the eula manually.

To get the latest release you can run
```bash
curl -OL https://github.com/Semptic/bandwidth-monitor/releases/latest/download/bandwidth-monitor
```

## Google Credentials

To get the credentials to access google sheets best you follow this [setup](https://developers.google.com/sheets/api/quickstart/python#set_up_your_environment) guide (ignore everything except `Enable the API` and `Authorize credentials for a desktop application`). By default bandwidht-monitor is using `client_secret.json`, but this can be changed.

## Usage

You can run it like
```bash
./bandwith-monitor <spreadsheet_id>
```

You get your sheet id from the URL: `https://docs.google.com/spreadsheets/d/1WOIazmVG9vr2-GcdLi6Yz4sCHjAkgnlqSTvetiUM1oE/edit#gid=1042246224`. The id is the part after `/d/` so in this example it would be `1WOIIzmVG9vr2-GcdLi6Yz4sCHjAkgnlqSTvetiUM1iE`.

## Development

### Prerequisite

* Install [cargo-make](https://github.com/sagiegurari/cargo-make). 

### Test

To test this project run

```bash
cargo make test
```