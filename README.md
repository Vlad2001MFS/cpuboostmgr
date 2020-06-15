# CPU Boost Manager for Linux

Monitor temperature of CPU and enable/disable CPU-boost based on max/min temperature from config.
If the temperature of CPU greater the max_temp then it disables CPU-boost. When the temperature will be less then min_temp the manager enables CPU-boost again. Rate of check of temperature can be modified by user and default is 100ms.

CPU Boost Manager can be useful for notebook users, because of CPU-boost may cause high temperatures of chip in some cases. This app prevents extreme temperatures in 90% cases(depends on your configuration of app).

____

App can be configured by file `~/.config/cpuboostmgrrc`. For example:
```
update_time = 75
max_temp = 75
min_temp = 70
cpu_boost_file = /sys/devices/system/cpu/cpufreq/boost
cpu_temp_file = /sys/class/hwmon/hwmon3/temp2_input
is_log_temp = false
```

Also can be used default builtin config:
```
update_time = 100
max_temp = 75
min_temp = 65
cpu_boost_file = /sys/devices/system/cpu/cpufreq/boost
cpu_temp_file = /sys/class/hwmon/hwmon3/temp2_input
is_log_temp = false
```

In my case `/sys/class/hwmon/hwmon3/temp2_input` is Tdie sensor of CPU. Check it on your system. Prefer Tdie sensor in all cases when it available, because of it has a highest update rate.

If `is_log_temp` is true app will be print CPU temperature and CPU-boost state in stdout with given update rate.

____

Assumes what app will be used as service of systemd with the following unit file:
```
[Unit]
  Description=CPU Boost Manager

[Service]
  ExecStart=sudo /usr/bin/cpuboostmgr
  Type=simple
  KillMode=process

  SyslogIdentifier=cpuboostmgr
  SyslogFacility=daemon

  Restart=always

[Install]
  WantedBy=graphical.target
```

Values of `Type` and `KillMode` in section `Service` are mostly important. Does not recommend to change this values.
