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

You need to install [speedtest-cli from ookla](https://www.speedtest.net/de/apps/cli) first. 

## Development

### Prerequisite

* Install [cargo-make](https://github.com/sagiegurari/cargo-make). 

### Test

To test this project run

```bash
cargo make test
```