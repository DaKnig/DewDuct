# DewDuct

... is a Youtube player for Linux on desktop and mobile.

## Screenshots

![Video view](https://github.com/DaKnig/DewDuct/assets/37626476/4ea8957e-99d4-4ebc-aaf6-8893784d6df8 "Video view")
![Popular videos view](https://github.com/DaKnig/DewDuct/assets/37626476/bc3635d2-222c-496a-9856-7bb01710f399 "Popular videos view")
![Search view](https://github.com/DaKnig/DewDuct/assets/37626476/a48193cf-ebe0-44ef-ae89-8163a668b595 "Search view")
![Channel view](https://github.com/DaKnig/DewDuct/assets/37626476/aced4e7b-5f76-4035-bdc5-54c6754fd794 "Channel view")

## Design decisions

- The UI should match that of NewPipe, with GTK widgets. I am not a designer
and I dont know how to make custom widgets, or make nice UI, so I just copy
what works!

## Installing

### Alpine linux and PostmarketOS

If you are on edge, run:

```bash
apk add dewduct
```

## Building

### Dependencies

Run time dependencies:

`openssl libadwaita mpv yt-dlp`

Compile time dependencies:

`rust cargo openssl-dev gtk4.0-dev libadwaita-dev`

To compile, run:

```bash
cargo install --git https://github.com/DaKnig/DewDuct
```

### PostmarketOS and Alpine linux:

```bash
apk add rust cargo openssl-dev gtk4.0-dev libadwaita-dev openssl gtk4.0 libadwaita mpv
cargo install --git https://github.com/DaKnig/DewDuct
```

## Road map:

For version 1.0 :

- [x] Popular videos page.

- [x] Cache for thumbnails.

- [x] Video page, with description, where you could press to play video.

- [ ] Select quality of video.

- [x] Popular videos page.

- [ ] Make downloads work with yt-dlp or so... or maybe make it myself?

- [x] Search for videos and channels.

- [ ] Subscribe to channels.

- [x] Subscription list page.

- [ ] "What's New", for videos from subscriptions, with a button for updating the list.

## Get in contact!

Currently, I am the sole developer of DewDuct.

Available on Matrix (`DaKnig` on `matrix.org`)

Please write any and all issues on this github page!

## Contributing

[![GitHub repo Good Issues for newbies](https://img.shields.io/github/issues/DaKnig/DewDuct/good%20first%20issue?style=flat&logo=github&logoColor=green&label=Good%20First%20issues)](https://github.com/DaKnig/DewDuct/issues?q=is%3Aopen+is%3Aissue+label%3A%22good+first+issue%22) [![GitHub Help Wanted issues](https://img.shields.io/github/issues/DaKnig/DewDuct/help%20wanted?style=flat&logo=github&logoColor=b545d1&label=%22Help%20Wanted%22%20issues)](https://github.com/DaKnig/DewDuct/issues?q=is%3Aopen+is%3Aissue+label%3A%22help+wanted%22) [![GitHub Help Wanted PRs](https://img.shields.io/github/issues-pr/DaKnig/DewDuct/help%20wanted?style=flat&logo=github&logoColor=b545d1&label=%22Help%20Wanted%22%20PRs)](https://github.com/DaKnig/DewDuct/pulls?q=is%3Aopen+is%3Aissue+label%3A%22help+wanted%22) [![GitHub repo Issues](https://img.shields.io/github/issues/DaKnig/DewDuct?style=flat&logo=github&logoColor=red&label=Issues)](https://github.com/DaKnig/DewDuct/issues?q=is%3Aopen)

👋 **Welcome, new contributors!**

Whether you're a seasoned developer or just getting started, your contributions are valuable to us. Don't hesitate to jump in, explore the project, and make an impact. To start contributing, please check out our [Contribution Guidelines](CONTRIBUTING.md). 
