#[doc = "Register `MMR2` reader"]
pub type R = crate::R<Mmr2Spec>;
#[doc = "Register `MMR2` writer"]
pub type W = crate::W<Mmr2Spec>;
#[doc = "Field `MTIMEMARK` reader - Mailbox Timemark"]
pub type MtimemarkR = crate::FieldReader<u16>;
#[doc = "Field `MTIMEMARK` writer - Mailbox Timemark"]
pub type MtimemarkW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PRIOR` reader - Mailbox Priority"]
pub type PriorR = crate::FieldReader;
#[doc = "Field `PRIOR` writer - Mailbox Priority"]
pub type PriorW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Mailbox Object Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mot {
    #[doc = "0: Mailbox is disabled. This prevents receiving or transmitting any messages with this mailbox."]
    MbDisabled = 0,
    #[doc = "1: Reception Mailbox. Mailbox is configured for reception. If a message is received while the mailbox data register is full, it is discarded."]
    MbRx = 1,
    #[doc = "2: Reception mailbox with overwrite. Mailbox is configured for reception. If a message is received while the mailbox is full, it overwrites the previous message."]
    MbRxOverwrite = 2,
    #[doc = "3: Transmit mailbox. Mailbox is configured for transmission."]
    MbTx = 3,
    #[doc = "4: Consumer Mailbox. Mailbox is configured in reception but behaves as a Transmit Mailbox, i.e., it sends a remote frame and waits for an answer."]
    MbConsumer = 4,
    #[doc = "5: Producer Mailbox. Mailbox is configured in transmission but also behaves like a reception mailbox, i.e., it waits to receive a Remote Frame before sending its contents."]
    MbProducer = 5,
}
impl From<Mot> for u8 {
    #[inline(always)]
    fn from(variant: Mot) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mot {
    type Ux = u8;
}
#[doc = "Field `MOT` reader - Mailbox Object Type"]
pub type MotR = crate::FieldReader<Mot>;
impl MotR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Mot> {
        match self.bits {
            0 => Some(Mot::MbDisabled),
            1 => Some(Mot::MbRx),
            2 => Some(Mot::MbRxOverwrite),
            3 => Some(Mot::MbTx),
            4 => Some(Mot::MbConsumer),
            5 => Some(Mot::MbProducer),
            _ => None,
        }
    }
    #[doc = "Mailbox is disabled. This prevents receiving or transmitting any messages with this mailbox."]
    #[inline(always)]
    pub fn is_mb_disabled(&self) -> bool {
        *self == Mot::MbDisabled
    }
    #[doc = "Reception Mailbox. Mailbox is configured for reception. If a message is received while the mailbox data register is full, it is discarded."]
    #[inline(always)]
    pub fn is_mb_rx(&self) -> bool {
        *self == Mot::MbRx
    }
    #[doc = "Reception mailbox with overwrite. Mailbox is configured for reception. If a message is received while the mailbox is full, it overwrites the previous message."]
    #[inline(always)]
    pub fn is_mb_rx_overwrite(&self) -> bool {
        *self == Mot::MbRxOverwrite
    }
    #[doc = "Transmit mailbox. Mailbox is configured for transmission."]
    #[inline(always)]
    pub fn is_mb_tx(&self) -> bool {
        *self == Mot::MbTx
    }
    #[doc = "Consumer Mailbox. Mailbox is configured in reception but behaves as a Transmit Mailbox, i.e., it sends a remote frame and waits for an answer."]
    #[inline(always)]
    pub fn is_mb_consumer(&self) -> bool {
        *self == Mot::MbConsumer
    }
    #[doc = "Producer Mailbox. Mailbox is configured in transmission but also behaves like a reception mailbox, i.e., it waits to receive a Remote Frame before sending its contents."]
    #[inline(always)]
    pub fn is_mb_producer(&self) -> bool {
        *self == Mot::MbProducer
    }
}
#[doc = "Field `MOT` writer - Mailbox Object Type"]
pub type MotW<'a, REG> = crate::FieldWriter<'a, REG, 3, Mot>;
impl<'a, REG> MotW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Mailbox is disabled. This prevents receiving or transmitting any messages with this mailbox."]
    #[inline(always)]
    pub fn mb_disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Mot::MbDisabled)
    }
    #[doc = "Reception Mailbox. Mailbox is configured for reception. If a message is received while the mailbox data register is full, it is discarded."]
    #[inline(always)]
    pub fn mb_rx(self) -> &'a mut crate::W<REG> {
        self.variant(Mot::MbRx)
    }
    #[doc = "Reception mailbox with overwrite. Mailbox is configured for reception. If a message is received while the mailbox is full, it overwrites the previous message."]
    #[inline(always)]
    pub fn mb_rx_overwrite(self) -> &'a mut crate::W<REG> {
        self.variant(Mot::MbRxOverwrite)
    }
    #[doc = "Transmit mailbox. Mailbox is configured for transmission."]
    #[inline(always)]
    pub fn mb_tx(self) -> &'a mut crate::W<REG> {
        self.variant(Mot::MbTx)
    }
    #[doc = "Consumer Mailbox. Mailbox is configured in reception but behaves as a Transmit Mailbox, i.e., it sends a remote frame and waits for an answer."]
    #[inline(always)]
    pub fn mb_consumer(self) -> &'a mut crate::W<REG> {
        self.variant(Mot::MbConsumer)
    }
    #[doc = "Producer Mailbox. Mailbox is configured in transmission but also behaves like a reception mailbox, i.e., it waits to receive a Remote Frame before sending its contents."]
    #[inline(always)]
    pub fn mb_producer(self) -> &'a mut crate::W<REG> {
        self.variant(Mot::MbProducer)
    }
}
impl R {
    #[doc = "Bits 0:15 - Mailbox Timemark"]
    #[inline(always)]
    pub fn mtimemark(&self) -> MtimemarkR {
        MtimemarkR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - Mailbox Priority"]
    #[inline(always)]
    pub fn prior(&self) -> PriorR {
        PriorR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:26 - Mailbox Object Type"]
    #[inline(always)]
    pub fn mot(&self) -> MotR {
        MotR::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - Mailbox Timemark"]
    #[inline(always)]
    #[must_use]
    pub fn mtimemark(&mut self) -> MtimemarkW<Mmr2Spec> {
        MtimemarkW::new(self, 0)
    }
    #[doc = "Bits 16:19 - Mailbox Priority"]
    #[inline(always)]
    #[must_use]
    pub fn prior(&mut self) -> PriorW<Mmr2Spec> {
        PriorW::new(self, 16)
    }
    #[doc = "Bits 24:26 - Mailbox Object Type"]
    #[inline(always)]
    #[must_use]
    pub fn mot(&mut self) -> MotW<Mmr2Spec> {
        MotW::new(self, 24)
    }
}
#[doc = "Mailbox Mode Register (MB = 2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mmr2Spec;
impl crate::RegisterSpec for Mmr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmr2::R`](R) reader structure"]
impl crate::Readable for Mmr2Spec {}
#[doc = "`write(|w| ..)` method takes [`mmr2::W`](W) writer structure"]
impl crate::Writable for Mmr2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MMR2 to value 0"]
impl crate::Resettable for Mmr2Spec {
    const RESET_VALUE: u32 = 0;
}
