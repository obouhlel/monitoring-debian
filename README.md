# Files for monitoring

- CPU : `/proc/stat`
- RAM : `/proc/meminfo`
- DISK : `/proc/diskstats`
- NET : `/proc/net/dev`
- Process :
    1. Get all PID process files: `ls /proc | grep -E '^[0-9]+$'`
    2. Name and state: Read `/proc/[PID]/status`.
    3. Command: Read `/proc/[PID]/cmdline`.
