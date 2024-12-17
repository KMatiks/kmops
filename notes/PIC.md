# Intel 8259 PIC
- Old systems typically had a primary and secondary PIC.
	- The secondary was connected to one of the interrupt lines of the primary PIC:
```
                     ____________                          ____________
Real Time Clock --> |            |   Timer -------------> |            |
ACPI -------------> |            |   Keyboard-----------> |            |      _____
Available --------> | Secondary  |----------------------> | Primary    |     |     |
Available --------> | Interrupt  |   Serial Port 2 -----> | Interrupt  |---> | CPU |
Mouse ------------> | Controller |   Serial Port 1 -----> | Controller |     |_____|
Co-Processor -----> |            |   Parallel Port 2/3 -> |            |
Primary ATA ------> |            |   Floppy disk -------> |            |
Secondary ATA ----> |____________|   Parallel Port 1----> |____________|

```
- Send a hardware interrupt to the CPU to avoid polling
- Each controller configured through two I/O ports
	- Command port
	- Data port
- I/O ports
	- Primary
		- Command port: 0x20
		- Data port: 0x21
	- Secondary
		- Command port: 0xa0
		- Data port: 0xa1
- Default PIC sends interrupt vector numbers in range of reserved values.
	- Typically new range of 32-47 chosen
- 
