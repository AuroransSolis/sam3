#[doc = "Register `FIFO164` reader"]
pub type R = crate::R<Fifo164Spec>;
#[doc = "Register `FIFO164` writer"]
pub type W = crate::W<Fifo164Spec>;
#[doc = "Field `DATA` reader - Data to Read or Data to Write"]
pub type DataR = crate::FieldReader<u32>;
#[doc = "Field `DATA` writer - Data to Read or Data to Write"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Data to Read or Data to Write"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Data to Read or Data to Write"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DataW<Fifo164Spec> {
        DataW::new(self, 0)
    }
}
#[doc = "FIFO Memory Aperture0 164\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo164::R`](R).  You can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo164::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fifo164Spec;
impl crate::RegisterSpec for Fifo164Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo164::R`](R) reader structure"]
impl crate::Readable for Fifo164Spec {}
#[doc = "`write(|w| ..)` method takes [`fifo164::W`](W) writer structure"]
impl crate::Writable for Fifo164Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
