# API for monitoring Debian OS

CPU : `/proc/stat`
RAM : `/proc/meminfo`
DISK : `/proc/diskstats`

Get all PID process files:

```sh
ls /proc | grep -E '^[0-9]+$'
```

Name and state: Read /proc/[PID]/stat or /proc/[PID]/status.
Command: Read /proc/[PID]/cmdline.
