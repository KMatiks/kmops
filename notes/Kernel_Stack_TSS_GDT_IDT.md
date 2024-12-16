- Dedicated memory region per kernel thread
- Typically 8-16 KB in size
	- Fixed
- Stack overflow stems from fixed nature
- Guard page placed after kernel stack
	- Marked as invalid page.
	- Since it's invalid, any access attempts will result in page fault
```
High Memory Address
┌───────────────┐
│   Guard Page  │ - Intentionally invalid page
├───────────────┤
│  Kernel Stack │ - Actual usable stack memory
└───────────────┘
Low Memory Address
```

- Double faults lead the CPU to try pushing the exception stack frame onto the current frame.
	- Includes the instruction pointer, code segment selector, CPU flags, other relevant info
	- Does this to PRESERVE the exact state of execution at the moment of exception
	- A triple fault can occur when:
		- We try writing to the stackguard and get a page fault exception.
		- A second page fault is triggered due to the CPU's attempt at writing the [interrupt stack frame](https://os.phil-opp.com/cpu-exceptions/#the-interrupt-stack-frame) the stack
		- This is results in two contributory exceptions, leading to a triple fault and thus a restart
		- Remedied by switching to another stack (done via HW before pushing of interrupt stack frame)

```
0x0000000000000000 - User Space Low Addresses
│
0x00007FFFFFFFFFFF - User Space Upper Limit
│
0x0000800000000000 - Kernel/Kernel Module Space Begins
│   ├── Kernel Code
│   ├── Kernel Data Structures
│   │   ├── IDT (Interrupt Descriptor Table)
│   │   ├── GDT (Global Descriptor Table)
│   │   ├── Per-CPU Data Structures
│   │   └── Other Global Kernel Structures
│   ├── Kernel Stacks
│   │   ├── Init Process Kernel Stack
│   │   ├── Per-Thread Kernel Stacks
│   │   └── Guard Pages
│   ├── Dynamic Kernel Mappings
│   └── Permanent Kernel Mappings
│
0xFFFFFFFFFFFFFFFF - Top of Virtual Address Space
```

```
Memory Regions:
┌───────────────┐ High Address
│    IDT        │ - Interrupt Descriptor Table
├───────────────┤
│    GDT        │ - Global Descriptor Table
├───────────────┤
│ Kernel Stack  │ - Per-thread kernel execution context
├───────────────┤
│ Other Kernel  │ - Various kernel data structures
│ Structures    │
└───────────────┘ Low Address
```

# TSS
- Task state segment
- In 32-bit mode, holds info about a task (e.g. processor register states)
- In 64-bit mode
	- Holds 2 stack tables
		- Privilege Stack Table (pst)
			- Used by CPU when privilege level changes
			- E.g.)
				- User mode (privilege level 3) exception would typically have CPU switch to kernel mode (privilege level 0) before invoking exception handler.
		- Interrupt Stack Table (ist)
	- Loading it
		- TSS uses segmentation system
		- Need to add a new segment descriptor to the GDT
		- Then can load via `ltr` instruction invocation
#### IST
- Interrupt stack table
- Table of 7 pointers to known-good stacks
- Part of TSS
# GDT (Global Descriptor Table)
- Contains segments of the program
- Segmentation memory model: https://pages.cs.wisc.edu/~remzi/OSTEP/vm-segmentation.pdf
- Needed in x86-64 for:
	- Kernel/user mode configuration or TSS loading
# IDT
- Interrupt descriptor table
- Defines the CPU exception-handler function associations
- Each entry has an options field
	- One of these includes `Interrupt Stack Table Index`, which we can use to specify which stack to switch to (1-7).
		- This can be used for the double fault handler to use a separate stack...









