#[doc = "Register `MR` reader"]
pub type R = crate::R<MrSpec>;
#[doc = "Register `MR` writer"]
pub type W = crate::W<MrSpec>;
#[doc = "Field `RTPRES` reader - Real-time Timer Prescaler Value"]
pub type RtpresR = crate::FieldReader<u16>;
#[doc = "Field `RTPRES` writer - Real-time Timer Prescaler Value"]
pub type RtpresW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `ALMIEN` reader - Alarm Interrupt Enable"]
pub type AlmienR = crate::BitReader;
#[doc = "Field `ALMIEN` writer - Alarm Interrupt Enable"]
pub type AlmienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTTINCIEN` reader - Real-time Timer Increment Interrupt Enable"]
pub type RttincienR = crate::BitReader;
#[doc = "Field `RTTINCIEN` writer - Real-time Timer Increment Interrupt Enable"]
pub type RttincienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTTRST` reader - Real-time Timer Restart"]
pub type RttrstR = crate::BitReader;
#[doc = "Field `RTTRST` writer - Real-time Timer Restart"]
pub type RttrstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - Real-time Timer Prescaler Value"]
    #[inline(always)]
    pub fn rtpres(&self) -> RtpresR {
        RtpresR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Alarm Interrupt Enable"]
    #[inline(always)]
    pub fn almien(&self) -> AlmienR {
        AlmienR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Real-time Timer Increment Interrupt Enable"]
    #[inline(always)]
    pub fn rttincien(&self) -> RttincienR {
        RttincienR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Real-time Timer Restart"]
    #[inline(always)]
    pub fn rttrst(&self) -> RttrstR {
        RttrstR::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Real-time Timer Prescaler Value"]
    #[inline(always)]
    #[must_use]
    pub fn rtpres(&mut self) -> RtpresW<MrSpec> {
        RtpresW::new(self, 0)
    }
    #[doc = "Bit 16 - Alarm Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn almien(&mut self) -> AlmienW<MrSpec> {
        AlmienW::new(self, 16)
    }
    #[doc = "Bit 17 - Real-time Timer Increment Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rttincien(&mut self) -> RttincienW<MrSpec> {
        RttincienW::new(self, 17)
    }
    #[doc = "Bit 18 - Real-time Timer Restart"]
    #[inline(always)]
    #[must_use]
    pub fn rttrst(&mut self) -> RttrstW<MrSpec> {
        RttrstW::new(self, 18)
    }
}
#[doc = "Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MrSpec;
impl crate::RegisterSpec for MrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mr::R`](R) reader structure"]
impl crate::Readable for MrSpec {}
#[doc = "`write(|w| ..)` method takes [`mr::W`](W) writer structure"]
impl crate::Writable for MrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MR to value 0x8000"]
impl crate::Resettable for MrSpec {
    const RESET_VALUE: u32 = 0x8000;
}
