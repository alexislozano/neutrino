# neutrino

**I am not working anymore on this project. If you want to become a maintainer of neutrino, please answer to [this issue](https://github.com/alexislozano/neutrino/issues/87).**

## Preamble

[Docs](https://docs.rs/neutrino) | 
[Repo](https://github.com/alexislozano/neutrino) | 
[Wiki](https://github.com/alexislozano/neutrino/wiki) | 
[Crate](https://crates.io/crates/neutrino)

Neutrino is a MVC GUI framework written in Rust. It lets users create GUI 
applications by positioning widgets on a window and by handling events. 
Neutrino is based on the [web-view](https://crates.io/crates/web-view) crate
provided by Boscop. As such, Neutrino renders the application using web 
technologies as HTML and CSS. 
As it is based on web-view, Neutrino does not embed a whole web browser. So 
don't worry, due to the very lightweight footprint of web-view, you won't 
have to buy more memory for your computer.

## Install

In order to use Neutrino, you will have to use cargo. Just add the following
line to your `Cargo.toml` and you'll be done : 

```text
neutrino = "<last_version>"
```

On Linux, you'll have to install webkit2gtk's development library. For example,
in Ubuntu or Debian:
```
sudo apt install -y libwebkit2gtk-4.0-dev
```

## Examples

![](https://raw.githubusercontent.com/wiki/alexislozano/neutrino/images/image_viewer/3.png)

![](https://raw.githubusercontent.com/wiki/alexislozano/neutrino/images/styling/3.png)

![](https://raw.githubusercontent.com/wiki/alexislozano/neutrino/images/styling/4.png)

![](https://raw.githubusercontent.com/wiki/alexislozano/neutrino/images/styling/5.png)

![](https://raw.githubusercontent.com/wiki/alexislozano/neutrino/images/styling/6.png)
