# Probe

Probe is a TUI ZMQ PUB/SUB monitor and debugger.

[![Build status](https://github.com/ckoehler/probe/workflows/ci/badge.svg)](https://github.com/ckoehler/probe/actions)


![screenshot](assets/screen1.png)

# Keyboard Shortcuts

| Key       | Action                          |
| ----      | -----                           |
| q         | Quit                            |
| h, Left   | Previous Tab                    |
| l, Right  | Next Tab                        |
| j         | Next Probe                      |
| k         | Previous Probe                  |
| \<Enter\> | Show Details for selected probe |

# Configuration 

Probe looks for a `probe.toml` file to know what to do. The format is very simple, just an array of one or more `[[probes]]`: 

```toml
[[probes]]
address = "tcp://127.0.0.1:5556"
name = "Probe 1"
[[probes]]
address = "tcp://127.0.0.1:5555"
name = "Probe 2"
filter = "2"


| Config   | Meaning    |
|--------------- | --------------- |
| `address`   | The ZMQ socket to subscribe to   |
| `name`   | The name of this probe, shown in the UI   |
| `filter`   | Optional regex filter, applied to the ZMQ message body. Default: `.*`   |

```
