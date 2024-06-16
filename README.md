
    
<div align="center">
    
![roller-banner](https://github.com/wavefnx/roller/assets/157986149/4b235f87-ede1-4512-80eb-76c7a61814d5)
</div>

<div align="center"> 
    
[Overview](#Overview) | [Disclaimer](#Disclaimer)  | [Installation](#Installation) | [Usage](#Usage) | [Examples](#Examples) | [Aknowledgements](#Aknowledgements) | [License](#License)
</div>


<div align="center">
    
[![CI](https://img.shields.io/github/actions/workflow/status/wavefnx/roller/ci.yml?style=flat-square&label=CI&labelColor=%23343940&color=%2340C057)](https://github.com/wavefnx/roller/actions/workflows/ci.yml)
[![MPL-2.0](https://img.shields.io/github/license/wavefnx/roller?style=flat-square&color=blue&label=)](LICENSE)
</div>

## Overview
Terminal interface tracking gas, transactions and data processed by Decentralized Networks. 


## Disclaimer
This library is in early development stages and subject to potential breaking changes.
Backward compatibility or further maintenance is not guaranteed. The package is intentionally not published on crates.io until and if there's an `alpha` release in the future.

Contributions are welcome. Users are encouraged to submit pull requests, fork, or alter the code in accordance with the terms outlined in the [LICENSE](LICENSE).


## Installation
You can currently build from source by running the following command in the root of the repository:
```rust
cargo build --release
```

## Usage
```
Terminal interface tracking gas, transactions and data processed by Decentralized Networks

Usage: roller [OPTIONS]

Options:
  -i, --interval-ms <INTERVAL_MS>    Interval in ms to wait between events. [default: 100]
                                     Increase for lower resource consumption, decrease for more frequent updates
      --api-endpoint <API_ENDPOINT>  Change the default API Endpoint by specifying a different URL
  -h, --help                         Print help
  -V, --version                      Print version
```

## Examples

```sh
// The default update interval (100ms) should offer a good balance between low resource consumption and updated data.
roller

// To run it as a long, background widget, you can increase the interval to 500ms, 1s or more.
roller -i 1000

// Additionally, to get SSE data live with no bounds, as they are being produced by the SSE API:
roller -i 0
```

## Aknowledgements
The interface is using the same API as [rollup.wtf](https://rollup.wtf), which is provided by [conduit.xyz](https://conduit.xyz).

## License
This library is released under the terms of the [Mozilla Public License](https://www.mozilla.org/en-US/MPL/) version 2.0. See [LICENSE](LICENSE).
