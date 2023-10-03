#[doc = "Register `PMC_OCR` reader"]
pub type R = crate::R<PMC_OCR_SPEC>;
#[doc = "Register `PMC_OCR` writer"]
pub type W = crate::W<PMC_OCR_SPEC>;
#[doc = "Field `CAL4` reader - RC Oscillator Calibration bits for 4 MHz"]
pub type CAL4_R = crate::FieldReader;
#[doc = "Field `CAL4` writer - RC Oscillator Calibration bits for 4 MHz"]
pub type CAL4_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `SEL4` reader - Selection of RC Oscillator Calibration bits for 4 MHz"]
pub type SEL4_R = crate::BitReader;
#[doc = "Field `SEL4` writer - Selection of RC Oscillator Calibration bits for 4 MHz"]
pub type SEL4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CAL8` reader - RC Oscillator Calibration bits for 8 MHz"]
pub type CAL8_R = crate::FieldReader;
#[doc = "Field `CAL8` writer - RC Oscillator Calibration bits for 8 MHz"]
pub type CAL8_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `SEL8` reader - Selection of RC Oscillator Calibration bits for 8 MHz"]
pub type SEL8_R = crate::BitReader;
#[doc = "Field `SEL8` writer - Selection of RC Oscillator Calibration bits for 8 MHz"]
pub type SEL8_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CAL12` reader - RC Oscillator Calibration bits for 12 MHz"]
pub type CAL12_R = crate::FieldReader;
#[doc = "Field `CAL12` writer - RC Oscillator Calibration bits for 12 MHz"]
pub type CAL12_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `SEL12` reader - Selection of RC Oscillator Calibration bits for 12 MHz"]
pub type SEL12_R = crate::BitReader;
#[doc = "Field `SEL12` writer - Selection of RC Oscillator Calibration bits for 12 MHz"]
pub type SEL12_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:6 - RC Oscillator Calibration bits for 4 MHz"]
    #[inline(always)]
    pub fn cal4(&self) -> CAL4_R {
        CAL4_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Selection of RC Oscillator Calibration bits for 4 MHz"]
    #[inline(always)]
    pub fn sel4(&self) -> SEL4_R {
        SEL4_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:14 - RC Oscillator Calibration bits for 8 MHz"]
    #[inline(always)]
    pub fn cal8(&self) -> CAL8_R {
        CAL8_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - Selection of RC Oscillator Calibration bits for 8 MHz"]
    #[inline(always)]
    pub fn sel8(&self) -> SEL8_R {
        SEL8_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:22 - RC Oscillator Calibration bits for 12 MHz"]
    #[inline(always)]
    pub fn cal12(&self) -> CAL12_R {
        CAL12_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 23 - Selection of RC Oscillator Calibration bits for 12 MHz"]
    #[inline(always)]
    pub fn sel12(&self) -> SEL12_R {
        SEL12_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - RC Oscillator Calibration bits for 4 MHz"]
    #[inline(always)]
    #[must_use]
    pub fn cal4(&mut self) -> CAL4_W<PMC_OCR_SPEC, 0> {
        CAL4_W::new(self)
    }
    #[doc = "Bit 7 - Selection of RC Oscillator Calibration bits for 4 MHz"]
    #[inline(always)]
    #[must_use]
    pub fn sel4(&mut self) -> SEL4_W<PMC_OCR_SPEC, 7> {
        SEL4_W::new(self)
    }
    #[doc = "Bits 8:14 - RC Oscillator Calibration bits for 8 MHz"]
    #[inline(always)]
    #[must_use]
    pub fn cal8(&mut self) -> CAL8_W<PMC_OCR_SPEC, 8> {
        CAL8_W::new(self)
    }
    #[doc = "Bit 15 - Selection of RC Oscillator Calibration bits for 8 MHz"]
    #[inline(always)]
    #[must_use]
    pub fn sel8(&mut self) -> SEL8_W<PMC_OCR_SPEC, 15> {
        SEL8_W::new(self)
    }
    #[doc = "Bits 16:22 - RC Oscillator Calibration bits for 12 MHz"]
    #[inline(always)]
    #[must_use]
    pub fn cal12(&mut self) -> CAL12_W<PMC_OCR_SPEC, 16> {
        CAL12_W::new(self)
    }
    #[doc = "Bit 23 - Selection of RC Oscillator Calibration bits for 12 MHz"]
    #[inline(always)]
    #[must_use]
    pub fn sel12(&mut self) -> SEL12_W<PMC_OCR_SPEC, 23> {
        SEL12_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Oscillator Calibration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmc_ocr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmc_ocr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PMC_OCR_SPEC;
impl crate::RegisterSpec for PMC_OCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmc_ocr::R`](R) reader structure"]
impl crate::Readable for PMC_OCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pmc_ocr::W`](W) writer structure"]
impl crate::Writable for PMC_OCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PMC_OCR to value 0x0040_4040"]
impl crate::Resettable for PMC_OCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0040_4040;
}
