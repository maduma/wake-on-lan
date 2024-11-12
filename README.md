# wake-device-on-lan

Command to start a device using 'wake on lan' protocol, given his mac-address

## Usage
```
wake-on-lan [options] [device]
wake-on-lan [options] create-alias mac alias
wake-on-lan remove-alias alias
wake-on-lan set-default-alias alias
wake-on-lan set-default-source-ip ip

device can be a mac address (eg 2c:f0:5d:e1:9e:d6) or an alias

options:

[-s ip|--source-ip ip]    set source ip for network selection
```