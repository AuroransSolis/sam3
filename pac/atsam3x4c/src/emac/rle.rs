#[doc = "Register `RLE` reader"]
pub type R = crate::R<RleSpec>;
#[doc = "Register `RLE` writer"]
pub type W = crate::W<RleSpec>;
#[doc = "Field `RLFM` reader - Receive Length Field Mismatch"]
pub type RlfmR = crate::FieldReader;
#[doc = "Field `RLFM` writer - Receive Length Field Mismatch"]
pub type RlfmW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Receive Length Field Mismatch"]
    #[inline(always)]
    pub fn rlfm(&self) -> RlfmR {
        RlfmR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Receive Length Field Mismatch"]
    #[inline(always)]
    #[must_use]
    pub fn rlfm(&mut self) -> RlfmW<RleSpec> {
        RlfmW::new(self, 0)
    }
}
#[doc = "Received Length Field Mismatch Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rle::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rle::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RleSpec;
impl crate::RegisterSpec for RleSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rle::R`](R) reader structure"]
impl crate::Readable for RleSpec {}
#[doc = "`write(|w| ..)` method takes [`rle::W`](W) writer structure"]
impl crate::Writable for RleSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RLE to value 0"]
impl crate::Resettable for RleSpec {
    const RESET_VALUE: u32 = 0;
}
