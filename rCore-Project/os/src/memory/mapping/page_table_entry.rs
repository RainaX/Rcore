use crate::memory::address::*;
use bit_field::BitField;
use bitflags::*;

#[derive(Copy, Clone, Default)]
pub struct PageTableEntry(usize);


const FLAG_RANGE: core::ops::Range<usize> = 0..8;
const PAGE_NUMBER_RANGE: core::ops::Range<usize> = 10..54;


impl PageTableEntry {
    pub fn new(page_number: Option<PhysicalPageNumber>, mut flags: Flags) -> Self {
        flags.set(Flags::VALID, page_number.is_some());
        Self (
            *0usize
                .set_bits(FLAG_RANGE, flags.bits() as usize)
                .set_bits(PAGE_NUMBER_RANGE, page_number.unwrap_or_default().into()),
        )
    }


    pub fn update_page_number(&mut self, ppn: Option<PhysicalPageNumber>) {
        if let Some(ppn) = ppn {
            self.0
                .set_bits(FLAG_RANGE, (self.flags() | Flags::VALID).bits() as usize)
                .set_bits(PAGE_NUMBER_RANGE, ppn.into());
        } else {
            self.0
                .set_bits(FLAG_RANGE, (self.flags() - Flags::VALID).bits() as usize)
                .set_bits(PAGE_NUMBER_RANGE, 0);
        }
    }


    pub fn clear(&mut self) {
        self.0 = 0;
    }


    pub fn page_number(&self) -> PhysicalPageNumber {
        PhysicalPageNumber::from(self.0.get_bits(10..54))
    }


    pub fn address(&self) -> PhysicalAddress {
        PhysicalAddress::from(self.page_number())
    }


    pub fn flags(&self) -> Flags {
        unsafe { Flags::from_bits_unchecked(self.0.get_bits(..8) as u8) }
    }


    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }


    pub fn has_next_level(&self) -> bool {
        let flags = self.flags();
        !(flags.contains(Flags::READABLE)
            || flags.contains(Flags::WRITABLE)
            || flags.contains(Flags::EXECUTABLE))
    }
}


impl core::fmt::Debug for PageTableEntry {
    fn fmt(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        formatter
            .debug_struct("PageTableEntry")
            .field("value", &self.0)
            .field("page_number", &self.page_number())
            .field("flags", &self.flags())
            .finish()
    }
}


bitflags! {
    #[derive(Default)]
    pub struct Flags: u8 {
        const VALID =       1 << 0;
        const READABLE =    1 << 1;
        const WRITABLE =    1 << 2;
        const EXECUTABLE =  1 << 3;
        const USER =        1 << 4;
        const GLOBAL =      1 << 5;
        const ACCESSED =    1 << 6;
        const DIRTY =       1 << 7;
    }
}


macro_rules! implement_flags {
    ($field: ident, $name: ident, $quote: literal) => {
        impl Flags {
            pub fn $name(value: bool) -> Flags {
                if value {
                    Flags::$field
                } else {
                    Flags::empty()
                }
            }
        }
    };
}


implement_flags! {USER, user, "USER"}
implement_flags! {READABLE, readable, "READABLE"}
implement_flags! {WRITABLE, writable, "WRITABLE"}
implement_flags! {EXECUTABLE, executable, "EXECUTABLE"}

