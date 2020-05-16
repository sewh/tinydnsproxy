tinydnsproxy
============

tinydnsproxy accepts and proxies DNS messages to upstream DNS-over-TLS providers. DNS-over-TLS is a protcol that uses [Transport Layer Security, or TLS](https://en.wikipedia.org/wiki/Transport_Layer_Security) to ensure that data sent over the network is reasonably secured from interception and modification.

In addition, tinydnsproxy can also prevent domain names from being resolved according to a set of block lists, allowing you to block certain domains from your network. These block lists can be updated automatically. Common use cases for domain blocking include; blocking website ads; preventing trackers from following you around the internet; and stopping malware on your network connecting back to malicious domains.

Ensure you have read the [Security](#Security) section below before using this tool.

## Usage

Start tinydnsproxy with a path to a config file, e.g. `./tinydnsproxy path_to_config.toml`. You can find an [example config file in the config/example.toml file](./config/example.toml). If any option is ambiguous, please feel free to open up an issue and I'll fix it.

## Security

In tinydnsproxy's config, each upstream DNS-over-TLS provider section (`dot_provider`) must have a `hostname` parameter. This is usually listed by the DNS-over-TLS provider on their website. For convenience, here are some links to where some major providers detail what their DNS-over-TLS hostnames are (valid as of 16th May 2020):

* Cloudflare :: [https://developers.cloudflare.com/1.1.1.1/dns-over-tls/](https://developers.cloudflare.com/1.1.1.1/dns-over-tls/);
* Google :: [https://developers.google.com/speed/public-dns/docs/dns-over-tls](https://developers.google.com/speed/public-dns/docs/dns-over-tls)
* LibreDNS :: [https://libredns.gr/](https://libredns.gr/)

This parameter is essential to ensure tinydnsproxy can verify it's talking to the provider you intend, and not a third-party who is attempting to redirect your traffic to their own DNS-over-TLS server. 

### TLS Considerations

tinydnsproxy attempts to make some sensible decisions about TLS;

* Remote peers are validated using the trust store on the device. This is the same mode of operation that most TLS clients, including web browsers, operate with;
* tinydnsproxy currently doesn't support certificate pinning. Despite being more secure once in operation, it is a difficult and ongoing process to configure correctly for each provider. That said, it is a future ambition to support it for those who are happy to take on the burden of using it;
* tinydnsproxy uses the [native\_tls Rust library](https://docs.rs/native-tls/0.2.4/native_tls/) library to create TLS connections. `native_tls` uses the platform's preferred TLS library, all of which are heavily used and trusted by a large amount of users.

### Other Security Considerations

* All HTTP block lists should be fetched using HTTPS, otherwise a third-party could modify the block list while you are downloading it;
* This code has not been reviewed by security experts;
* tinydnsproxy comes with no guarantees or promises of support. 

## Building with Cargo

To build natively, you'll need a working rust installation. [rustup.rs](https://rustup.rs/) is the fastest way to do this. Then running `cargo build --release` will produce a release quality binary.

## Building for Embedded Devices (such as routers)

I wanted to deploy tinydnsproxy to an ARM-based device, so there is an included makefile and dockerfile for building an ARM version that should work on embedded devices such as routers (I am deploying tinsdnsproxy onto an ARM-based router that runs [OpenWRT](https://openwrt.org/).) To build this version, ensure Docker is installed and run `make armv7`. Note, it expects to be run as root and if it isn't it will prompt you for a sudo password. I encourage you to review the makefile and the dockerfile to ensure it doesn't do anything you'd find objectionable with root privileges.

Currently only ARMv7 is supported. Adding support for other targets such as MIPS and other ARM chipsets is a future ambition.

## License

The code in this repository is licensed to you under the terms of the GNU General Public License Version V3 (GPLv3.) The full terms of the license are included in the [LICENSE.txt](./LICENSE.txt) file.

tinydnsproxy also relies on third-party libraries and components that are referred to in the source code and linked to in binaries derived from the source. Please ensure you adhere the license terms of those libaries and components in addition to the terms of this repository if you plan to host your own copies of the source code or binaries derived from the source code.
