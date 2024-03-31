#[doc = "Register `MR` reader"]
pub type R = crate::R<MrSpec>;
#[doc = "Register `MR` writer"]
pub type W = crate::W<MrSpec>;
#[doc = "Field `URSTEN` reader - User Reset Enable"]
pub type UrstenR = crate::BitReader;
#[doc = "Field `URSTEN` writer - User Reset Enable"]
pub type UrstenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `URSTIEN` reader - User Reset Interrupt Enable"]
pub type UrstienR = crate::BitReader;
#[doc = "Field `URSTIEN` writer - User Reset Interrupt Enable"]
pub type UrstienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERSTL` reader - External Reset Length"]
pub type ErstlR = crate::FieldReader;
#[doc = "Field `ERSTL` writer - External Reset Length"]
pub type ErstlW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Write Access Password\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Key {
    #[doc = "165: Writing any other value in this field aborts the write operation.Always reads as 0."]
    Passwd = 165,
}
impl From<Key> for u8 {
    #[inline(always)]
    fn from(variant: Key) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Key {
    type Ux = u8;
}
impl crate::IsEnum for Key {}
#[doc = "Field `KEY` reader - Write Access Password"]
pub type KeyR = crate::FieldReader<Key>;
impl KeyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Key> {
        match self.bits {
            165 => Some(Key::Passwd),
            _ => None,
        }
    }
    #[doc = "Writing any other value in this field aborts the write operation.Always reads as 0."]
    #[inline(always)]
    pub fn is_passwd(&self) -> bool {
        *self == Key::Passwd
    }
}
#[doc = "Field `KEY` writer - Write Access Password"]
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 8, Key>;
impl<'a, REG> KeyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Writing any other value in this field aborts the write operation.Always reads as 0."]
    #[inline(always)]
    pub fn passwd(self) -> &'a mut crate::W<REG> {
        self.variant(Key::Passwd)
    }
}
impl R {
    #[doc = "Bit 0 - User Reset Enable"]
    #[inline(always)]
    pub fn ursten(&self) -> UrstenR {
        UrstenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - User Reset Interrupt Enable"]
    #[inline(always)]
    pub fn urstien(&self) -> UrstienR {
        UrstienR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:11 - External Reset Length"]
    #[inline(always)]
    pub fn erstl(&self) -> ErstlR {
        ErstlR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 24:31 - Write Access Password"]
    #[inline(always)]
    pub fn key(&self) -> KeyR {
        KeyR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - User Reset Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ursten(&mut self) -> UrstenW<MrSpec> {
        UrstenW::new(self, 0)
    }
    #[doc = "Bit 4 - User Reset Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn urstien(&mut self) -> UrstienW<MrSpec> {
        UrstienW::new(self, 4)
    }
    #[doc = "Bits 8:11 - External Reset Length"]
    #[inline(always)]
    #[must_use]
    pub fn erstl(&mut self) -> ErstlW<MrSpec> {
        ErstlW::new(self, 8)
    }
    #[doc = "Bits 24:31 - Write Access Password"]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KeyW<MrSpec> {
        KeyW::new(self, 24)
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
#[doc = "`reset()` method sets MR to value 0x01"]
impl crate::Resettable for MrSpec {
    const RESET_VALUE: u32 = 0x01;
}
