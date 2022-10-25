obj-m += hello_world.o

all:
# this assumes that you are building the driver for the current kernel
# that is running on your machine. If this is not the case,
# replace it with `make -C /path/to/kernel M=$(PWD) modules`
	    make -C /lib/modules/$(shell uname -r)/build M=$(PWD) modules

clean:
# this assumes that you are building the driver for the current kernel
# that is running on your machine. If this is not the case,
# replace it with `make -C /path/to/kernel M=$(PWD) modules`
	    make -C /lib/modules/$(shell uname -r)/build M=$(PWD) clean

