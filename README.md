
## Dependencies

Rusty Web depends on fuse3 and the headers files; on Ubuntu:

```
apt-get install libfuse-dev
```

## Helpful Commands

If the program crashes without unmount and you get the error:
```
umount: mnt: Transport endpoint is not connected
```
use the following command to unmount the file system:

```
fusermount -u <mount_path>
```