MEMORY
{
	FLASH (rx): ORIGIN = 0x08000000, LENGTH = 256k
	RAM (rwx): ORIGIN = 0x20000000, LENGTH = 40k
}

_estack = ORIGIN(RAM) + LENGTH(RAM);
_Min_Stack_Size = 1k;

SECTIONS
{
	.isr_vector :
	{
		KEEP(*(.vector_table));
	} > FLASH

	.text :
	{
		*(.text*);
		*(.rodata*);
		. = ALIGN(4);
	} > FLASH

	_etext = ADDR(.text) + SIZEOF(.text);

	.data : AT (_etext)
	{
		. = ALIGN(4);
		_sdata = .;
		*(.data*);
		. = ALIGN(4);
		_edata = .;
	} > RAM

	_sidata = LOADADDR(.data);

	.bss (NOLOAD):
	{
		. = ALIGN(4);
		_sbss = .;
		*(.bss*);
		*(COMMON);
		. = ALIGN(4);
		_ebss = .;
	} > RAM

	/DISCARD/ :
	{
		*(.ARM.exidx);
		*(.ARM.exidx.*);
		*(.ARM.extab.*);
	}
}

ASSERT(_ebss + _Min_Stack_Size <= _estack, "ERROR: RAM overflowed!");
