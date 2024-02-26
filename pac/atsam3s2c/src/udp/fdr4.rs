#[doc = "Register `FDR4` reader"]
pub type R = crate::R<Fdr4Spec>;
#[doc = "Register `FDR4` writer"]
pub type W = crate::W<Fdr4Spec>;
#[doc = "Field `FIFO_DATA` reader - FIFO Data Value"]
pub type FifoDataR = crate::FieldReader;
#[doc = "Field `FIFO_DATA` writer - FIFO Data Value"]
pub type FifoDataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - FIFO Data Value"]
    #[inline(always)]
    pub fn fifo_data(&self) -> FifoDataR {
        FifoDataR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - FIFO Data Value"]
    #[inline(always)]
    #[must_use]
    pub fn fifo_data(&mut self) -> FifoDataW<Fdr4Spec> {
        FifoDataW::new(self, 0)
    }
}
#[doc = "Endpoint FIFO Data Register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdr4::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdr4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fdr4Spec;
impl crate::RegisterSpec for Fdr4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdr4::R`](R) reader structure"]
impl crate::Readable for Fdr4Spec {}
#[doc = "`write(|w| ..)` method takes [`fdr4::W`](W) writer structure"]
impl crate::Writable for Fdr4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
