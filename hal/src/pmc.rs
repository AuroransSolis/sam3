use crate::pac::{pmc::pmc_mckr::PRES_A, PMC};

pub fn set_master_clk_prescaler(pmc: &PMC, prescaler: PRES_A) {
    // Set prescaler bits
    pmc.pmc_mckr
        .write(|master_clk_reg| master_clk_reg.pres().variant(prescaler));
    // Wait for clock to be ready again
    while pmc.pmc_sr.read().mckrdy().bit_is_clear() {}
}

// Only defined for peripheral IDs 9, 11-44
pub(crate) unsafe fn enable_peripheral_clk(pmc: &PMC, id: u32) {
    if id < 32 {
        let mask = 1 << id;
        if pmc.pmc_pcsr0.read().bits() & mask == 0 {
            pmc.pmc_pcer0.write_with_zero(|reg| reg.bits(mask));
        }
    } else {
        let mask = 1 << (id - 32);
        if pmc.pmc_pcsr1.read().bits() & mask == 0 {
            pmc.pmc_pcer1.write_with_zero(|reg| reg.bits(mask));
        }
    }
}

pub(crate) unsafe fn disable_peripheral_clk(pmc: &PMC, id: u32) {
    if id < 32 {
        let mask = 1 << id;
        if pmc.pmc_pcsr0.read().bits() & mask == mask {
            pmc.pmc_pcdr0.write_with_zero(|reg| reg.bits(mask));
        }
    } else {
        let mask = 1 << (id - 32);
        if pmc.pmc_pcsr1.read().bits() & mask == mask {
            pmc.pmc_pcdr1.write_with_zero(|reg| reg.bits(mask));
        }
    }
}
