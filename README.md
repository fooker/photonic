# 🌠 Photonic &emsp; [![Latest Version]][crates.io] [![Build Status]][actions] [![Documentation]][docs]

[Build Status]: https://img.shields.io/github/actions/workflow/status/fooker/photonic/default.yml?branch=main
[actions]: https://github.com/fooker/photonic/actions?query=branch%3Amain

[Latest Version]: https://img.shields.io/crates/v/photonic.svg
[crates.io]: https://crates.io/crates/photonic

[Documentation]: https://docs.rs/photonic/badge.svg
[docs]: https://docs.rs/photonic


Photonic is an open-source rust framework for lighting animation and control.

Whether you're creating a dazzling lighting setup for an event or enhancing the ambient lighting of your living space, Photonic provides a versatile platform for managing and controlling your lighting setup. 

## Features
* **🎇 Lighting Animations:** Create dynamic lighting effects by combining a predefined effects or implement your own individual animations.
* **🎛 Control Interfaces:** Expose what needs to be controlled and integrate with a variety of protocols and applications.
* **🏮 Customisation:** Tailor your lighting setup to suit your specific needs and preferences.
* **🌐 Open Source:** Collaborate with the community to improve and expand Photonic's capabilities.

## Overview
Photonic main component is a Scene. A Scene consists of multiple nodes, attributes and inputs. The nodes form a graph where each frame is passed from node to node and each node can process and manipulate the frame. Each node exposes attributes that control the behavior of the node. Depending on how attributes are assigned, the value of the attribute can change over time. Inputs control a specific value and are exported through the control interfaces.

## Example
```rust
#[tokio::main]
async fn main() -> Result<()> {
    let mut scene = Scene::new();

    let rate_input = scene.input::<f32>("rate")?;
    let color_input = scene.input::<Range<Rgb>>("color")?;

    let base = scene.node("raindrops", Raindrops {
        rate: rate_input.attr(0.3),
        decay: (0.96, 0.98).fixed(),
        color: color_input
            .attr(Range(Hsl::new(187.5, 0.25, 0.5).into_color(), Hsl::new(223.92, 0.5, 0.5).into_color()))
            .map(|v| v.map(Hsl::from_color)),
    })?;
    
    let brightness_input = scene.input::<f32>("brightness")?;

    let brightness = scene.node("brightness", Brightness {
        value: brightness_input.attr(1.0),
        source: base,
        range: None,
    })?;

    let output = Terminal::new(80)
        .with_path("/tmp/Photonic")
        .with_waterfall(true);

    let mut scene = scene.run(brightness, output).await?;

    let cli = Photonic_interface_cli::stdio::CLI;
    scene.serve("CLI", cli);

    return Ok(scene.run(60).await?);
}
```

## Related crates
The `photonic` crate contains the base framework and some general useful helpers.
Other, more advanced and specialized functionality is provided by sibling crates.
Here is a list of all crates known:

| Name                                                                    | Description                                                             |
|-------------------------------------------------------------------------|-------------------------------------------------------------------------|
| [photonic-effects](https://crates.io/photonic-effects/)                 | A curated set of nodes and attributes                                   |
| [photonic-dynamic](https://crates.io/photonic-dynamic/)                 | Deserialize photonic scenes from a declaration file                     |
| [photonic-dynamic-runner](https://crates.io/photonic-dynamic-runner/)   | Load and run photonic scenes from a declaration file                    |
| [photonic-audio](https://crates.io/photonic-audio/)                     | React to audio inputs                                                   |
| [photonic-input-cli](https://crates.io/photonic-interface-cli/)         | Interactive CLI interface                                               |
| [photonic-input-grpc](https://crates.io/photonic-interface-grpc/)       | Remote control photonic using gRPC calls                                |
| [photonic-input-mqtt](https://crates.io/photonic-interface-grpc/)       | Expose photonic inputs as MQTT topics                                   |
| [photonic-input-restore](https://crates.io/photonic-interface-restore/) | Save and restore photonic inputs from persistence file                  |
| [photonic-output-net](https://crates.io/photonic-output-net/)           | Output scenes to network protocols like WLED, NetDMX, ArtNet and others |
| [photonic-output-null](https://crates.io/photonic-output-null/)         | Run a scene without any output - for testing                            |
| [photonic-output-split](https://crates.io/photonic-output-split/)       | Split output to multiple output targets                                 |
| [photonic-output-terminal](https://crates.io/photonic-output-terminal/) | Output scenes to your to a terminal                                     |

If there is a crate missing in this list, please create a pull request and tell us about it.

## Contributing
We welcome contributions from the community to help improve Photonic.
Whether you're a developer, designer, or enthusiast, there are many ways to get involved:

* **Bug Reports:** Report any issues or bugs you encounter while using Photonic.
* **Feature Requests:** Suggest new features or enhancements to make Photonic even more powerful.
* **Pull Requests:** Submit pull requests to address bugs, implement new features, or improve documentation.

## License
Photonic is licensed under the MIT License, which means you are free to use, modify, and distribute the software for both commercial and non-commercial purposes. See the [LICENSE](./LICENSE) file for more details.

## Support
If you have any questions, concerns, or feedback about Photonic, please [contact us](mailto:fooker@lab.sh) or open an issue on the project's GitHub repository.

## Acknowledgements
We would like to thank all contributors and supporters who have helped make Photonic possible. Your contributions and feedback are greatly appreciated!

