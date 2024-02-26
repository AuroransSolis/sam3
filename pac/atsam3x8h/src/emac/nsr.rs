#[doc = "Register `NSR` reader"]
pub type R = crate::R<NsrSpec>;
#[doc = "Field `MDIO` reader - "]
pub type MdioR = crate::BitReader;
#[doc = "Field `IDLE` reader - "]
pub type IdleR = crate::BitReader;
impl R {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn mdio(&self) -> MdioR {
        MdioR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn idle(&self) -> IdleR {
        IdleR::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "Network Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nsr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NsrSpec;
impl crate::RegisterSpec for NsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nsr::R`](R) reader structure"]
impl crate::Readable for NsrSpec {}
