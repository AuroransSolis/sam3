#[doc = "Register `FDR2` reader"]
pub type R = crate::R<Fdr2Spec>;
#[doc = "Register `FDR2` writer"]
pub type W = crate::W<Fdr2Spec>;
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
    pub fn fifo_data(&mut self) -> FifoDataW<Fdr2Spec> {
        FifoDataW::new(self, 0)
    }
}
#[doc = "Endpoint FIFO Data Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`fdr2::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fdr2Spec;
impl crate::RegisterSpec for Fdr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdr2::R`](R) reader structure"]
impl crate::Readable for Fdr2Spec {}
#[doc = "`write(|w| ..)` method takes [`fdr2::W`](W) writer structure"]
impl crate::Writable for Fdr2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
