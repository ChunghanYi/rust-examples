// SPDX-License-Identifier: GPL-2.0

DefinitionBlock ("", "SSDT", 2, "TEST", "VIRTACPI", 0x00000001)
{
	Scope (\_SB)
	{
		Device (T432)
		{
			Name (_HID, "LNUXBEEF")  // ACPI hardware ID to match
			Name (_UID, 1)
			Name (_STA, 0x0F)        // Device present, enabled
			Name (_CRS, ResourceTemplate ()
			{
				Memory32Fixed (ReadWrite, 0xFED00000, 0x1000)
			})
		}
	}
}
