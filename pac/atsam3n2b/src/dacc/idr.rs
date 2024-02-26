#[doc = "Register `IDR` writer"]
pub type W = crate::W<IdrSpec>;
#[doc = "Field `TXRDY` writer - Transmission Ready Interrupt Disable"]
pub type TxrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENDTX` writer - End of PDC Interrupt Disable"]
pub type EndtxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXBUFE` writer - Buffer Empty Interrupt Disable"]
pub type TxbufeW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Transmission Ready Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn txrdy(&mut self) -> TxrdyW<IdrSpec> {
        TxrdyW::new(self, 0)
    }
    #[doc = "Bit 1 - End of PDC Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn endtx(&mut self) -> EndtxW<IdrSpec> {
        EndtxW::new(self, 1)
    }
    #[doc = "Bit 2 - Buffer Empty Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn txbufe(&mut self) -> TxbufeW<IdrSpec> {
        TxbufeW::new(self, 2)
    }
}
#[doc = "Interrupt Disable Register\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IdrSpec;
impl crate::RegisterSpec for IdrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`idr::W`](W) writer structure"]
impl crate::Writable for IdrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
