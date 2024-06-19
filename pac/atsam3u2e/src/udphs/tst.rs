#[doc = "Register `TST` reader"]
pub type R = crate::R<TstSpec>;
#[doc = "Register `TST` writer"]
pub type W = crate::W<TstSpec>;
#[doc = "Speed Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SpeedCfg {
    #[doc = "0: Normal Mode: The macro is in Full Speed mode, ready to make a High Speed identification, if the host supports it and then to automatically switch to High Speed mode"]
    Normal = 0,
    #[doc = "2: Force High Speed: Set this value to force the hardware to work in High Speed mode. Only for debug or test purpose."]
    HighSpeed = 2,
    #[doc = "3: Force Full Speed: Set this value to force the hardware to work only in Full Speed mode. In this configuration, the macro will not respond to a High Speed reset handshake."]
    FullSpeed = 3,
}
impl From<SpeedCfg> for u8 {
    #[inline(always)]
    fn from(variant: SpeedCfg) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SpeedCfg {
    type Ux = u8;
}
impl crate::IsEnum for SpeedCfg {}
#[doc = "Field `SPEED_CFG` reader - Speed Configuration"]
pub type SpeedCfgR = crate::FieldReader<SpeedCfg>;
impl SpeedCfgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SpeedCfg> {
        match self.bits {
            0 => Some(SpeedCfg::Normal),
            2 => Some(SpeedCfg::HighSpeed),
            3 => Some(SpeedCfg::FullSpeed),
            _ => None,
        }
    }
    #[doc = "Normal Mode: The macro is in Full Speed mode, ready to make a High Speed identification, if the host supports it and then to automatically switch to High Speed mode"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == SpeedCfg::Normal
    }
    #[doc = "Force High Speed: Set this value to force the hardware to work in High Speed mode. Only for debug or test purpose."]
    #[inline(always)]
    pub fn is_high_speed(&self) -> bool {
        *self == SpeedCfg::HighSpeed
    }
    #[doc = "Force Full Speed: Set this value to force the hardware to work only in Full Speed mode. In this configuration, the macro will not respond to a High Speed reset handshake."]
    #[inline(always)]
    pub fn is_full_speed(&self) -> bool {
        *self == SpeedCfg::FullSpeed
    }
}
#[doc = "Field `SPEED_CFG` writer - Speed Configuration"]
pub type SpeedCfgW<'a, REG> = crate::FieldWriter<'a, REG, 2, SpeedCfg>;
impl<'a, REG> SpeedCfgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Normal Mode: The macro is in Full Speed mode, ready to make a High Speed identification, if the host supports it and then to automatically switch to High Speed mode"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(SpeedCfg::Normal)
    }
    #[doc = "Force High Speed: Set this value to force the hardware to work in High Speed mode. Only for debug or test purpose."]
    #[inline(always)]
    pub fn high_speed(self) -> &'a mut crate::W<REG> {
        self.variant(SpeedCfg::HighSpeed)
    }
    #[doc = "Force Full Speed: Set this value to force the hardware to work only in Full Speed mode. In this configuration, the macro will not respond to a High Speed reset handshake."]
    #[inline(always)]
    pub fn full_speed(self) -> &'a mut crate::W<REG> {
        self.variant(SpeedCfg::FullSpeed)
    }
}
#[doc = "Field `TST_J` reader - Test J Mode"]
pub type TstJR = crate::BitReader;
#[doc = "Field `TST_J` writer - Test J Mode"]
pub type TstJW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TST_K` reader - Test K Mode"]
pub type TstKR = crate::BitReader;
#[doc = "Field `TST_K` writer - Test K Mode"]
pub type TstKW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TST_PKT` reader - Test Packet Mode"]
pub type TstPktR = crate::BitReader;
#[doc = "Field `TST_PKT` writer - Test Packet Mode"]
pub type TstPktW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPMODE2` reader - OpMode2"]
pub type Opmode2R = crate::BitReader;
#[doc = "Field `OPMODE2` writer - OpMode2"]
pub type Opmode2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Speed Configuration"]
    #[inline(always)]
    pub fn speed_cfg(&self) -> SpeedCfgR {
        SpeedCfgR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Test J Mode"]
    #[inline(always)]
    pub fn tst_j(&self) -> TstJR {
        TstJR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Test K Mode"]
    #[inline(always)]
    pub fn tst_k(&self) -> TstKR {
        TstKR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Test Packet Mode"]
    #[inline(always)]
    pub fn tst_pkt(&self) -> TstPktR {
        TstPktR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - OpMode2"]
    #[inline(always)]
    pub fn opmode2(&self) -> Opmode2R {
        Opmode2R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Speed Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn speed_cfg(&mut self) -> SpeedCfgW<TstSpec> {
        SpeedCfgW::new(self, 0)
    }
    #[doc = "Bit 2 - Test J Mode"]
    #[inline(always)]
    #[must_use]
    pub fn tst_j(&mut self) -> TstJW<TstSpec> {
        TstJW::new(self, 2)
    }
    #[doc = "Bit 3 - Test K Mode"]
    #[inline(always)]
    #[must_use]
    pub fn tst_k(&mut self) -> TstKW<TstSpec> {
        TstKW::new(self, 3)
    }
    #[doc = "Bit 4 - Test Packet Mode"]
    #[inline(always)]
    #[must_use]
    pub fn tst_pkt(&mut self) -> TstPktW<TstSpec> {
        TstPktW::new(self, 4)
    }
    #[doc = "Bit 5 - OpMode2"]
    #[inline(always)]
    #[must_use]
    pub fn opmode2(&mut self) -> Opmode2W<TstSpec> {
        Opmode2W::new(self, 5)
    }
}
#[doc = "UDPHS Test Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tst::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tst::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TstSpec;
impl crate::RegisterSpec for TstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tst::R`](R) reader structure"]
impl crate::Readable for TstSpec {}
#[doc = "`write(|w| ..)` method takes [`tst::W`](W) writer structure"]
impl crate::Writable for TstSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TST to value 0"]
impl crate::Resettable for TstSpec {
    const RESET_VALUE: u32 = 0;
}
