#[doc = "Register `FIFO221` reader"]
pub type R = crate::R<Fifo221Spec>;
#[doc = "Register `FIFO221` writer"]
pub type W = crate::W<Fifo221Spec>;
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
    pub fn data(&mut self) -> DataW<Fifo221Spec> {
        DataW::new(self, 0)
    }
}
#[doc = "FIFO Memory Aperture0 221\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo221::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo221::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fifo221Spec;
impl crate::RegisterSpec for Fifo221Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo221::R`](R) reader structure"]
impl crate::Readable for Fifo221Spec {}
#[doc = "`write(|w| ..)` method takes [`fifo221::W`](W) writer structure"]
impl crate::Writable for Fifo221Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
