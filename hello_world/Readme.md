# Hello World Driver

A driver that prints *Hello World* to the debug console (`dmesg`).

## Build

To build the driver, make sure that the Makefile is correct and
run

```bash
make
```

## Load the driver

To load the driver use

```bash
sudo insmod hello_world.ko
```

To verify that the driver works, run:

```bash
dmesg
```

You should see a list of kernel messages similar to:
```
[212429.659083] audit: type=1400 audit(1666644503.387:41): apparmor="STATUS" operation="profile_replace" profile="unconfined" name="snap.lxd.check-kernel" pid=21400 comm="apparmor_parser"
[212429.795154] audit: type=1400 audit(1666644503.523:42): apparmor="STATUS" operation="profile_replace" profile="unconfined" name="snap.lxd.hook.configure" pid=21402 comm="apparmor_parser"
[212429.796168] audit: type=1400 audit(1666644503.523:43): apparmor="STATUS" operation="profile_replace" profile="unconfined" name="snap.lxd.daemon" pid=21401 comm="apparmor_parser"
[212429.885563] audit: type=1400 audit(1666644503.611:44): apparmor="STATUS" operation="profile_replace" profile="unconfined" name="snap.lxd.hook.install" pid=21403 comm="apparmor_parser"
[212429.900052] audit: type=1400 audit(1666644503.627:45): apparmor="STATUS" operation="profile_replace" profile="unconfined" name="snap.lxd.hook.remove" pid=21404 comm="apparmor_parser"
[255420.964020] hello_world: Hello World loaded
[255420.964026] hello_world: Hello World
```

## Unload the driver

To unload the driver use

```bash
sudo rmmod hello_world
```