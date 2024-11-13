# luced
A lightweighted daemon written by Rust to monitor and record the trend of system metrics, including CPU, disks, memory, etc.

`luced` will collect and record the following information and store into a `SQLite` database in different tables with a preset interval:

 - CPU: percentage of usage
 - Memory: percentage of usage, GB of usage
 - Disks: the given mounted point, its percent of usage, GB of usage

## Installation instructions

Now you can compile by yourself using `cargo`. It can be run directly with its default setting by `luced`.

For configuration, `luced` allows using `-c` to specify the config file. Here is the tamplate of [config.toml](./data/config.toml). You can put it under `~/.config/luced/config.toml`, which is the default working dir of `luced`.

> Recommand to create this folder. If the dir does not exist, the database will be created locally. If it exists, the database will be initialized under `~/.config/luced/database.sqlite` by default.

## Run as a service

Feel free to run this as a user service. Here is the [template](./data/luced.service). Create the service file `~/.config/systemd/user/luced.service` and copying the content.

```bash
# Reload the user daemon to recognize the new service:
systemctl --user daemon-reload

# Enable the service to start it automatically when you log in:
systemctl --user enable luced.service

# Start the service manually:
systemctl --user start luced.service
```

## Future plan

Features awaiting:
 - Automatically managing and squashing the old data so that the database will not be too large.
 - Add more metrics to be collected, such as network, GPU, etc.
 - Develop a GUI app to visualizing and managing `luced`, maybe in another repo.
 - ...

## License
See [LICENSE](./LICENSE).