import("../pkg/index.js")
    .then(photonic => {
        const canvas = document.getElementById('output');

        const root = {
            name: 'blackout',
            type: 'blackout',
            active: {
                input: 'blackout',
                initial: false,
            },
            source: {
                type: 'brightness',
                name: 'brightness',
                brightness: {
                    input: 'brightness',
                    initial: 1.0,
                },
                source: {
                    type: 'raindrops',
                    name: 'raindrops',
                    rate: 0.3,
                    decay: [0.6, 0.8],
                    color: {
                        type: 'fader',
                        easing: {
                            func: 'linear',
                            speed: '5s',
                        },
                        input: {
                            type: 'sequence',
                            next: {
                                input: 'preset_next',
                            },
                            prev: {
                                input: 'preset_prev',
                            },
                            values: [
                                ["hsl(245.31, 0.5, 0.5)", "hsl(333.47, 0.7, 0.5)"],
                                ["hsl(0.0, 0.45, 0.5)", "hsl(17.5, 0.55, 0.5)"],
                                ["hsl(187.5, 0.25, 0.5)", "hsl(223.92, 0.5, 0.5)"],
                            ],
                        },
                    },
                },
                // source: {
                //     type: 'solid',
                //     name: 'solid',
                //     solid: {
                //         type: 'fader',
                //         easing: {
                //             func: 'linear',
                //             speed: '3s',
                //         },
                //         input: {
                //             type: 'sequence',
                //             next: {
                //                 input: 'preset_next',
                //             },
                //             prev: {
                //                 input: 'preset_prev',
                //             },
                //             values: [
                //                 "hsl(245.31, 0.5, 0.5)",
                //                 "hsl(0.0, 0.45, 0.5)",
                //                 "hsl(187.5, 0.25, 0.5)",
                //             ],
                //         },
                //     },
                // },
            },
        };

        const system = photonic.render(canvas, root, 100);

        let prev = undefined;

        function update(curr) {
            if (prev === undefined) {
                prev = curr;
            }

            if (prev !== curr) {
                const duration = BigInt(Math.trunc((curr - prev) * 1000));
                prev = curr;

                system.render(duration);
            }

            window.requestAnimationFrame(update);
        }

        window.requestAnimationFrame(update);

        document.getElementById('next').onclick = (event) => {
            system.send('preset_next', undefined);
        }
    }).catch(console.error);