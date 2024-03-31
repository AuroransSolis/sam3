#[doc = "Register `CSTOR` reader"]
pub type R = crate::R<CstorSpec>;
#[doc = "Register `CSTOR` writer"]
pub type W = crate::W<CstorSpec>;
#[doc = "Field `CSTOCYC` reader - Completion Signal Timeout Cycle Number"]
pub type CstocycR = crate::FieldReader;
#[doc = "Field `CSTOCYC` writer - Completion Signal Timeout Cycle Number"]
pub type CstocycW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Completion Signal Timeout Multiplier\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cstomul {
    #[doc = "0: CSTOCYC x 1"]
    _1 = 0,
    #[doc = "1: CSTOCYC x 16"]
    _16 = 1,
    #[doc = "2: CSTOCYC x 128"]
    _128 = 2,
    #[doc = "3: CSTOCYC x 256"]
    _256 = 3,
    #[doc = "4: CSTOCYC x 1024"]
    _1024 = 4,
    #[doc = "5: CSTOCYC x 4096"]
    _4096 = 5,
    #[doc = "6: CSTOCYC x 65536"]
    _65536 = 6,
    #[doc = "7: CSTOCYC x 1048576"]
    _1048576 = 7,
}
impl From<Cstomul> for u8 {
    #[inline(always)]
    fn from(variant: Cstomul) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cstomul {
    type Ux = u8;
}
impl crate::IsEnum for Cstomul {}
#[doc = "Field `CSTOMUL` reader - Completion Signal Timeout Multiplier"]
pub type CstomulR = crate::FieldReader<Cstomul>;
impl CstomulR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cstomul {
        match self.bits {
            0 => Cstomul::_1,
            1 => Cstomul::_16,
            2 => Cstomul::_128,
            3 => Cstomul::_256,
            4 => Cstomul::_1024,
            5 => Cstomul::_4096,
            6 => Cstomul::_65536,
            7 => Cstomul::_1048576,
            _ => unreachable!(),
        }
    }
    #[doc = "CSTOCYC x 1"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cstomul::_1
    }
    #[doc = "CSTOCYC x 16"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        *self == Cstomul::_16
    }
    #[doc = "CSTOCYC x 128"]
    #[inline(always)]
    pub fn is_128(&self) -> bool {
        *self == Cstomul::_128
    }
    #[doc = "CSTOCYC x 256"]
    #[inline(always)]
    pub fn is_256(&self) -> bool {
        *self == Cstomul::_256
    }
    #[doc = "CSTOCYC x 1024"]
    #[inline(always)]
    pub fn is_1024(&self) -> bool {
        *self == Cstomul::_1024
    }
    #[doc = "CSTOCYC x 4096"]
    #[inline(always)]
    pub fn is_4096(&self) -> bool {
        *self == Cstomul::_4096
    }
    #[doc = "CSTOCYC x 65536"]
    #[inline(always)]
    pub fn is_65536(&self) -> bool {
        *self == Cstomul::_65536
    }
    #[doc = "CSTOCYC x 1048576"]
    #[inline(always)]
    pub fn is_1048576(&self) -> bool {
        *self == Cstomul::_1048576
    }
}
#[doc = "Field `CSTOMUL` writer - Completion Signal Timeout Multiplier"]
pub type CstomulW<'a, REG> = crate::FieldWriter<'a, REG, 3, Cstomul, crate::Safe>;
impl<'a, REG> CstomulW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CSTOCYC x 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cstomul::_1)
    }
    #[doc = "CSTOCYC x 16"]
    #[inline(always)]
    pub fn _16(self) -> &'a mut crate::W<REG> {
        self.variant(Cstomul::_16)
    }
    #[doc = "CSTOCYC x 128"]
    #[inline(always)]
    pub fn _128(self) -> &'a mut crate::W<REG> {
        self.variant(Cstomul::_128)
    }
    #[doc = "CSTOCYC x 256"]
    #[inline(always)]
    pub fn _256(self) -> &'a mut crate::W<REG> {
        self.variant(Cstomul::_256)
    }
    #[doc = "CSTOCYC x 1024"]
    #[inline(always)]
    pub fn _1024(self) -> &'a mut crate::W<REG> {
        self.variant(Cstomul::_1024)
    }
    #[doc = "CSTOCYC x 4096"]
    #[inline(always)]
    pub fn _4096(self) -> &'a mut crate::W<REG> {
        self.variant(Cstomul::_4096)
    }
    #[doc = "CSTOCYC x 65536"]
    #[inline(always)]
    pub fn _65536(self) -> &'a mut crate::W<REG> {
        self.variant(Cstomul::_65536)
    }
    #[doc = "CSTOCYC x 1048576"]
    #[inline(always)]
    pub fn _1048576(self) -> &'a mut crate::W<REG> {
        self.variant(Cstomul::_1048576)
    }
}
impl R {
    #[doc = "Bits 0:3 - Completion Signal Timeout Cycle Number"]
    #[inline(always)]
    pub fn cstocyc(&self) -> CstocycR {
        CstocycR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Completion Signal Timeout Multiplier"]
    #[inline(always)]
    pub fn cstomul(&self) -> CstomulR {
        CstomulR::new(((self.bits >> 4) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Completion Signal Timeout Cycle Number"]
    #[inline(always)]
    #[must_use]
    pub fn cstocyc(&mut self) -> CstocycW<CstorSpec> {
        CstocycW::new(self, 0)
    }
    #[doc = "Bits 4:6 - Completion Signal Timeout Multiplier"]
    #[inline(always)]
    #[must_use]
    pub fn cstomul(&mut self) -> CstomulW<CstorSpec> {
        CstomulW::new(self, 4)
    }
}
#[doc = "Completion Signal Timeout Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cstor::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cstor::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CstorSpec;
impl crate::RegisterSpec for CstorSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cstor::R`](R) reader structure"]
impl crate::Readable for CstorSpec {}
#[doc = "`write(|w| ..)` method takes [`cstor::W`](W) writer structure"]
impl crate::Writable for CstorSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSTOR to value 0"]
impl crate::Resettable for CstorSpec {
    const RESET_VALUE: u32 = 0;
}
