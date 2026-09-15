The purpose of this project is to investigate how a driver drives a device.

The rough idea is that we'll bind a device to vfio-pci and this program will take ownership of it. Then, we will present
the same device to a VM over a vfio-user socket. The only thing we'll do is intercept requests and interrupts. 
